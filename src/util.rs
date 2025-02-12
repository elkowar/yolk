use std::{
    collections::HashSet,
    io::Write,
    path::{Path, PathBuf},
    process::Stdio,
};

use cached::UnboundCache;
use fs_err::OpenOptions;
use miette::{Context as _, IntoDiagnostic as _};
use regex::Regex;
use which::which_global;

use crate::yolk_paths::default_yolk_dir;

fn try_sudo(args: &[String]) -> miette::Result<()> {
    let sudo_command = which_global("sudo")
        .or_else(|_| which_global("doas"))
        .or_else(|_| which_global("run0"))
        .map_err(|_| miette::miette!("No sudo, doas, or run0 command found"))?;

    let mut cmd = std::process::Command::new(sudo_command);
    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .args(args);
    let output = cmd.output().into_diagnostic()?;
    if !output.status.success() {
        tracing::error!(
            "Failed to run command with sudo: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[derive(Default, Debug)]
pub struct DeploymentPriviledgeTracker {
    /// Symlink creation mappings (actual_path, symlink_path) that failed due to insufficient permissions
    symlink_create: Vec<(PathBuf, PathBuf)>,
    /// Symlink deletion paths (symlink_path) that failed due to insufficient permissions
    symlink_remove: Vec<PathBuf>,
}

impl DeploymentPriviledgeTracker {
    pub fn new() -> Self {
        Self {
            symlink_create: Vec::new(),
            symlink_remove: Vec::new(),
        }
    }

    pub fn failed_creations(&self) -> &Vec<(PathBuf, PathBuf)> {
        &self.symlink_create
    }

    pub fn failed_deletions(&self) -> &Vec<PathBuf> {
        &self.symlink_remove
    }

    pub fn try_run_elevated(self) -> miette::Result<()> {
        if self.symlink_create.is_empty() && self.symlink_remove.is_empty() {
            tracing::trace!("No priviledge escalation necessary, all symlink operations succeeded");
            return Ok(());
        }
        let create_args = self.symlink_create.iter().map(|(original, symlink)| {
            [
                "--create-symlink".to_string(),
                format!("{}::::{}", original.display(), symlink.display()),
            ]
        });
        let delete_args = self.symlink_remove.iter().map(|symlink| {
            [
                "--delete-symlink".to_string(),
                symlink.to_string_lossy().to_string(),
            ]
        });
        let yolk_binary = std::env::args().nth(0).unwrap_or("yolk".to_string());
        let yolk_binary_path = if yolk_binary.starts_with('/') {
            yolk_binary
        } else {
            which_global(yolk_binary)
                .map(|x| x.to_string_lossy().to_string())
                .unwrap_or_else(|_| "yolk".to_string())
        };
        let args = [yolk_binary_path, "root-manage-symlinks".to_string()]
            .into_iter()
            .chain(create_args.flatten())
            .chain(delete_args.flatten())
            .collect::<Vec<_>>();
        tracing::info!("Attempting to run command as root: {}", args.join(" "));
        try_sudo(&args)?;
        Ok(())
    }

    /// Create a symlink from at the path `link` pointing to the `original` file.
    pub fn create_symlink(
        &mut self,
        original: impl AsRef<Path>,
        link: impl AsRef<Path>,
    ) -> miette::Result<()> {
        let link = link.as_ref();
        let original = original.as_ref();
        tracing::trace!("Creating symlink at {} -> {}", link.abbr(), original.abbr());

        if let Err(err) = symlink::symlink_auto(original, link) {
            if err.kind() != std::io::ErrorKind::PermissionDenied {
                return Err(err).into_diagnostic().wrap_err_with(|| {
                    format!(
                        "Failed to create symlink at {} -> {}",
                        link.abbr(),
                        original.abbr()
                    )
                })?;
            }
            self.symlink_create
                .push((original.to_path_buf(), link.to_path_buf()));
        }
        return Ok(());
    }
    /// Remove a symlink from at the path `link` pointing to the `original` file.
    pub fn delete_symlink(&mut self, path: impl AsRef<Path>) -> miette::Result<()> {
        let path = path.as_ref();
        if !path.is_symlink() {
            miette::bail!("Path is not a symlink: {}", path.abbr());
        }
        let result = if path.symlink_metadata().into_diagnostic()?.is_dir() {
            symlink::remove_symlink_dir(path)
        } else {
            match symlink::remove_symlink_file(path) {
                Ok(()) => Ok(()),
                Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => Err(e),
                Err(e) => {
                    tracing::debug!(
                        "Failed to remove file symlink, trying dir symlink removal as fallback: {:?}", e
                    );
                    symlink::remove_symlink_dir(path)
                }
            }
        };
        match result {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                self.symlink_remove.push(path.to_path_buf());
            }
            Err(e) => {
                return Err(e)
                    .into_diagnostic()
                    .wrap_err(format!("Failed to remove symlink at {}", path.abbr()))
            }
        }
        Ok(())
    }
}

/// Create a symlink at `link` pointing to `original`.
pub fn create_symlink(original: impl AsRef<Path>, link: impl AsRef<Path>) -> miette::Result<()> {
    let link = link.as_ref();
    let original = original.as_ref();
    tracing::trace!("Creating symlink at {} -> {}", link.abbr(), original.abbr());
    symlink::symlink_auto(original, link)
        .into_diagnostic()
        .wrap_err_with(|| {
            format!(
                "Failed to create symlink at {} -> {}",
                link.abbr(),
                original.abbr()
            )
        })?;
    Ok(())
}

/// Delete a symlink at `path`, but only if it actually is a symlink.
pub fn remove_symlink(path: impl AsRef<Path>) -> miette::Result<()> {
    let path = path.as_ref();
    if !path.is_symlink() {
        miette::bail!("Path is not a symlink: {}", path.abbr());
    }
    if path.symlink_metadata().into_diagnostic()?.is_dir() {
        symlink::remove_symlink_dir(path)
            .into_diagnostic()
            .wrap_err_with(|| format!("Failed to remove symlink dir at {}", path.abbr()))?;
    } else {
        let result = symlink::remove_symlink_file(path);
        if let Err(e) = result {
            symlink::remove_symlink_dir(path)
                .into_diagnostic()
                .wrap_err("Failed to remove symlink dir as fallback from symlink file")
                .wrap_err_with(|| {
                    format!("Failed to remove symlink file at {}: {e:?}", path.abbr())
                })?;
        }
    }
    Ok(())
}

/// Rename or move a file, but only if the destination doesn't exist.
/// This is a safer verison of [`std::fs::rename`] that doesn't overwrite files.
pub fn rename_safely(original: impl AsRef<Path>, new: impl AsRef<Path>) -> miette::Result<()> {
    let original = original.as_ref();
    let new = new.as_ref();
    tracing::trace!("Renaming {} -> {}", original.abbr(), new.abbr());
    miette::ensure!(
        !new.exists(),
        "Failed to move file {} to {}: File already exists.",
        original.abbr(),
        new.abbr()
    );
    fs_err::rename(original, new)
        .into_diagnostic()
        .wrap_err("Failed to rename file")?;
    Ok(())
}

pub fn file_entries_recursive(
    path: impl AsRef<Path>,
) -> impl Iterator<Item = miette::Result<PathBuf>> {
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter(|x| x.as_ref().map_or(true, |x| !x.path().is_dir()))
        .map(|x| x.map(|x| x.into_path()))
        .map(|x| x.into_diagnostic())
}

/// Ensure that a file contains the given lines, appending them if they are missing. If the file does not yet exist, it will be created.
pub fn ensure_file_contains_lines(path: impl AsRef<Path>, lines: &[&str]) -> miette::Result<()> {
    let path = path.as_ref();

    let mut trailing_newline_exists = true;

    let existing_lines = if path.exists() {
        let content = fs_err::read_to_string(path).into_diagnostic()?;
        trailing_newline_exists = content.ends_with('\n');
        content.lines().map(|x| x.to_string()).collect()
    } else {
        HashSet::new()
    };
    if lines.iter().all(|x| existing_lines.contains(*x)) {
        return Ok(());
    }
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .into_diagnostic()?;
    let missing_lines = lines.iter().filter(|x| !existing_lines.contains(**x));
    if !trailing_newline_exists {
        writeln!(file).into_diagnostic()?;
    }
    for line in missing_lines {
        writeln!(file, "{}", line).into_diagnostic()?;
    }
    Ok(())
}

/// Ensure that a file does not contain the given lines, removing them if they are present.
pub fn ensure_file_doesnt_contain_lines(
    path: impl AsRef<Path>,
    lines: &[&str],
) -> miette::Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(());
    }
    let content = fs_err::read_to_string(path).into_diagnostic()?;
    let trailing_newline_exists = content.ends_with('\n');
    let remaining_lines = content
        .lines()
        .filter(|x| !lines.contains(x))
        .collect::<Vec<_>>();
    if remaining_lines.len() == content.lines().count() {
        return Ok(());
    }
    let new_content = format!(
        "{}{}",
        remaining_lines.join("\n"),
        if trailing_newline_exists { "\n" } else { "" }
    );
    fs_err::write(path, new_content).into_diagnostic()?;
    Ok(())
}

