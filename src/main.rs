use std::{io::Read as _, path::PathBuf, str::FromStr};

use clap::{Parser, Subcommand};
use miette::{IntoDiagnostic, Result};
use script::eval_ctx;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};
use yolk::{EvalMode, Yolk};

pub mod script;
mod templating;
#[cfg(test)]
pub mod test;
mod util;
mod yolk;
mod yolk_paths;

#[derive(clap::Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,

    /// Provide a custom yolk directory
    #[arg(long, env = "YOLK_DIR", global = true)]
    yolk_dir: Option<PathBuf>,

    /// Provide a custom home directory that everything will be resolved relative to
    #[arg(long, env = "YOLK_HOME_DIR", global = true)]
    home_dir: Option<PathBuf>,

    /// Enable debug logging
    #[arg(long, short = 'v', global = true)]
    debug: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Initialize the yolk directory
    Init,
    /// Deploy an egg
    Deploy { name: String },
    /// Evaluate an expression like it would be done in a template
    Eval {
        /// Evaluate in canonical context instead.
        #[arg(long)]
        canonical: bool,
        /// The expression to evaluate
        expr: String,
    },
    /// Add a file or directory to an egg in yolk
    Add {
        /// The name of the egg
        name: String,
        /// The file to add into your egg
        path: PathBuf,
    },
    /// Re-evaluate all local templates to ensure that they are in a consistent state
    Sync {
        /// Sync to canonical state. This should only be necessary for debugging purposes.
        #[arg(long)]
        canonical: bool,
    },
    /// Run a git-command within the yolk directory while in canonical state.
    #[clap(alias = "g")]
    Git {
        #[clap(allow_hyphen_values = true)]
        command: Vec<String>,
    },
    /// Make the given file template capable, by adding it to the yolk_templates file
    #[clap(name = "mktmpl")]
    MakeTemplate {
        /// The files you want to turn into templates
        #[arg(required = true)]
        paths: Vec<PathBuf>,
    },
    /// Evaluate a given templated file, or read a templated string from stdin
    #[clap(name = "eval-template")]
    EvalTemplate {
        #[arg(long)]
        canonical: bool,
        /// The path to the file you want to evaluate
        /// If not provided, the program will read from stdin
        path: Option<PathBuf>,
    },

    /// List all the eggs in your yolk directory
    List,
}

pub(crate) fn main() -> Result<()> {
    let args = Args::parse();

    let env_filter = if args.debug {
        tracing_subscriber::EnvFilter::from_str("debug").unwrap()
    } else {
        tracing_subscriber::EnvFilter::from_default_env()
    };
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(env_filter)
        .init();

    if let Err(err) = run_command(args) {
        eprintln!("{:?}", err);
    }
    Ok(())
}

fn run_command(args: Args) -> Result<()> {
    let mut yolk_paths = yolk_paths::YolkPaths::from_env();
    if let Some(d) = args.yolk_dir {
        yolk_paths.set_yolk_dir(d);
    }
    if let Some(d) = args.home_dir {
        yolk_paths.set_home_dir(d);
    }

    let yolk = Yolk::new(yolk_paths);
    match &args.command {
        Command::Init => yolk.init_yolk()?,
        Command::Deploy { name: egg } => yolk.deploy_egg(egg)?,
        Command::Add { name: egg, path } => yolk.add_to_egg(egg, path)?,
        Command::List => {
            for egg in yolk.list_eggs()? {
                let egg = egg?;
                println!(
                    "{} [{}]",
                    egg.name(),
                    if egg.is_deployed()? {
                        "active"
                    } else {
                        "inactive"
                    }
                );
            }
        }
        Command::Sync { canonical } => {
            let mode = match *canonical {
                true => EvalMode::Canonical,
                false => EvalMode::Local,
            };
            yolk.sync_to_mode(mode)?
        }
        Command::Eval { expr, canonical } => {
            let mode = match *canonical {
                true => EvalMode::Canonical,
                false => EvalMode::Local,
            };
            println!("{}", yolk.eval_template_lua(mode, expr)?);
        }
        Command::Git { command } => {
            yolk.with_canonical_state(|| {
                std::process::Command::new("git")
                    .args(command)
                    .args(&[
                        "--git-dir",
                        &yolk.paths().yolk_internal_path().to_string_lossy(),
                        "--work-tree",
                        &yolk.paths().root_path().to_string_lossy(),
                    ])
                    .current_dir(yolk.paths().root_path())
                    .status()
                    .into_diagnostic()?;
                Ok(())
            })?;
        }
        Command::MakeTemplate { paths } => {
            yolk.add_to_templated_files(paths)?;
        }
        Command::EvalTemplate { path, canonical } => {
            let text = match path {
                Some(path) => std::fs::read_to_string(path).into_diagnostic()?,
                None => {
                    let mut buffer = String::new();
                    std::io::stdin()
                        .read_to_string(&mut buffer)
                        .into_diagnostic()?;
                    buffer
                }
            };
            let mode = match *canonical {
                true => EvalMode::Canonical,
                false => EvalMode::Local,
            };
            let mut eval_ctx = yolk.prepare_eval_ctx_for_templates(mode)?;
            let result = yolk.eval_template(&mut eval_ctx, "inline", &text)?;
            println!("{}", result);
        }
    }
    Ok(())
}
