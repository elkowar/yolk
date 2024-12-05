use std::path::{Path, PathBuf};

use miette::{Context as _, IntoDiagnostic as _};

/// Create a symlink at `link` pointing to `original`.
pub fn create_symlink(original: impl AsRef<Path>, link: impl AsRef<Path>) -> miette::Result<()> {
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

    fn to_abbrev_str(&self) -> String {
        match (
            dirs::home_dir(),
            dirs::config_dir().map(|x| x.join("yolk").join("eggs")),
        ) {
            (Some(home), Some(eggs)) => self
                .strip_prefix(&eggs)
                .map(|x| PathBuf::from("eggs").join(x))
                .or_else(|_| self.strip_prefix(&home).map(|x| PathBuf::from("~").join(x)))
                .unwrap_or_else(|_| self.into())
                .display()
                .to_string(),
            _ => self.display().to_string(),
        }
    }
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
