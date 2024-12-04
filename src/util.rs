use std::{
    ops::Range,
    path::{Path, PathBuf},
};

use miette::{Context as _, IntoDiagnostic as _, LabeledSpan};
use pest::Span;

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
impl<T> Result<T, miette::Report> {
    fn as_span_diagnostic_range(self, span: Range<usize>) -> Result<T, miette::Report> {
        self.map_err(|e| create_diagnostic(span, e))
    }
    fn as_span_diagnostic(self, span: Span<'_>) -> Result<T, miette::Report> {
        self.map_err(|e| create_diagnostic(span.start()..span.end(), e))
    }
}

pub fn create_diagnostic(span: Range<usize>, e: miette::Report) -> miette::Report {
    miette::miette!(labels = vec![LabeledSpan::at(span, "here")], "{}", e)
}

#[extend::ext(pub)]
impl Path {
    /// [`fs_err::canonicalize`] but on windows it doesn't return UNC paths.
    fn canonical(&self) -> miette::Result<PathBuf> {
        Ok(dunce::simplified(&fs_err::canonicalize(self).into_diagnostic()?).to_path_buf())
    }
}