#[extend::ext(pub)]
impl Path {
    /// [`fs_err::canonicalize`] but on windows it doesn't return UNC paths.
    fn canonical(&self) -> miette::Result<PathBuf> {
        Ok(dunce::simplified(&fs_err::canonicalize(self).into_diagnostic()?).to_path_buf())
    }

    /// Stringify the path into an abbreviated form.
    ///
    /// This replaces the home path with `~`, as well as reducing paths that point into the eggs directory to `eggs/rest/of/path`.
    fn abbr(&self) -> String {
        let eggs = default_yolk_dir().join("eggs");
        match dirs::home_dir() {
            Some(home) => self
                .strip_prefix(&eggs)
                .map(|x| PathBuf::from("eggs").join(x))
                .or_else(|_| self.strip_prefix(&home).map(|x| PathBuf::from("~").join(x)))
                .unwrap_or_else(|_| self.into())
                .display()
                .to_string(),
            _ => self.display().to_string(),
        }
    }

    /// Expands `~` in a path to the home directory.
    fn expanduser(&self) -> PathBuf {
        #[cfg(not(test))]
        let Some(home) = dirs::home_dir() else {
            return self.to_path_buf();
        };
        #[cfg(test)]
        let home = test_util::get_home_dir();

        if let Some(first) = self.components().next() {
            if first.as_os_str().to_string_lossy() == "~" {
                return home.join(self.strip_prefix("~").unwrap());
            }
        }
        self.to_path_buf()
    }
}

