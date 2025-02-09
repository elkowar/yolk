use miette::{Context as _, IntoDiagnostic, Result};
use std::{
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
}
