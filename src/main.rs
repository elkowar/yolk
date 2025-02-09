use std::{
    collections::HashSet,
    io::Read as _,
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::{Parser, Subcommand};
use fs_err::PathExt;
use miette::{IntoDiagnostic, Result};
use notify_debouncer_full::{new_debouncer, DebounceEventResult};
use owo_colors::OwoColorize as _;
use rhai::Dynamic;
use tracing_subscriber::{
    filter, fmt::format::FmtSpan, layer::SubscriberExt as _, util::SubscriberInitExt as _,
    EnvFilter, Layer,
};

use yolk::{
    util::PathExt as _,
    yolk::{EvalMode, Yolk},
};

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
    #[arg(long, short = 'v', global = true, action = clap::ArgAction::Count)]
    debug: u8,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Initialize the yolk directory.
    ///
    /// This creates a directory called `yolk` within your config directory, and initializes it with the basic yolk directory structure.
    Init,
    /// Show the current state of your yolk eggs.
    Status,

    /// Make sure you don't accidentally commit your local egg states.
    ///
    /// This renames `.git` to `.yolk_git` to ensure that git interaction happens through the yolk CLI
    Safeguard,
    /// Make sure you don't accidentally commit your local egg states
    ///
    /// Evaluate a rhai expression.
    ///
    /// The expression is executed in the same scope that template tag expression are evaluated in.
    Eval {
        /// Evaluate in canonical context instead.
        #[arg(long)]
        canonical: bool,
        /// The rhai expression to evaluate.
        expr: String,
    },

    /// Sync all template files and sync the deployments to match the configuration in yolk.rhai.
    ///
    /// This will modify your template files in place, as well as deploying or undeploying any eggs to match your egg configuration.
    #[clap(alias = "s")]
    Sync {
        /// Sync to canonical state. This should only be necessary for debugging purposes.
        #[arg(long)]
        canonical: bool,
    },

    /// Evaluate a given templated file, or read a templated string from stdin.
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

    /// Open your `yolk.rhai` or the given egg in your `$EDITOR` of choice.
    Edit { egg: Option<String> },

    /// Watch for changes in your templated files and re-sync them when they change.
    Watch {
        #[arg(long)]
        canonical: bool,
        /// Don't actually update any files, just evaluate the templates and print any errors.
        #[arg(long)]
        no_sync: bool,
    },

    /// Run a git-command within the yolk directory.
    #[clap(alias = "g")]
    Git {
        #[clap(allow_hyphen_values = true)]
        command: Vec<String>,
    },

    #[cfg(feature = "docgen")]
    #[command(hide(true))]
    Docs { dir: PathBuf },
}

pub(crate) fn main() -> Result<()> {
    let args = Args::parse();

    init_logging(&args);
    if let Err(err) = run_command(args) {
        eprintln!("{:?}", err);
    }
    Ok(())
}

fn init_logging(args: &Args) {
    let env_filter = match &args.debug {
        0 if matches!(args.command, Command::Git { .. }) => {
            EnvFilter::from_str("yolk=warn").unwrap()
        }
        0 => EnvFilter::from_str("yolk=info").unwrap(),
        1 => EnvFilter::from_str("yolk=debug").unwrap(),
        _ => EnvFilter::from_str("yolk=trace").unwrap(),
    };
    let mut format_layer = tracing_subscriber::fmt::layer()
        .without_time()
        .with_ansi(true)
        .with_target(false)
        .with_level(true)
        .with_writer(std::io::stderr);
    let mut include_span_info = false;
    if args.debug > 1 {
        format_layer = format_layer.with_target(true).with_level(true);
    }
    if args.debug > 2 {
        format_layer = format_layer
            .with_file(true)
            .with_line_number(true)
            .with_target(false);
    }
    if args.debug > 3 {
        format_layer = format_layer.with_span_events(FmtSpan::ACTIVE);
        include_span_info = true;
    }
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(env_filter);
    tracing_subscriber::registry()
        .with(format_layer.with_filter(filter::filter_fn(move |meta| {
            !meta.is_span() || include_span_info
        })))
        .with(env_filter)
        .init();
}

fn run_command(args: Args) -> Result<()> {
    let mut yolk_paths = yolk::yolk_paths::YolkPaths::from_env();
    if let Some(d) = args.yolk_dir {
        tracing::trace!("Setting yolk dir to {}", d.display());
        yolk_paths.set_yolk_dir(d);
    }
    if let Some(d) = args.home_dir {
        tracing::trace!("Setting home dir to {}", d.display());
        yolk_paths.set_home_dir(d);
    }

    let yolk = Yolk::new(yolk_paths);
    match &args.command {
        Command::Init => yolk.init_yolk(None)?,
        // TODO: we shoul likely also do this as part of init, maybe
        Command::Safeguard => yolk.paths().safeguard_git_dir()?,
        Command::Status => {
            yolk.init_git_config(None)?;
            yolk.paths().check()?;
            yolk.validate_config_invariants()?;
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
        Command::Sync { canonical } => {
            // Lets always ensure that the yolk dir is in a properly set up state.
            // This should later be replaced with some sort of version-aware compatibility check.
            yolk.init_git_config(None)?;

            yolk.sync_to_mode(match *canonical {
                true => EvalMode::Canonical,
                false => EvalMode::Local,
            })?
        }
        Command::Eval { expr, canonical } => {
            let mut eval_ctx = yolk.prepare_eval_ctx_for_templates(match *canonical {
                true => EvalMode::Canonical,
                false => EvalMode::Local,
            })?;
            let result = eval_ctx
                .eval_rhai::<Dynamic>(expr)
                .map_err(|e| e.into_report("<inline>", expr))?;
            println!("{result}");
        }
        Command::Git { command } => {
            yolk.init_git_config(None)?;
            yolk.validate_config_invariants()?;
            yolk.paths()
                .start_git_command_builder()?
                .args(command)
                .status()
                .into_diagnostic()?;
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

        Command::Edit { egg: egg_name } => {
            match egg_name {
                Some(egg_name) => {
                    let egg = yolk.load_egg(egg_name)?;
                    let cd_path = egg
                        .find_first_deployed_symlink()?
                        .unwrap_or_else(|| yolk.paths().egg_path(egg_name));
                    let _ = std::env::set_current_dir(&cd_path);
                    let mut main_file = egg.config().main_file.clone();
                    // If no main_file is specified and there's exactly one file in the egg directory, use that
                    if main_file.is_none() {
                        let mut files = egg.path().fs_err_read_dir().into_diagnostic()?;
                        if let Some(first_file) = files.next() {
                            if files.next().is_none() {
                                main_file = Some(first_file.into_diagnostic()?.path());
                            }
                        }
                    }
                    if let Some(ref main_file) = main_file {
                        edit::edit_file(egg.path().join(main_file)).into_diagnostic()?;
                    } else {
                        edit::edit_file(&cd_path).into_diagnostic()?;
                    }
                }
                None => {
                    let _ = std::env::set_current_dir(yolk.paths().root_path());
                    edit::edit_file(yolk.paths().yolk_rhai_path()).into_diagnostic()?;
                }
            };
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
                tracing::info!("Watching {}", dir.abbr());
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

        #[cfg(feature = "docgen")]
        Command::Docs { dir } => {
            let docs = doc_generator::generate_docs(yolk)?;
            for (name, docs) in docs {
                fs_err::write(dir.join(format!("{name}.md")), docs).into_diagnostic()?;
            }
        }
    }
    Ok(())
}
