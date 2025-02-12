use std::path::{Path, PathBuf};

use fs_err::PathExt;
use miette::{Context as _, IntoDiagnostic, Result};
use normalize_path::NormalizePath;

use crate::{
    eggs_config::EggConfig,
    git_utils::Git,
    util::{self, PathExt as _},
};

const DEFAULT_YOLK_RHAI: &str = indoc::indoc! {r#"
    export let data = #{
        for_vcs: LOCAL,
        cool_setting: if SYSTEM.hostname == "foo" { 10 } else { 25 }
    };
    export let eggs = #{
        foo: #{ targets: "~/.config/your-application", enabled: false, templates: [] }
    };
"#};

pub struct YolkPaths {
    /// Path to the yolk directory.
    root_path: PathBuf,
    home: PathBuf,
}

pub fn default_yolk_dir() -> PathBuf {
    let standard_dir = dirs::config_dir()
        .expect("No config dir available")
        .join("yolk");
    if standard_dir.exists() {
        standard_dir
    } else {
        let config_dir = if let Ok(config_dir) = std::env::var("XDG_CONFIG_DIR") {
            PathBuf::from(config_dir)
        } else {
            dirs::home_dir()
                .expect("No home dir available")
                .join(".config")
        };
        config_dir.join("yolk")
    }
}

impl YolkPaths {
    pub fn new(path: PathBuf, home: PathBuf) -> Self {
        YolkPaths {
            root_path: path,
            home: home
                .canonical()
                .expect("Failed to canonicalize home directory"),
        }
    }

    pub fn set_yolk_dir(&mut self, path: PathBuf) {
        tracing::trace!("Updating oylk-dir to {}", path.display());
        self.root_path = path;
    }
    pub fn set_home_dir(&mut self, path: PathBuf) {
        tracing::trace!("Updating home-dir to {}", path.display());
        self.home = path
            .canonical()
            .expect("Failed to canonicalize home directory");
    }

    #[allow(unused)]
    pub fn check(&self) -> Result<()> {
        miette::ensure!(
            self.root_path().exists(),
            "Yolk directory does not exist at {}",
            self.root_path().abbr()
        );
        miette::ensure!(
            self.yolk_rhai_path().exists(),
            "Yolk directory does not contain a yolk.rhai file at {}",
            self.yolk_rhai_path().abbr()
        );
        miette::ensure!(
            self.eggs_dir_path().exists(),
            "Yolk directory does not contain an eggs directory at {}",
            self.eggs_dir_path().abbr()
        );
        Ok(())
    }

    /// Safeguard git directory by renaming it to `.yolk_git`
    pub fn safeguard_git_dir(&self) -> Result<()> {
        if self.root_path().join(".git").exists() {
            miette::ensure!(
                !self.yolk_safeguarded_git_path().exists(),
                help = "Safeguarded Yolk renames .git to .yolk_git to ensure you don't accidentally commit without yolks processing",
                "Yolk directory contains both a .git directory and a .yolk_git directory"
            );
            util::rename_safely(
                self.root_path().join(".git"),
                self.yolk_safeguarded_git_path(),
            )?;
        } else {
            miette::ensure!(
                self.yolk_safeguarded_git_path().exists(),
                help = "Run yolk init first",
                "Failed to safeguard yolk git directory, as no .git directory was found"
            );
        }
        Ok(())
    }

    /// Return the path to the active git directory,
    /// which is either the [`yolk_default_git_path`] (`.git`) or the [`yolk_safeguarded_git_path`] (`.yolk_git`) if it exists.
    pub fn active_yolk_git_dir(&self) -> Result<PathBuf> {
        let default_git_dir = self.yolk_default_git_path();
        let safeguarded_git_dir = self.yolk_safeguarded_git_path();
        if safeguarded_git_dir.exists() {
            Ok(safeguarded_git_dir)
        } else if default_git_dir.exists() {
            Ok(default_git_dir)
        } else {
            miette::bail!(
                help = "Run `yolk init`, then try again!",
                "No git directory initialized"
            )
        }
    }

