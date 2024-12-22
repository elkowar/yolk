use std::path::{Path, PathBuf};

use cached::UnboundCache;
use miette::{Context as _, IntoDiagnostic as _};
use regex::Regex;

use crate::yolk_paths::default_yolk_dir;

/// Rename or move a file, but only if the destination doesn't exist.
/// This is a safer verison of [`std::fs::rename`] that doesn't overwrite files.
pub fn rename_safely(original: impl AsRef<Path>, new: impl AsRef<Path>) -> miette::Result<()> {
    let original = original.as_ref();
    let new = new.as_ref();
    tracing::trace!("Renaming {} -> {}", original.abbr(), new.abbr());
    if new.exists() {
        miette::bail!(
            "Failed to move file {} to {}: File already exists.",
            original.abbr(),
            new.abbr()
        );
    }
    fs_err::rename(original, new)
        .into_diagnostic()
        .wrap_err("Failed to rename file")?;
    Ok(())
}

/// Create a symlink at `link` pointing to `original`.
pub fn create_symlink(original: impl AsRef<Path>, link: impl AsRef<Path>) -> miette::Result<()> {
    let link = link.as_ref();
    let original = original.as_ref();
    tracing::trace!("Creating symlink at {} -> {}", link.abbr(), original.abbr());
    #[cfg(unix)]
    fs_err::os::unix::fs::symlink(original, link)
        .into_diagnostic()
        .wrap_err("Failed to create symlink")?;
    #[cfg(target_os = "windows")]
    {
        if original.as_ref().is_dir() {
            fs_err::os::windows::fs::symlink_dir(original, link)
                .into_diagnostic()
                .wrap_err("Failed to create symlink")?;
        } else {
            std::os::windows::fs::symlink_file(original, link)
                .into_diagnostic()
                .wrap_err("Failed to create symlink")?;
        }
    }
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
        let home = PathBuf::from(std::env::var("HOME").unwrap());

        if let Some(first) = self.components().next() {
            if first.as_os_str().to_string_lossy() == "~" {
                return home.join(self.strip_prefix("~").unwrap());
            }
        }
        self.to_path_buf()
    }
}

/// like <https://crates.io/crates/testresult>, but shows the debug output instead of display.
#[cfg(test)]
pub type TestResult<T = ()> = std::result::Result<T, TestError>;

#[cfg(test)]
#[derive(Debug)]
pub enum TestError {}

#[cfg(test)]
impl<T: std::fmt::Debug + std::fmt::Display> From<T> for TestError {
    #[track_caller] // Will show the location of the caller in test failure messages
    fn from(error: T) -> Self {
        // Use alternate format for rich error message for anyhow
        // See: https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations
        panic!("error: {} - {:?}", std::any::type_name::<T>(), error);
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
pub fn setup_and_init_test_yolk() -> miette::Result<(
    assert_fs::TempDir,
    crate::yolk::Yolk,
    assert_fs::fixture::ChildPath,
)> {
    use assert_fs::prelude::PathChild as _;

    let home = assert_fs::TempDir::new().into_diagnostic()?;
    let paths = crate::yolk_paths::YolkPaths::new(home.join("yolk"), home.to_path_buf());
    let yolk = crate::yolk::Yolk::new(paths);
    std::env::set_var("HOME", home.to_string_lossy().to_string());
    let eggs = home.child("yolk/eggs");
    yolk.init_yolk()?;
    Ok((home, yolk, eggs))
}

#[cfg(test)]
pub fn render_error(e: impl miette::Diagnostic) -> String {
    use miette::GraphicalReportHandler;

    let mut out = String::new();
    GraphicalReportHandler::new()
        .with_theme(miette::GraphicalTheme::unicode_nocolor())
        .render_report(&mut out, &e)
        .unwrap();
    out
}

#[cfg(test)]
pub fn miette_no_color() {
    miette::set_hook(Box::new(|_| {
        Box::new(miette::MietteHandlerOpts::new().color(false).build())
    }))
    .unwrap();
}
