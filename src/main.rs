use std::{collections::HashSet, io::Read as _, path::PathBuf, str::FromStr};

use clap::{Parser, Subcommand};
use miette::{IntoDiagnostic, Result};
use notify_debouncer_full::{new_debouncer, DebounceEventResult};
use owo_colors::OwoColorize as _;
use script::eval_ctx;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};
use yolk::{EvalMode, Yolk};

pub mod eggs_config;
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
    /// Show the current state of your yolk eggs
    Status,
    /// Make sure you don't accidentally commit your local egg states
    ///
    /// This renames `.git` to `.yolk_git` to ensure that git interaction happens through the yolk CLI
    Safeguard,
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
    #[clap(alias = "s")]
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
    #[clap(alias = "mktmpl")]
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
    /// Open your `yolk.lua` or the given egg in your `$EDITOR` of choice
    Edit { egg: Option<String> },
    Watch {
        #[arg(long)]
        canonical: bool,
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
        Command::Safeguard => yolk.paths().safeguard_git_dir()?,
        Command::Status => {
            yolk.paths().check()?;
            if yolk.paths().active_yolk_git_dir()? == yolk.paths().yolk_default_git_path() {
                println!("Yolk git is not Safeguarded. It is recommended to run `yolk safeguard`.");
            }
            yolk.with_canonical_state(|| {
                yolk.paths()
                    .start_git_command_builder()?
                    .args(["status", "--short"])
                    .status()
                    .into_diagnostic()
            })?;
        }
        Command::Deploy { name: egg } => yolk.deploy_egg(egg)?,
        Command::Add { name: egg, path } => yolk.add_to_egg(egg, path)?,
        Command::List => {
            let mut eggs = yolk.list_eggs()?.collect::<Result<Vec<_>>>()?;
            eggs.sort_by_key(|egg| egg.name().to_string());
            for egg in eggs {
                let deployed = egg.is_deployed()?;
                println!(
                    "{}",
                    format!("{} {}", if deployed { "✓" } else { "✗" }, egg.name(),)
                        .if_supports_color(owo_colors::Stream::Stdout, |text| {
                            text.color(if deployed {
                                owo_colors::AnsiColors::Green
                            } else {
                                owo_colors::AnsiColors::Default
                            })
                        })
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
                yolk.paths()
                    .start_git_command_builder()?
                    .args(command)
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
            let result = yolk.eval_template(&mut eval_ctx, "unnamed", &text)?;
            println!("{}", result);
        }
        Command::Edit { egg } => {
            let path = match egg {
                Some(egg_name) => {
                    let egg = yolk.paths().get_egg(egg_name)?;
                    egg.find_first_targetting_symlink()?
                        .unwrap_or_else(|| yolk.paths().egg_path(egg_name))
                }
                None => yolk.paths().yolk_lua_path(),
            };

            if let Some(parent) = path.parent() {
                let _ = std::env::set_current_dir(parent);
            }
            edit::edit_file(path).into_diagnostic()?;
        }
        Command::Watch { canonical } => {
            let mode = match *canonical {
                true => EvalMode::Canonical,
                false => EvalMode::Local,
            };

            let mut dirs_to_watch = HashSet::new();
            let mut files_to_watch = HashSet::new();
            let script_path = yolk.paths().yolk_lua_path();
            files_to_watch.insert(script_path.clone());

            for egg in yolk.paths().list_eggs()? {
                let egg = egg?;
                for path in egg.template_paths()? {
                    if let Some(parent) = path.parent() {
                        dirs_to_watch.insert(parent.to_path_buf());
                    }
                    files_to_watch.insert(path.to_path_buf());
                }
            }

            let mut debouncer = new_debouncer(
                std::time::Duration::from_millis(800),
                None,
                move |res: DebounceEventResult| {
                    let mut eval_ctx = match yolk.prepare_eval_ctx_for_templates(mode) {
                        Ok(x) => x,
                        Err(e) => {
                            eprintln!("Error: {e:?}");
                            return;
                        }
                    };
                    match res {
                        Ok(events) => {
                            let changed = events
                                .into_iter()
                                .filter(|evt| {
                                    evt.paths.iter().any(|x| files_to_watch.contains(x))
                                        && (!matches!(evt.kind, notify::EventKind::Access(_)))
                                })
                                .flat_map(|x| x.paths.clone().into_iter())
                                .collect::<HashSet<_>>();
                            if changed.contains(&yolk.paths().yolk_lua_path()) {
                                if let Err(e) = yolk.sync_to_mode(mode) {
                                    eprintln!("Error: {e:?}");
                                }
                            } else {
                                for path in changed {
                                    if let Err(e) = yolk.sync_template_file(&mut eval_ctx, path) {
                                        eprintln!("Error: {e:?}");
                                    }
                                }
                            }
                        }
                        Err(error) => tracing::error!("Error: {error:?}"),
                    }
                },
            )
            .into_diagnostic()?;

            for dir in dirs_to_watch {
                tracing::info!("Watching {}", dir.display());
                debouncer
                    .watch(&dir, notify::RecursiveMode::Recursive)
                    .into_diagnostic()?;
            }
            // Watch the yolk dir non-recursively to catch updates to yolk.lua
            debouncer
                .watch(&script_path, notify::RecursiveMode::NonRecursive)
                .into_diagnostic()?;

            loop {}
        }
    }
    Ok(())
}
