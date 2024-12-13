use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use fs_err::PathExt;
use miette::{IntoDiagnostic, Result};

use crate::util::PathExt as _;

const DEFAULT_LUA: &str = indoc::indoc! {r#"
    data = {
        generating_for_vcs = not LOCAL,
        cool_setting = if SYSTEM.hostname == "foo" then
            10
        else
            25
    }
"#};

const DEFAULT_EGGS_LUA: &str = indoc::indoc! {r#"
    eggs = {}
    return eggs
"#};

const DEFAULT_GITIGNORE: &str = indoc::indoc! {r#"
    # Ignore the yolk git directory
    /.yolk_git
"#};

pub struct YolkPaths {
    /// Path to the yolk directory.
    root_path: PathBuf,
    home: PathBuf,
}

pub fn default_yolk_dir() -> PathBuf {
    dirs::config_dir()
        .expect("No config dir available")
        .join("yolk")
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

    pub fn from_env() -> Self {
        Self::new(
            default_yolk_dir(),
            dirs::home_dir().expect("No home dir available"),
        )
    }

    pub fn set_yolk_dir(&mut self, path: PathBuf) {
        self.root_path = path;
    }
    pub fn set_home_dir(&mut self, path: PathBuf) {
        self.home = path
            .canonical()
            .expect("Failed to canonicalize home directory");
    }

    #[allow(unused)]
    pub fn check(&self) -> Result<()> {
        if !self.root_path().exists() {
            miette::bail!(
                "Yolk directory does not exist at {}",
                self.root_path().display()
            );
        }
        if !self.yolk_lua_path().exists() {
            miette::bail!(
                "Yolk directory does not contain a yolk.lua file at {}",
                self.yolk_lua_path().display()
            );
        }
        if !self.eggs_dir_path().exists() {
            miette::bail!(
                "Yolk directory does not contain an eggs directory at {}",
                self.eggs_dir_path().display()
            );
        }
        Ok(())
    }

    pub fn create(&self) -> Result<()> {
        let path = self.root_path();
        if path.exists()
            && path.is_dir()
            && fs_err::read_dir(path).into_diagnostic()?.next().is_some()
        {
            miette::bail!("Yolk directory already exists at {}", path.display());
        }
        fs_err::create_dir_all(path).into_diagnostic()?;
        fs_err::create_dir_all(self.eggs_dir_path()).into_diagnostic()?;
        fs_err::write(self.root_path().join(".gitignore"), DEFAULT_GITIGNORE).into_diagnostic()?;
        fs_err::write(self.yolk_lua_path(), DEFAULT_LUA).into_diagnostic()?;
        fs_err::write(self.eggs_lua_path(), DEFAULT_EGGS_LUA).into_diagnostic()?;

        Ok(())
    }

    /// Safeguard git directory by renaming it to `.yolk_git`
    pub fn safeguard_git_dir(&self) -> Result<()> {
        if self.root_path().join(".git").exists() {
            if self.yolk_safeguarded_git_path().exists() {
                miette::bail!(
                    help = "Safeguarded Yolk renames .git to .yolk_git to ensure you don't accidentally commit without yolks processing",
                    "Yolk directory contains both a .git directory and a .yolk_git directory"
                );
            } else {
                fs_err::rename(
                    self.root_path().join(".git"),
                    self.yolk_safeguarded_git_path(),
                )
                .into_diagnostic()?;
            }
        }
        Ok(())
    }

    /// Start an invocation of the `git` command with the `--git-dir` and `--work-tree` set to the yolk git and root path.
    pub fn start_git_command_builder(&self) -> Result<std::process::Command> {
        let mut cmd = std::process::Command::new("git");
        cmd.current_dir(self.root_path()).args([
            "--git-dir",
            &self.active_yolk_git_dir()?.to_string_lossy(),
            "--work-tree",
            &self.root_path().to_string_lossy(),
        ]);
        Ok(cmd)
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
            miette::bail!("No git directory initialized")
        }
    }
    ///
    /// Path to the `yolk.lua` file
    pub fn yolk_lua_path(&self) -> PathBuf {
        self.root_path.join("yolk.lua")
    }

    /// Path to the `eggs.lua` file
    pub fn eggs_lua_path(&self) -> PathBuf {
        self.root_path.join("eggs.lua")
    }

    /// Path to the `eggs` directory
    pub fn eggs_dir_path(&self) -> PathBuf {
        self.root_path.join("eggs")
    }

    pub fn egg_path(&self, egg_name: &str) -> PathBuf {
        self.eggs_dir_path().join(egg_name)
    }

    pub fn get_egg(&self, name: &str) -> Result<Egg> {
        Egg::open(self.home.clone(), self.egg_path(name))
    }

    pub fn list_eggs(&self) -> Result<impl Iterator<Item = Result<Egg>> + '_> {
        let entries = self.eggs_dir_path().fs_err_read_dir().into_diagnostic()?;
        Ok(entries.filter_map(|entry| {
            entry
                .ok()
                .map(|x| Egg::open(self.home_path().to_path_buf(), x.path()))
        }))
    }
}

pub struct Egg {
    egg_dir: PathBuf,
    home_path: PathBuf,
}

impl Egg {
    pub fn open(home: PathBuf, egg_path: PathBuf) -> Result<Self> {
        if !egg_path.is_dir() {
            miette::bail!(
                "Egg {} does not exist",
                egg_path
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
            )
        }
        Ok(Self {
            home_path: home.canonical()?,
            egg_dir: egg_path.canonical()?,
        })
    }

    #[allow(unused)]
    pub fn path(&self) -> &Path {
        &self.egg_dir
    }

    /// Check if the egg is _fully_ deployed (-> All contained entries have corresponding symlinks)
    pub fn is_deployed(&self) -> Result<bool> {
        for entry in self.entries()? {
            if !check_is_deployed_recursive(&self.home_path, &self.egg_dir, entry.path())? {
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

    /// Returns a list of all entries in this egg,
    /// meaning all files and directories in the egg dir except for the yolk_templates file.
    pub fn entries(&self) -> Result<Vec<fs_err::DirEntry>> {
        let mut entries = Vec::new();
        for entry in self.egg_dir.fs_err_read_dir().into_diagnostic()? {
            let entry = entry.into_diagnostic()?;
            if entry.file_name() == "yolk_templates" {
                continue;
            }
            entries.push(entry)
        }
        Ok(entries)
    }

    /// Returns a list of all the template paths in this egg in canonical form.
    pub fn template_paths(&self) -> Result<HashSet<PathBuf>> {
        let tmpl_list_file = self.egg_dir.join("yolk_templates");
        if !tmpl_list_file.is_file() {
            return Ok(HashSet::new());
        }
        let tmpl_paths = fs_err::read_to_string(tmpl_list_file).into_diagnostic()?;
        let tmpl_paths = tmpl_paths
            .lines()
            .map(|x| self.egg_dir.join(x))
            .filter(|x| x.exists()) // TODO: emit some warning for inexistant files in yolk_templates file
            .map(|x| x.canonical())
            .collect::<Result<_>>()?;
        Ok(tmpl_paths)
    }

    pub fn find_first_targetting_symlink(&self) -> Result<Option<PathBuf>> {
        find_first_deployed_symlink_recursive(&self.home_path, &self.egg_dir, &self.egg_dir)
    }
}

/// Basically the same as `check_is_deployed_recursive`, but it returns the first symlink that is found,
/// rather than checking for all of them to exist.
// TODO: Clean this up and combine this with `check_is_deployed_recursive` somehow
fn find_first_deployed_symlink_recursive(
    target_root: impl AsRef<Path>,
    egg_root: impl AsRef<Path>,
    current: impl AsRef<Path>,
) -> Result<Option<PathBuf>> {
    let target_root = target_root.as_ref();
    let egg_root = egg_root.as_ref();
    let current = current.as_ref();
    let target_file = target_root.join(current.strip_prefix(egg_root).into_diagnostic()?);
    if target_file.is_symlink() && target_file.canonical()? == current {
        Ok(Some(target_file))
    } else if target_file.is_file() {
        Ok(None)
    } else if target_file.is_dir() {
        for entry in fs_err::read_dir(current).into_diagnostic()? {
            let entry = entry.into_diagnostic()?;
            if let Some(file) =
                find_first_deployed_symlink_recursive(target_root, egg_root, entry.path())?
            {
                return Ok(Some(file));
            }
        }
        Ok(None)
    } else {
        Ok(None)
    }
}

fn check_is_deployed_recursive(
    target_root: impl AsRef<Path>,
    egg_root: impl AsRef<Path>,
    current: impl AsRef<Path>,
) -> Result<bool> {
    let target_root = target_root.as_ref();
    let egg_root = egg_root.as_ref();
    let current = current.as_ref();
    let target_file = target_root.join(current.strip_prefix(egg_root).into_diagnostic()?);
    if target_file.is_symlink() && target_file.canonical()? == current {
        Ok(true)
    } else if target_file.is_file() {
        Ok(false)
    } else if target_file.is_dir() {
        for entry in fs_err::read_dir(current).into_diagnostic()? {
            let entry = entry.into_diagnostic()?;
            if !check_is_deployed_recursive(target_root, egg_root, entry.path())? {
                return Ok(false);
            }
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod test {
    use assert_fs::{
        assert::PathAssert,
        prelude::{FileWriteStr, PathChild, PathCreateDir},
    };
    use predicates::{path::exists, prelude::PredicateBooleanExt};
    use testresult::TestResult;

    use crate::{eggs_config::EggConfig, yolk::Yolk};

    use super::YolkPaths;

    #[test]
    pub fn test_setup() {
        let root = assert_fs::TempDir::new().unwrap();
        let yolk_paths = YolkPaths::new(root.child("yolk").to_path_buf(), root.to_path_buf());
        assert!(yolk_paths.check().is_err());
        yolk_paths.create().unwrap();
        assert!(yolk_paths.check().is_ok());
        root.child("yolk/eggs").assert(exists());
        root.child("yolk/yolk.lua").assert(exists());
    }

    #[test]
    pub fn test_is_deployed() -> TestResult {
        let home = assert_fs::TempDir::new().unwrap();
        let yolk_paths = YolkPaths::new(home.child("yolk").to_path_buf(), home.to_path_buf());
        yolk_paths.create()?;
        let yolk = Yolk::new(yolk_paths);

        home.child("content/dir_old").create_dir_all()?;
        home.child("content/dir_old/file_old").write_str("")?;
        let test_egg_dir = home.child("yolk/eggs/test_egg");
        test_egg_dir.child("content/file").write_str("")?;
        test_egg_dir.child("content/dir1").create_dir_all()?;
        test_egg_dir.child("content/dir2/dir1").create_dir_all()?;
        test_egg_dir.child("content/dir2/file1").write_str("")?;
        test_egg_dir.child("content/dir_old/file1").write_str("")?;
        test_egg_dir.child("content/dir_old/dir1").write_str("")?;
        test_egg_dir.child("content/dir3").create_dir_all()?;
        test_egg_dir.child("content/dir3/file1").write_str("")?;
        test_egg_dir.child("content/dir4/dir1").create_dir_all()?;
        let egg = yolk.paths().get_egg("test_egg")?;

        assert!(!(egg.is_deployed()?));
        yolk.deploy_egg("test_egg", &EggConfig::new(".", &home))?;
        assert!(egg.is_deployed()?);
        fs_err::remove_file(home.child("content/dir_old/file1"))?;
        assert!(!(egg.is_deployed()?));

        Ok(())
    }

    #[test]
    pub fn test_safeguard() -> TestResult {
        let root = assert_fs::TempDir::new().unwrap();
        let yolk_paths = YolkPaths::new(root.child("yolk").to_path_buf(), root.to_path_buf());
        yolk_paths.create()?;
        let yolk = Yolk::new(yolk_paths);
        root.child("yolk/.git").create_dir_all()?;
        yolk.paths().safeguard_git_dir()?;
        root.child("yolk/.git").assert(exists().not());
        root.child("yolk/.yolk_git").assert(exists());
        Ok(())
    }

    // #[test]
    // pub fn test_get_templated_files() -> TestResult {
    //     let root = assert_fs::TempDir::new().unwrap();
    //     let yolk_paths = YolkPaths::new(root.child("yolk").to_path_buf(), root.to_path_buf());
    //     yolk_paths.create()?;
    //     todo!("Write test");
    //     let yolk = Yolk::new(yolk_paths);
    //     root.child("foo/file").write_str("foo")?;
    //     // yolk.add_to_egg("foo", root.child("foo"))?;
    //     // yolk.add_to_templated_files(&[root.child("foo/file")])?;
    //     let egg = yolk.paths().get_egg("foo")?;
    //     assert_eq!(
    //         vec![root.child("foo/file").to_path_buf().canonical()?],
    //         egg.template_paths()?.into_iter().collect::<Vec<_>>()
    //     );
    //     fs_err::remove_file(root.child("foo/file"))?;
    //     assert_eq!(
    //         Vec::<PathBuf>::new(),
    //         egg.template_paths()?.into_iter().collect::<Vec<_>>()
    //     );

    //     Ok(())
    // }
}
