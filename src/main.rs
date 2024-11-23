use anyhow::Result;
use clap::{Parser, Subcommand};
use yolk::Yolk;

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
    #[clap(name = "tmpl")]
    MakeTemplate {
        thing: String,
        #[arg(required = true)]
        paths: Vec<std::path::PathBuf>,
    },
}

pub(crate) fn main() -> Result<()> {
    let args = Args::parse();

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
        Command::Sync => yolk.sync()?,
        Command::Eval { expr } => {
            println!("{}", yolk.eval_rhai(yolk::EvalMode::Local, expr)?);
        }
        Command::Git { command } => {
            yolk.prepare_canonical()?;
            std::process::Command::new("git")
                .args(command)
                .current_dir(yolk.paths().root_path())
                .status()?;
        }
        Command::MakeTemplate { thing, paths } => {
            yolk.add_to_templated_files(thing, paths)?;
        }
    }
    Ok(())
}
