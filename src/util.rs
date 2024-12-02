use std::{error::Error, path::Path};

use miette::{Diagnostic, LabeledSpan};
use pest::Span;

/// Create a symlink at `link` pointing to `original`.
pub fn create_symlink(original: impl AsRef<Path>, link: impl AsRef<Path>) -> std::io::Result<()> {
    #[cfg(unix)]
    fs_err::os::unix::fs::symlink(original, link)?;
    #[cfg(target_os = "windows")]
    {
        if original.as_ref().is_dir() {
            fs_err::os::windows::fs::symlink_dir(original, link)?;
        } else {
            std::os::windows::fs::symlink_file(original, link)?;
        }
    }
    Ok(())
}

#[extend::ext(pub)]
impl<T> Result<T, miette::Report> {
    fn as_span_diagnostic(self, span: Span<'_>) -> Result<T, miette::Report> {
        self.map_err(|e| create_diagnostic(span, e))
    }
}

pub fn create_diagnostic(span: Span<'_>, e: miette::Report) -> miette::Report {
    miette::miette!(
        labels = vec![LabeledSpan::at(span.start()..span.end(), "here")],
        "{}",
        e
    )
}
