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
}

#[derive(Debug, Subcommand)]
enum Command {
    Init,
    Use {
        name: String,
    },
    Eval {
        expr: String,
    },
    Add {
        name: String,
        path: std::path::PathBuf,
    },
    Sync,
}

pub(crate) fn main() -> Result<()> {
    let args = Args::parse();
    let yolk = Yolk::new(yolk_paths::YolkPaths::testing());
    match &args.command {
        Command::Init => yolk.init_yolk()?,
        Command::Use { name } => yolk.use_thing(name)?,
        Command::Add { name, path } => yolk.add_thing(name, path)?,
        Command::Sync => yolk.sync()?,
        Command::Eval { expr } => {
            println!("{}", yolk.eval_rhai(yolk::EvalMode::Local, expr)?);
        }
    }
    Ok(())
}
