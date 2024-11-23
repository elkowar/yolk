use std::path::Path;

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
