use std::{
    collections::HashSet,
    io::Read as _,
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::{Parser, Subcommand};
use miette::{IntoDiagnostic, Result};
use notify_debouncer_full::{new_debouncer, DebounceEventResult};
use owo_colors::OwoColorize as _;
use rhai::Dynamic;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};
use util::PathExt;
use yolk::{EvalMode, Yolk};

mod doc_generator;

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
    /// Evaluate an expression like it would be done in a template
    Eval {
        /// Evaluate in canonical context instead.
        #[arg(long)]
        canonical: bool,
        /// The expression to evaluate
        expr: String,
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
    /// Open your `yolk.rhai` or the given egg in your `$EDITOR` of choice
    Edit { egg: Option<String> },
    /// Watch for changes in your templated files and re-sync them when they change.
    Watch {
        #[arg(long)]
        canonical: bool,
        /// Don't actually update any files, just evaluate the templates and print any errors.
        #[arg(long)]
        no_sync: bool,
    },

    #[command(hide(true))]
    Docs { dir: PathBuf },
}

pub(crate) fn main() -> Result<()> {
    let args = Args::parse();

    let env_filter = if args.debug {
        tracing_subscriber::EnvFilter::from_str("debug").unwrap()
    } else {
        let default = if matches!(args.command, Command::Git { .. }) {
            EnvFilter::new("yolk=warn")
        } else {
            EnvFilter::new("yolk=info")
        };
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(default)
    };
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .without_time()
                .with_ansi(true)
                .with_level(true),
        )
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
            // TODO: Add a verification that exactly all the eggs in the eggs dir are defined in the
            // yolk.rhai file.

            yolk.paths().check()?;
            if yolk.paths().active_yolk_git_dir()? == yolk.paths().yolk_default_git_path() {
                println!("Yolk git is not safeguarded. It is recommended to run `yolk safeguard`.");
            }
            yolk.with_canonical_state(|| {
                yolk.paths()
                    .start_git_command_builder()?
                    .args(["status", "--short"])
                    .status()
                    .into_diagnostic()
            })?;
        }
        Command::List => {
            let mut eggs = yolk.list_eggs()?;
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
        Command::Sync { canonical } => yolk.sync_to_mode(match *canonical {
            true => EvalMode::Canonical,
            false => EvalMode::Local,
        })?,
        Command::Eval { expr, canonical } => {
            let mut eval_ctx = yolk.prepare_eval_ctx_for_templates(match *canonical {
                true => EvalMode::Canonical,
                false => EvalMode::Local,
            })?;
            let result = eval_ctx.eval_rhai::<Dynamic>(expr).map_err(|e| {
                miette::Report::from(e).with_source_code(
                    miette::NamedSource::new("<inline>", expr.to_string()).with_language("Rust"),
                )
            })?;
            println!("{result}");
        }
        Command::Git { command } => {
            let mut cmd = yolk.paths().start_git_command_builder()?;
            cmd.args(command);
            // if the command is `git push`, we don't need to enter canonical state
            // before executing it

            let first_cmd = command.first().map(|x| x.as_ref());
            if first_cmd == Some("push")
                || first_cmd == Some("init")
                || first_cmd == Some("pull")
                || first_cmd.is_none()
            {
                cmd.status().into_diagnostic()?;
            } else {
                yolk.with_canonical_state(|| {
                    cmd.status().into_diagnostic()?;
                    Ok(())
                })?;
            }
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
            let mut eval_ctx = yolk.prepare_eval_ctx_for_templates(match *canonical {
                true => EvalMode::Canonical,
                false => EvalMode::Local,
            })?;
            let result = yolk.eval_template(&mut eval_ctx, "unnamed", &text)?;
            println!("{}", result);
        }
        Command::Edit { egg } => {
            let path = match egg {
                Some(egg_name) => {
                    let egg = yolk.get_egg(egg_name)?;
                    egg.find_first_deployed_symlink()?
                        .unwrap_or_else(|| yolk.paths().egg_path(egg_name))
                }
                None => yolk.paths().yolk_rhai_path(),
            };

            if let Some(parent) = path.parent() {
                let _ = std::env::set_current_dir(parent);
            }
            edit::edit_file(path).into_diagnostic()?;
        }
        Command::Watch { canonical, no_sync } => {
            let no_sync = *no_sync;
            let mode = match *canonical {
                true => EvalMode::Canonical,
                false => EvalMode::Local,
            };

            let mut dirs_to_watch = HashSet::new();
            let mut files_to_watch = HashSet::new();
            let script_path = yolk.paths().yolk_rhai_path();
            files_to_watch.insert(script_path.clone());

            let mut eval_ctx = yolk.prepare_eval_ctx_for_templates(mode)?;
            let egg_configs = yolk.load_egg_configs(&mut eval_ctx)?;

            for (egg_name, egg_config) in egg_configs {
                for path in egg_config.templates_globexpanded(yolk.paths().egg_path(&egg_name))? {
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

                    let mut on_file_updated = |path: &Path| {
                        let result = if no_sync {
                            let Ok(content) = fs_err::read_to_string(path) else {
                                return;
                            };
                            let path = path.to_string_lossy();
                            yolk.eval_template(&mut eval_ctx, &path, &content)
                                .map(|_| ())
                        } else {
                            yolk.sync_template_file(&mut eval_ctx, path)
                        };
                        if let Err(e) = result {
                            eprintln!("Error: {e:?}");
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
                            if changed.contains(&yolk.paths().yolk_rhai_path()) {
                                if no_sync {
                                    for file in files_to_watch.iter() {
                                        on_file_updated(file);
                                    }
                                } else if let Err(e) = yolk.sync_to_mode(mode) {
                                    eprintln!("Error: {e:?}");
                                }
                            } else {
                                for path in changed {
                                    on_file_updated(&path);
                                }
                            }
                        }
                        Err(error) => tracing::error!("Error: {error:?}"),
                    }
                },
            )
            .into_diagnostic()?;

            for dir in dirs_to_watch {
                tracing::info!("Watching {}", dir.to_abbrev_str());
                debouncer
                    .watch(&dir, notify::RecursiveMode::Recursive)
                    .into_diagnostic()?;
            }
            // Watch the yolk dir non-recursively to catch updates to yolk.rhai
            debouncer
                .watch(&script_path, notify::RecursiveMode::NonRecursive)
                .into_diagnostic()?;

            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        Command::Docs { dir } => {
            let docs = doc_generator::generate_docs(yolk)?;
            for (name, docs) in docs {
                fs_err::write(dir.join(format!("{name}.md")), docs).into_diagnostic()?;
            }
        }
    }
    Ok(())
}
