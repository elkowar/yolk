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
    Git {
        #[clap(allow_hyphen_values = true)]
        command: Vec<String>,
    },
    /// Make the given file template capable, by adding it to the yolk_templates file
    #[clap(name = "tmpl")]
    MakeTemplate {
        thing: String,
        paths: Vec<std::path::PathBuf>,
    },
    /// List all things in your yolk setup
    List,
}

pub(crate) fn main() -> Result<()> {
    let args = Args::parse();
    let yolk = Yolk::new(yolk_paths::YolkPaths::from_env());
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
        Command::List => {
            println!(
                "{}",
                yolk.list_things()?
                    .into_iter()
                    .map(|(path, active)| format!(
                        "{} [{}]",
                        path.display(),
                        if active { "active" } else { "inactive" }
                    ))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        }
    }
    Ok(())
}
