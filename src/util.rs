use std::path::Path;

/// Create a symlink directory at `link` pointing to `original`.
pub fn create_symlink_dir(
    original: impl AsRef<Path>,
    link: impl AsRef<Path>,
) -> std::io::Result<()> {
    #[cfg(unix)]
    fs_err::os::unix::fs::symlink(original, link)?;
    #[cfg(target_os = "windows")]
    fs_err::os::windows::fs::symlink_dir(original, link)?;
    Ok(())
}
