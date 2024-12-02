use std::{io::Read as _, str::FromStr};

use clap::{Parser, Subcommand};
use miette::{IntoDiagnostic, Result};
use script::eval_ctx;
use templating::document::ParseError;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};
use yolk::{EvalMode, Yolk};

pub mod script;
mod templating;
mod util;
mod yolk;
mod yolk_paths;

#[derive(clap::Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,

    /// Provide a custom yolk directory
    #[arg(short = 'd', long, env = "YOLK_DIR", global = true)]
    yolk_dir: Option<std::path::PathBuf>,

    /// Enable debug logging
    #[arg(long, short = 'v', global = true)]
    debug: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Initialize the yolk directory
    Init,
    /// Use an egg
    Use { name: String },
    /// Evaluate an expression like it would be done in a template
    Eval {
        #[arg(long)]
        canonical: bool,
        expr: String,
    },
    /// Add a file or directory to an egg in yolk
    Add {
        name: String,
        path: std::path::PathBuf,
    },
    /// Re-evaluate all local templates to ensure that they are in a consistent state
    Sync {
        #[arg(long)]
        canonical: bool,
    },
    /// Run a git-command within the yolk directory while in canonical state.
    Git {
        #[clap(allow_hyphen_values = true)]
        command: Vec<String>,
    },
    /// Make the given file template capable, by adding it to the yolk_templates file
    #[clap(name = "mktmpl")]
    MakeTemplate {
        egg: String,
        #[arg(required = true)]
        paths: Vec<std::path::PathBuf>,
    },
    /// Evaluate a given templated file, or read a templated string from stdin
    #[clap(name = "eval-template")]
    EvalTemplate {
        #[arg(long)]
        canonical: bool,
        path: Option<std::path::PathBuf>,
    },
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
    let yolk_paths = if let Some(yolk_dir) = args.yolk_dir {
        yolk_paths::YolkPaths::from_env_with_root(yolk_dir)
    } else {
        yolk_paths::YolkPaths::from_env()
    };

    let yolk = Yolk::new(yolk_paths);
    match &args.command {
        Command::Init => yolk.init_yolk()?,
        Command::Use { name } => yolk.use_egg(name)?,
        Command::Add { name, path } => yolk.add_egg(name, path)?,
        Command::Sync { canonical } => {
            let mode = if *canonical {
                EvalMode::Canonical
            } else {
                EvalMode::Local
            };

            yolk.sync_to_mode(mode)?
        }
        Command::Eval { expr, canonical } => {
            let mode = if *canonical {
                EvalMode::Canonical
            } else {
                EvalMode::Local
            };
            println!("{}", yolk.eval_lua(mode, expr)?);
        }
        Command::Git { command } => {
            yolk.with_canonical_state(|| {
                std::process::Command::new("git")
                    .args(command)
                    .current_dir(yolk.paths().root_path())
                    .status()
                    .into_diagnostic()?;
                Ok(())
            })?;
        }
        Command::MakeTemplate { egg, paths } => {
            yolk.add_to_templated_files(egg, paths)?;
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
            let mode = if *canonical {
                EvalMode::Canonical
            } else {
                EvalMode::Local
            };
            let mut eval_ctx = yolk.prepare_eval_ctx_for_templates(mode)?;
            let result = yolk.eval_template(&mut eval_ctx, &text)?;
            println!("{}", result);
        }
    }
    Ok(())
}