pub fn create_regex(s: impl AsRef<str>) -> miette::Result<Regex> {
    cached::cached_key! {
         REGEXES: UnboundCache<String, Result<Regex, regex::Error>> = UnboundCache::new();
         Key = { s.to_string() };
         fn create_regex_cached(s: &str) -> Result<Regex, regex::Error> = {
             Regex::new(s)
         }
    }
    create_regex_cached(s.as_ref()).into_diagnostic()
}

#[cfg(test)]
pub mod test_util {
    use std::cell::RefCell;
    use std::path::PathBuf;
    use std::thread_local;

    use miette::IntoDiagnostic as _;

    thread_local! {
        static HOME_DIR: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
    }

    pub fn set_home_dir(path: PathBuf) {
        HOME_DIR.with(|home_dir| {
            *home_dir.borrow_mut() = Some(path);
        });
    }

    pub fn get_home_dir() -> PathBuf {
        HOME_DIR.with_borrow(|x| x.clone()).expect(
            "Home directory not set in this test. Use `set_home_dir` to set the home directory.",
        )
    }

    /// like <https://crates.io/crates/testresult>, but shows the debug output instead of display.
    pub type TestResult<T = ()> = std::result::Result<T, TestError>;

    #[derive(Debug)]
    pub enum TestError {}

    impl<T: std::fmt::Debug + std::fmt::Display> From<T> for TestError {
        #[track_caller] // Will show the location of the caller in test failure messages
        fn from(error: T) -> Self {
            // Use alternate format for rich error message for anyhow
            // See: https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations
            panic!("error: {} - {:?}", std::any::type_name::<T>(), error);
        }
    }

    pub fn setup_and_init_test_yolk() -> miette::Result<(
        assert_fs::TempDir,
        crate::yolk::Yolk,
        assert_fs::fixture::ChildPath,
    )> {
        use assert_fs::prelude::PathChild as _;

        let home = assert_fs::TempDir::new().into_diagnostic()?;
        let paths = crate::yolk_paths::YolkPaths::new(home.join("yolk"), home.to_path_buf());
        let yolk = crate::yolk::Yolk::new(paths);
        std::env::set_var("HOME", "/tmp/TEST_HOMEDIR_SHOULD_NOT_BE_USED");
        set_home_dir(home.to_path_buf());

        let eggs = home.child("yolk/eggs");
        let yolk_binary_path = assert_cmd::cargo::cargo_bin("yolk");
        yolk.init_yolk(Some(yolk_binary_path.to_string_lossy().as_ref()))?;
        Ok((home, yolk, eggs))
    }

    pub fn render_error(e: impl miette::Diagnostic) -> String {
        use miette::GraphicalReportHandler;

        let mut out = String::new();
        GraphicalReportHandler::new()
            .with_theme(miette::GraphicalTheme::unicode_nocolor())
            .render_report(&mut out, &e)
            .unwrap();
        out
    }

    pub fn render_report(e: miette::Report) -> String {
        use miette::GraphicalReportHandler;

        let mut out = String::new();
        GraphicalReportHandler::new()
            .with_theme(miette::GraphicalTheme::unicode_nocolor())
            .render_report(&mut out, e.as_ref())
            .unwrap();
        out
    }
}
