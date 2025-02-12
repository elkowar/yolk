use std::{
    path::{Path, PathBuf},
    process::Stdio,
};

use miette::{Context as _, IntoDiagnostic};
use which::which_global;

use crate::util::PathExt as _;

#[derive(Default, Debug)]
pub struct Deployer {
    /// Symlinks that were successfully created
    created_symlinks: Vec<PathBuf>,
    /// Symlink creation mappings (actual_path, symlink_path) that failed due to insufficient permissions
    missing_permissions_create: Vec<(PathBuf, PathBuf)>,
    /// Symlink deletion paths (symlink_path) that failed due to insufficient permissions
    missing_permissions_remove: Vec<PathBuf>,
}

impl Deployer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn created_symlinks(&self) -> &Vec<PathBuf> {
        &self.created_symlinks
    }

    pub fn failed_creations(&self) -> &Vec<(PathBuf, PathBuf)> {
        &self.missing_permissions_create
    }

    pub fn failed_deletions(&self) -> &Vec<PathBuf> {
        &self.missing_permissions_remove
    }

    pub fn add_created_symlink(&mut self, link_path: PathBuf) {
        self.created_symlinks.push(link_path);
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
            self.missing_permissions_create
                .push((original.to_path_buf(), link.to_path_buf()));
        } else {
            self.created_symlinks.push(link.to_path_buf());
        }
        return Ok(());
    }
    /// Remove a symlink from at the path `link` pointing to the `original` file.
    pub fn delete_symlink(&mut self, path: impl AsRef<Path>) -> miette::Result<()> {
        let path = path.as_ref();
        tracing::trace!("Planning to delete symlink at {}", path.abbr());
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
                self.missing_permissions_remove.push(path.to_path_buf());
            }
            Err(e) => {
                return Err(e)
                    .into_diagnostic()
                    .wrap_err(format!("Failed to remove symlink at {}", path.abbr()))
            }
        }
        Ok(())
    }

    pub fn try_run_elevated(self) -> miette::Result<()> {
        if self.missing_permissions_create.is_empty() && self.missing_permissions_remove.is_empty()
        {
            tracing::trace!("No priviledge escalation necessary, all symlink operations succeeded");
            return Ok(());
        }
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
            .chain(
                self.missing_permissions_create
                    .iter()
                    .map(|(original, symlink)| {
                        [
                            "--create-symlink".to_string(),
                            format!("{}::::{}", original.display(), symlink.display()),
                        ]
                    })
                    .flatten(),
            )
            .chain(
                self.missing_permissions_remove
                    .iter()
                    .map(|symlink| {
                        [
                            "--delete-symlink".to_string(),
                            symlink.to_string_lossy().to_string(),
                        ]
                    })
                    .flatten(),
            )
            .collect::<Vec<_>>();
        tracing::info!(
            "Some symlink operations require root permissions: {} {}",
            if self.missing_permissions_create.is_empty() {
                "".to_string()
            } else {
                format!(
                    "create {}",
                    self.missing_permissions_create
                        .iter()
                        .map(|x| format!("{}", x.1.display()))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            },
            if self.missing_permissions_remove.is_empty() {
                "".to_string()
            } else {
                format!(
                    "delete {}",
                    self.missing_permissions_remove
                        .iter()
                        .map(|x| format!("{}", x.display()))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        );
        try_sudo(&args)?;
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
