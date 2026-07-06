use std::{
    collections::HashMap,
    io::{IsTerminal as _, Write as _},
    path::{Path, PathBuf},
};

use indoc::printdoc;
use miette::{IntoDiagnostic as _, Result};
use yolk::{eggs_config::DeploymentStrategy, util::PathExt as _, yolk::Yolk};

pub enum AdoptAction {
    PrintConfig,
    AppendConfigAndSync,
}

pub fn prompt_adopt_action(egg_name: &str) -> Result<AdoptAction> {
    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        return Ok(AdoptAction::PrintConfig);
    }

    loop {
        printdoc! {r"
            How should Yolk finish adopting `{egg_name}`?
              1) Move files and print the yolk.rhai config to add manually
              2) Append the config to yolk.rhai and run `yolk sync`
        "};
        print!("Select an option [1/2] (default: 1): ");
        std::io::stdout().flush().into_diagnostic()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).into_diagnostic()?;
        match input.trim() {
            "" | "1" => return Ok(AdoptAction::PrintConfig),
            "2" => return Ok(AdoptAction::AppendConfigAndSync),
            _ => println!("Please enter 1 or 2.\n"),
        }
    }
}

pub fn print_adopt_instructions(
    yolk: &Yolk,
    egg_name: &str,
    strategy: DeploymentStrategy,
    templates: &[PathBuf],
    target_paths: &HashMap<PathBuf, PathBuf>,
    fallback_target: &Path,
) {
    println!("Add this entry to the `eggs` map in your yolk.rhai:\n");
    println!(
        "    {}\n",
        format_adopt_config_entry(
            yolk.paths().home_path(),
            egg_name,
            strategy,
            templates,
            target_paths,
            fallback_target,
        )
    );
    println!(
        "Yolk moved the files into `eggs/{egg_name}`. After adding the entry, run `yolk sync` to create the symlink so the config is available at its target path."
    );
}

pub fn append_adopt_config_to_yolk_rhai(
    yolk: &Yolk,
    egg_name: &str,
    strategy: DeploymentStrategy,
    templates: &[PathBuf],
    target_paths: &HashMap<PathBuf, PathBuf>,
    fallback_target: &Path,
) -> Result<()> {
    let config_value = format_adopt_config_value(
        yolk.paths().home_path(),
        strategy,
        templates,
        target_paths,
        fallback_target,
    );
    let snippet = format!(
        "\neggs[{}] = {}; // Added by `yolk adopt {egg_name}`.\n",
        fmt_rhai_str(egg_name),
        config_value,
    );
    let mut file = fs_err::OpenOptions::new()
        .append(true)
        .open(yolk.paths().yolk_rhai_path())
        .into_diagnostic()?;
    file.write_all(snippet.as_bytes()).into_diagnostic()?;
    Ok(())
}

fn format_adopt_config_entry(
    home_path: &Path,
    egg_name: &str,
    strategy: DeploymentStrategy,
    templates: &[PathBuf],
    target_paths: &HashMap<PathBuf, PathBuf>,
    fallback_target: &Path,
) -> String {
    format!(
        "{}: {},",
        fmt_rhai_str(egg_name),
        format_adopt_config_value(
            home_path,
            strategy,
            templates,
            target_paths,
            fallback_target
        ),
    )
}

fn format_adopt_config_value(
    home_path: &Path,
    strategy: DeploymentStrategy,
    templates: &[PathBuf],
    target_paths: &HashMap<PathBuf, PathBuf>,
    fallback_target: &Path,
) -> String {
    format!(
        "#{{ enabled: true, strategy: {}, targets: {}, templates: {} }}",
        fmt_rhai_str(deployment_strategy_name(strategy)),
        format_adopt_targets(home_path, target_paths, fallback_target),
        format_path_list(templates),
    )
}

fn deployment_strategy_name(strategy: DeploymentStrategy) -> &'static str {
    match strategy {
        DeploymentStrategy::Merge => "merge",
        DeploymentStrategy::Put => "put",
    }
}

fn format_adopt_targets(
    home_path: &Path,
    target_paths: &HashMap<PathBuf, PathBuf>,
    fallback_target: &Path,
) -> String {
    if target_paths.is_empty() {
        return fmt_rhai_str(format_config_path(home_path, fallback_target));
    }

    let mut mappings = target_paths.iter().collect::<Vec<_>>();
    mappings.sort_by_key(|(source, _)| source.to_string_lossy().to_string());
    let entries = mappings
        .into_iter()
        .map(|(source, target)| {
            let src_path = fmt_rhai_str(source.to_string_lossy());
            let target_path = fmt_rhai_str(format_config_path(
                home_path,
                &canonicalize_target_path(target),
            ));
            format!("{src_path}: {target_path}")
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("#{{ {entries} }}")
}

fn format_path_list(paths: &[PathBuf]) -> String {
    if paths.is_empty() {
        return "[]".to_string();
    }
    format!(
        "[{}]",
        paths
            .iter()
            .map(|path| fmt_rhai_str(path.to_string_lossy()))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn canonicalize_target_path(path: &Path) -> PathBuf {
    if path.exists() {
        return path.canonical().unwrap_or_else(|_| path.to_path_buf());
    }

    match (path.parent(), path.file_name()) {
        (Some(parent), Some(file_name)) => parent
            .canonical()
            .map(|parent| parent.join(file_name))
            .unwrap_or_else(|_| path.to_path_buf()),
        _ => path.to_path_buf(),
    }
}

fn format_config_path(home_path: &Path, path: &Path) -> String {
    if let Ok(relative) = path.strip_prefix(home_path) {
        if relative.as_os_str().is_empty() {
            "~".to_string()
        } else {
            PathBuf::from("~").join(relative).display().to_string()
        }
    } else {
        path.display().to_string()
    }
}

fn fmt_rhai_str(value: impl AsRef<str>) -> String {
    format!(
        "\"{}\"",
        value.as_ref().replace('\\', "\\\\").replace('"', "\\\"")
    )
}
