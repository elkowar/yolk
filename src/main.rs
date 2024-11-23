use std::{io::Read as _, str::FromStr};

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};
use yolk::{EvalMode, Yolk};

mod eval_ctx;
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
    #[arg(short = 'd', long, env = "YOLK_DIR")]
    yolk_dir: Option<std::path::PathBuf>,

    /// Enable debug logging
    #[arg(long)]
    debug: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Initialize the yolk directory
    Init,
    /// Use a thing
    Use { name: String },
    /// Evaluate an expression in the local context
    Eval { expr: String },
    /// Add a file or directory to a thing in yolk
    Add {
        name: String,
        path: std::path::PathBuf,
    },
    /// Re-evaluate all local templates to ensure that they are in a consistent state
    Sync,
    /// Run a git-command within the yolk directory,
    /// while ensuring that the canonical directory is up-to-date
    Git {
        #[clap(allow_hyphen_values = true)]
        command: Vec<String>,
    },
    /// Make the given file template capable, by adding it to the yolk_templates file
    #[clap(name = "mktmpl")]
    MakeTemplate {
        thing: String,
        #[arg(required = true)]
        paths: Vec<std::path::PathBuf>,
    },
    /// Evaluate a given templated file, or read a templated string from stdin
    #[clap(name = "eval-template")]
    EvalTemplate { path: Option<std::path::PathBuf> },
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

    let yolk_paths = if let Some(yolk_dir) = args.yolk_dir {
        yolk_paths::YolkPaths::from_env_with_root(yolk_dir)
    } else {
        yolk_paths::YolkPaths::from_env()
    };

    let yolk = Yolk::new(yolk_paths);
    match &args.command {
        Command::Init => yolk.init_yolk()?,
        Command::Use { name } => yolk.use_thing(name)?,
        Command::Add { name, path } => yolk.add_thing(name, path)?,
        Command::Sync => yolk.sync_to_mode(EvalMode::Local)?,
        Command::Eval { expr } => {
            println!("{}", yolk.eval_rhai(yolk::EvalMode::Local, expr)?);
        }
        Command::Git { command } => {
            yolk.with_canonical_state(|| {
                std::process::Command::new("git")
                    .args(command)
                    .current_dir(yolk.paths().root_path())
                    .status()?;
                Ok(())
            })?;
        }
        Command::MakeTemplate { thing, paths } => {
            yolk.add_to_templated_files(thing, paths)?;
        }
        Command::EvalTemplate { path } => {
            let text = match path {
                Some(path) => std::fs::read_to_string(path)?,
                None => {
                    let mut buffer = String::new();
                    std::io::stdin().read_to_string(&mut buffer)?;
                    buffer
                }
            };
            let engine = eval_ctx::make_engine();
            let mut eval_ctx = yolk.prepare_eval_ctx(EvalMode::Local, &engine)?;
            let result = yolk.eval_template(&mut eval_ctx, &text)?;
            println!("{}", result);
        }
    }
    Ok(())
}
