use miette::{Context as _, IntoDiagnostic, Result};
use std::{
    io::Write,
    path::{Path, PathBuf},
    process::Stdio,
};

pub struct Git {
    root_path: PathBuf,
    git_dir_path: PathBuf,
}
impl Git {
    pub fn new(root_path: impl Into<PathBuf>, git_dir_path: impl Into<PathBuf>) -> Self {
        Self {
            root_path: root_path.into(),
            git_dir_path: git_dir_path.into(),
        }
    }

    pub fn start_git_command_builder(&self) -> std::process::Command {
        let mut cmd = std::process::Command::new("git");
        cmd.current_dir(&self.root_path).args([
            "--git-dir",
            &self.git_dir_path.to_string_lossy(),
            "--work-tree",
            &self.root_path.to_string_lossy(),
        ]);
        cmd
    }

    pub fn add(&self, path: impl AsRef<Path>) -> Result<()> {
        let output = self
            .start_git_command_builder()
            .args(["add", &path.as_ref().display().to_string()])
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .into_diagnostic()
            .wrap_err("git add failed to run")?;
        miette::ensure!(output.status.success(), "git add failed");
        Ok(())
    }

    /// Returns true if the given path is ignored by git.
    pub fn check_ignore(&self, path: impl AsRef<Path>) -> Result<bool> {
        let output = self
            .start_git_command_builder()
            .args(["check-ignore", "-q", &path.as_ref().display().to_string()])
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .into_diagnostic()
            .wrap_err("git check-ignore failed to run")?;
        miette::ensure!(output.status.success(), "git check-ignore failed");
        Ok(output.status.success())
    }

    /// Run git update-index.
    /// - `sha1` should be the output of [`Git::hash_object`],
    /// - `path` should be the path of the file inside the repository
    pub fn update_index(&self, sha1: &str, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let path = if path.is_absolute() {
            path.strip_prefix(&self.root_path).into_diagnostic()?
        } else {
            path
        };
        // TODO: use the exact mode of the file, rather than a default regular file mode
        let output = self
            .start_git_command_builder()
            .args([
                "update-index",
                "--add",
                "--cacheinfo",
                &format!("100644,{sha1},{}", path.display()),
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .into_diagnostic()
            .wrap_err("git update-index failed to run")?;
        miette::ensure!(output.status.success(), "git update-index failed");
        Ok(())
    }

    pub fn hash_object(&self, content: &[u8]) -> Result<String> {
        let mut child = self
            .start_git_command_builder()
            .args(["hash-object", "-t", "blob", "-w", "--stdin"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .into_diagnostic()
            .wrap_err("Failed to spawn git process")?;
        child
            .stdin
            .take()
            .unwrap()
            .write_all(content)
            .into_diagnostic()
            .wrap_err("Failed to write file content to git hash-object")?;
        let output = child
            .wait_with_output()
            .into_diagnostic()
            .wrap_err("git hash-object failed")?;
        miette::ensure!(output.status.success(), "git hash-object failed");
        Ok(String::from_utf8(output.stdout)
            .into_diagnostic()?
            .trim()
            .to_string())
    }
}