    pub fn create(&self) -> Result<()> {
        let path = self.root_path();
        if path.exists()
            && path.is_dir()
            && fs_err::read_dir(path).into_diagnostic()?.next().is_some()
        {
            tracing::info!("Yolk directory already exists at {}", path.abbr());
            return Ok(());
        }
        fs_err::create_dir_all(path).into_diagnostic()?;
        fs_err::create_dir_all(self.eggs_dir_path()).into_diagnostic()?;
        fs_err::write(self.yolk_rhai_path(), DEFAULT_YOLK_RHAI).into_diagnostic()?;

        Ok(())
    }

    /// Start a [Git] command helper with the paths correctly set for this yolk repository
    pub fn start_git(&self) -> Result<Git> {
        Ok(Git::new(self.root_path(), self.active_yolk_git_dir()?))
    }
    pub fn root_path(&self) -> &std::path::Path {
        &self.root_path
    }
    pub fn home_path(&self) -> &std::path::Path {
        &self.home
    }
    pub fn yolk_default_git_path(&self) -> PathBuf {
        self.root_path.join(".git")
    }
    pub fn yolk_safeguarded_git_path(&self) -> PathBuf {
        self.root_path.join(".yolk_git")
    }

    /// Path to the `yolk.rhai` file
    pub fn yolk_rhai_path(&self) -> PathBuf {
        self.root_path.join("yolk.rhai")
    }

    /// Path to the `eggs` directory
    pub fn eggs_dir_path(&self) -> PathBuf {
        self.root_path.join("eggs")
    }

    pub fn egg_path(&self, egg_name: &str) -> PathBuf {
        self.eggs_dir_path().join(egg_name)
    }

    pub fn get_egg(&self, name: &str, config: EggConfig) -> Result<Egg> {
        Egg::open(self.home.clone(), self.egg_path(name), config)
    }

    pub fn previous_egg_deployment_locations_db_path(&self) -> PathBuf {
        self.root_path.join(".previous_deployment_targets")
    }

    pub fn previous_egg_deployment_locations_db(&self) -> Result<PreviousEggDeploymentLocationsDb> {
        PreviousEggDeploymentLocationsDb::open(self.root_path.join(".deployed_cache"))
    }
}

pub struct PreviousEggDeploymentLocationsDb {
    path: PathBuf,
}

impl PreviousEggDeploymentLocationsDb {
    fn open(path: PathBuf) -> Result<Self> {
        fs_err::create_dir_all(&path).into_diagnostic()?;
        Ok(Self { path })
    }

    pub fn egg_data_path(&self, egg_name: &str) -> PathBuf {
        self.path.join(egg_name)
    }

    pub fn read(&self, egg_name: &str) -> Result<Vec<PathBuf>> {
        let cache_path = self.egg_data_path(egg_name);
        if cache_path.exists() {
            Ok(fs_err::read_to_string(cache_path)
                .into_diagnostic()?
                .lines()
                .map(PathBuf::from)
                .collect())
        } else {
            Ok(Vec::new())
        }
    }

    pub fn write(&self, egg_name: &str, symlinks: &[PathBuf]) -> Result<()> {
        let cache_path = self.egg_data_path(egg_name);
        let content = symlinks
            .iter()
            .map(|x| x.to_string_lossy())
            .collect::<Vec<_>>()
            .join("\n");
        fs_err::write(cache_path, content)
            .into_diagnostic()
            .with_context(|| format!("Failed to update egg deployment cache for egg {egg_name}"))
    }
}

#[derive(Debug)]
pub struct Egg {
    egg_dir: PathBuf,
    config: EggConfig,
    home_path: PathBuf,
}

impl Egg {
    pub fn open(home: PathBuf, egg_path: PathBuf, config: EggConfig) -> Result<Self> {
        miette::ensure!(
            egg_path.is_dir(),
            "No egg at {} does not exist",
            egg_path.abbr(),
        );
        Ok(Self {
            home_path: home.canonical()?,
            egg_dir: egg_path.canonical()?,
            config,
        })
    }

    #[allow(unused)]
    pub fn path(&self) -> &Path {
        &self.egg_dir
    }

