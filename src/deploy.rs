use std::{
    path::{Path, PathBuf},
    process::Stdio,
};

use fs_err::PathExt as _;
use miette::{Context as _, IntoDiagnostic};
use normalize_path::NormalizePath as _;
use which::which_global;

use crate::util::PathExt as _;

/// Struct that keeps track of the deployment and undeployment process of multiple symlinks.
///
/// We keep track of all created symlinks, as well as all symlinks where the creation or deletion failed due to insufficient permissions.
/// In case of missing permissions, you can then use [`Deployer::try_run_elevated()`] to retry the operation with elevated privileges.
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
                        "Failed to create symlink at {}",
                        format_symlink(link.abbr(), original.abbr())
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
        tracing::trace!("Deleting symlink at {}", path.abbr());
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

    /// Set up a symlink from the given `link_path` to the given `actual_path`, recursively.
    /// Also takes the `egg_root` dir, to ensure we can safely delete any stale symlinks on the way there.
    ///
    /// Requires all paths to be absolute, will panic otherwise.
    ///
    /// This means:
    /// - If `link_path` exists and is a file, abort
    /// - If `link_path` exists and is a symlink into the egg dir, remove the symlink and then continue.
    /// - If `actual_path` is a file, symlink.
    /// - If `actual_path` is a directory that does not exist in `link_path`, symlink it.
    /// - If `actual_path` is a directory that already exists in `link_path`, recurse into it and `symlink_recursive` `actual_path`s children.
    #[tracing::instrument(skip_all, fields(
        egg_root = egg_root.as_ref().abbr(),
        actual_path = actual_path.as_ref().abbr(),
        link_path = link_path.as_ref().abbr()
    ))]
    pub fn symlink_recursive(
        &mut self,
        egg_root: impl AsRef<Path>,
        actual_path: impl AsRef<Path>,
        link_path: &impl AsRef<Path>,
    ) -> miette::Result<()> {
        fn inner(
            deployer: &mut Deployer,
            egg_root: PathBuf,
            actual_path: PathBuf,
            link_path: PathBuf,
        ) -> miette::Result<()> {
            let actual_path = actual_path.normalize();
            let link_path = link_path.normalize();
            let egg_root = egg_root.normalize();
            link_path.assert_absolute("link_path");
            actual_path.assert_absolute("actual_path");
            actual_path.assert_starts_with(&egg_root, "actual_path");
            tracing::trace!(
                "symlink_recursive({}, {})",
                actual_path.abbr(),
                link_path.abbr()
            );

            let actual_path = actual_path.canonical()?;

            if link_path.is_symlink() {
                let link_target = link_path.fs_err_read_link().into_diagnostic()?;
                if link_target == actual_path {
                    deployer.add_created_symlink(link_path);
                    return Ok(());
                } else if link_target.exists() {
                    miette::bail!(
                        "Failed to create symlink {}, as a file already exists there",
                        format_symlink(link_path.abbr(), actual_path.abbr())
                    );
                } else if link_target.starts_with(&egg_root) {
                    tracing::info!(
                        "Removing dead symlink {}",
                        format_symlink(link_path.abbr(), link_target.abbr())
                    );
                    deployer.delete_symlink(&link_path)?;
                    cov_mark::hit!(remove_dead_symlink);
                    // After we've removed that file, creating the symlink later will succeed!
                } else {
                    miette::bail!(
                        "Encountered dead symlink, but it doesn't target the egg dir: {}",
                        link_path.abbr(),
                    );
                }
            } else if link_path.exists() {
                tracing::trace!("link_path exists as non-symlink {}", link_path.abbr());
                if link_path.is_dir() && actual_path.is_dir() {
                    for entry in actual_path.fs_err_read_dir().into_diagnostic()? {
                        let entry = entry.into_diagnostic()?;
                        deployer.symlink_recursive(
                            &egg_root,
                            entry.path(),
                            &link_path.join(entry.file_name()),
                        )?;
                    }
                    return Ok(());
                } else if link_path.is_dir() || actual_path.is_dir() {
                    miette::bail!(
                        "Conflicting file or directory {} with {}",
                        actual_path.abbr(),
                        link_path.abbr()
                    );
                }
            }
            deployer.create_symlink(&actual_path, &link_path)?;
            tracing::info!(
                "created symlink {}",
                format_symlink(link_path.abbr(), actual_path.abbr()),
            );
            Ok(())
        }
        inner(
            self,
            egg_root.as_ref().to_path_buf(),
            actual_path.as_ref().to_path_buf(),
            link_path.as_ref().to_path_buf(),
        )
    }

    #[tracing::instrument(skip(actual_path, link_path), fields(
        actual_path = actual_path.as_ref().abbr(),
        link_path = link_path.as_ref().abbr()
    ))]
    pub fn remove_symlink_recursive(
        &mut self,
        actual_path: impl AsRef<Path>,
        link_path: &impl AsRef<Path>,
    ) -> miette::Result<()> {
        let actual_path = actual_path.as_ref();
        let link_path = link_path.as_ref();
        if link_path.is_symlink() && link_path.canonical()? == actual_path {
            tracing::info!(
                "Removing symlink {}",
                format_symlink(link_path.abbr(), actual_path.abbr())
            );
            self.delete_symlink(link_path)?;
        } else if link_path.is_dir() && actual_path.is_dir() {
            for entry in actual_path.fs_err_read_dir().into_diagnostic()? {
                let entry = entry.into_diagnostic()?;
                self.remove_symlink_recursive(entry.path(), &link_path.join(entry.file_name()))?;
            }
        } else if link_path.exists() {
            miette::bail!(
                help = "Yolk will only try to remove files that are symlinks pointing into the corresponding egg.",
                "Tried to remove deployment of {}, but {} doesn't link to it",
                actual_path.abbr(),
                link_path.abbr()
            );
        }
        Ok(())
    }

    /// Retry running symlink creation and deletion with root priviledges.
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
                "Failed to create symlink at {}",
                format_symlink(link.abbr(), original.abbr())
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

fn format_symlink(link_path: impl AsRef<Path>, original_path: impl AsRef<Path>) -> String {
    format!(
        "{} -> {}",
        link_path.as_ref().display(),
        original_path.as_ref().display()
    )
}