    /// Check if the egg is _fully_ deployed (-> All contained entries have corresponding symlinks)
    #[tracing::instrument(skip_all, fields(egg.name = %self.name()))]
    pub fn is_deployed(&self) -> Result<bool> {
        if self.config.targets.is_empty() {
            return Ok(false);
        }
        for x in self.find_deployed_symlinks()? {
            if x.context("Got error while iterating through deployed files or egg")?
                .is_err()
            {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn name(&self) -> &str {
        self.egg_dir
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
    }

    /// Iterate over the deployed symlinks of this egg.
    ///
    /// See [`TraverseDeployment`] for more information.
    #[tracing::instrument(skip_all)]
    pub fn find_deployed_symlinks(&self) -> Result<TraverseDeployment> {
        let targets = self
            .config
            .targets_expanded(&self.home_path, self.path())
            .context("Failed to expand targets map")?;
        Ok(TraverseDeployment::new(targets))
    }

    /// Find the first deployed symlink of a deployment.
    /// Note that this is not sufficient to check if the egg is fully deployed.
    #[tracing::instrument(skip_all)]
    pub fn find_first_deployed_symlink(&self) -> Result<Option<PathBuf>> {
        match self.find_deployed_symlinks()?.next() {
            Some(Ok(Ok(x))) => Ok(Some(x)),
            Some(Ok(Err(_))) => Ok(None),
            Some(Err(x)) => Err(x),
            None => Ok(None),
        }
    }

    pub fn config(&self) -> &EggConfig {
        &self.config
    }

    /// Get a mutable reference to the egg configuration. Deliberately only available for tests.
    #[cfg(test)]
    pub fn config_mut(&mut self) -> &mut EggConfig {
        &mut self.config
    }
}

/// An iterator that traverses a deployed egg and yields paths to all symlinks of the deployment.
///
/// Returns
/// - `Ok(Ok(path))` for a symlink that is correctly deployed,
/// - `Ok(Err(path_in_egg))` for a path inside an egg that does not have a corresponding deployed symlink
/// - `Err(err)` if there is an error
/// - `None` if the traversal is finished
pub struct TraverseDeployment {
    stack: Vec<(PathBuf, PathBuf)>,
}
impl TraverseDeployment {
    fn new(stack: impl IntoIterator<Item = (PathBuf, PathBuf)>) -> Self {
        let stack: Vec<_> = stack.into_iter().collect();
        Self { stack }
    }
}

impl Iterator for TraverseDeployment {
    type Item = miette::Result<Result<PathBuf, PathBuf>>;
    fn next(&mut self) -> Option<miette::Result<Result<PathBuf, PathBuf>>> {
        let (in_egg, link) = self.stack.pop()?;
        let in_egg = in_egg.normalize();
        let link = link.normalize();

        tracing::trace!(
            stack = ?self
                .stack
                .iter()
                .map(|x| (x.0.abbr(), x.1.abbr()))
                .collect::<Vec<_>>(),
            "checking for deployment {} -> {}.",
            link.abbr(),
            in_egg.abbr(),
        );

        if link.is_symlink() {
            match (
                in_egg.canonical(),
                link.fs_err_read_link().into_diagnostic(),
            ) {
                (Ok(in_egg), Ok(link)) if in_egg.normalize() == link.normalize() => {
                    Some(Ok(Ok(link)))
                }
                (Ok(in_egg), Ok(_)) => Some(Ok(Err(in_egg))),
                (Err(e), _) | (_, Err(e)) => Some(Err(e)),
            }
        } else if link.is_dir() && in_egg.is_dir() {
            cov_mark::hit!(traverse_dir_recursive);
            for in_egg_entry in fs_err::read_dir(&in_egg).ok()? {
                let in_egg_entry = match in_egg_entry {
                    Ok(x) => x,
                    Err(e) => return Some(Err(miette::miette!(e))),
                };
                let link_entry = link.join(in_egg_entry.file_name());
                self.stack.push((in_egg_entry.path(), link_entry));
            }
            return self.next();
        } else {
            return Some(Ok(Err(in_egg)));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        deploy::remove_symlink,
        util::test_util::{setup_and_init_test_yolk, TestResult},
        yolk_paths::{Egg, DEFAULT_YOLK_RHAI},
    };
    use assert_fs::{
        assert::PathAssert,
        prelude::{FileWriteStr, PathChild, PathCreateDir},
        TempDir,
    };
    use miette::IntoDiagnostic;
    use predicates::path::exists;
    use test_log::test;

    use crate::eggs_config::EggConfig;

    use super::YolkPaths;

    #[test]
    pub fn test_setup() {
        let root = assert_fs::TempDir::new().unwrap();
        let yolk_paths = YolkPaths::new(root.child("yolk").to_path_buf(), root.to_path_buf());
        assert!(yolk_paths.check().is_err());
        yolk_paths.create().unwrap();
        assert!(yolk_paths.check().is_ok());
        root.child("yolk/eggs").assert(exists());
        root.child("yolk/yolk.rhai").assert(DEFAULT_YOLK_RHAI);
    }

    #[test]
    pub fn test_is_deployed_2() -> TestResult {
        cov_mark::check_count!(traverse_dir_recursive, 0);
        let (home, yolk, eggs) = setup_and_init_test_yolk()?;
        eggs.child("foo/foo.toml").write_str("")?;
        eggs.child("foo/thing/thing.toml").write_str("")?;
        let egg = Egg::open(
            home.to_path_buf(),
            eggs.child("foo").to_path_buf(),
            EggConfig::default().with_target("foo.toml", home.child("foo.toml")),
        )?;
        yolk.sync_egg_deployment(&egg)?;
        assert!(egg.is_deployed()?);
        Ok(())
    }

    #[test]
    pub fn test_is_deployed_single_dir() -> TestResult {
        cov_mark::check_count!(traverse_dir_recursive, 0);
        let (home, yolk, eggs) = setup_and_init_test_yolk()?;

        let test_egg_dir = eggs.child("test_egg");
        test_egg_dir.child("foo").create_dir_all()?;
        test_egg_dir.child("foo/bar").write_str("")?;
        let egg = Egg::open(
            home.to_path_buf(),
            test_egg_dir.to_path_buf(),
            EggConfig::new_merge(".", home.child("target")),
        )?;
        assert!(!(egg.is_deployed()?));
        yolk.sync_egg_deployment(&egg)?;
        assert!(egg.is_deployed()?);
        remove_symlink(home.child("target"))?;
        assert!(!(egg.is_deployed()?));
        Ok(())
    }

    #[test]
    pub fn test_is_deployed() -> TestResult {
        cov_mark::check!(traverse_dir_recursive);
        let (home, yolk, eggs) = setup_and_init_test_yolk()?;
        let test_egg_dir = eggs.child("test_egg");

        home.child("content/dir_old").create_dir_all()?;
        home.child("content/dir_old/file_old").write_str("")?;
        test_egg_dir.child("content/file").write_str("")?;
        test_egg_dir.child("content/dir1").create_dir_all()?;
        test_egg_dir.child("content/dir2/dir1").create_dir_all()?;
        test_egg_dir.child("content/dir2/file1").write_str("")?;
        test_egg_dir.child("content/dir_old/file1").write_str("")?;
        test_egg_dir.child("content/dir_old/dir1").write_str("")?;
        test_egg_dir.child("content/dir3").create_dir_all()?;
        test_egg_dir.child("content/dir3/file1").write_str("")?;
        test_egg_dir.child("content/dir4/dir1").create_dir_all()?;

        let egg = Egg::open(
            home.to_path_buf(),
            test_egg_dir.to_path_buf(),
            EggConfig::new_merge(".", &home),
        )?;
        assert!(!(egg.is_deployed()?));
        yolk.sync_egg_deployment(&egg)?;
        assert!(egg.is_deployed()?);
        fs_err::remove_file(home.child("content/dir_old/file1"))?;
        assert!(!(egg.is_deployed()?));
        Ok(())
    }

    #[test]
    pub fn test_default_script() -> TestResult {
        let root = TempDir::new().into_diagnostic()?;
        let yolk_paths = YolkPaths::new(root.child("yolk").to_path_buf(), root.to_path_buf());
        yolk_paths.create().unwrap();
        let yolk = crate::yolk::Yolk::new(yolk_paths);
        _ = yolk.prepare_eval_ctx_for_templates(crate::yolk::EvalMode::Local)?;
        Ok(())
    }
}
