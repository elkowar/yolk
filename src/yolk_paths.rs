use std::path::{Path, PathBuf};

use fs_err::PathExt as _;
use miette::{IntoDiagnostic as _, Result};

const DEFAULT_LUA: &str = indoc::indoc! {r#"
    function canonical_data()
        return {}
    end
    function local_data(system)
        canonical_data()
    end
"#};

pub struct YolkPaths {
    /// Path to the yolk directory.
    root_path: std::path::PathBuf,
    home: std::path::PathBuf,
}

impl YolkPaths {
    pub fn new(path: std::path::PathBuf, home: std::path::PathBuf) -> Self {
        YolkPaths {
            root_path: path,
            home,
        }
    }

    #[allow(unused)]
    pub fn testing() -> Self {
        let base_dir = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("test_home");
        Self::new(base_dir.join("yolk"), base_dir)
    }

    pub fn from_env_with_root(root_path: PathBuf) -> Self {
        Self {
            root_path,
            home: dirs::home_dir().expect("No config dir available"),
        }
    }

    pub fn from_env() -> Self {
        Self {
            root_path: dirs::config_dir()
                .expect("No config dir available")
                .join("yolk"),
            home: dirs::home_dir().expect("No config dir available"),
        }
    }

    #[allow(unused)]
    pub fn check(&self) -> Result<()> {
        if !self.root_path().exists() {
            miette::bail!(
                "Yolk directory does not exist at {}",
                self.root_path().display()
            );
        }
        if !self.script_path().exists() {
            miette::bail!(
                "Yolk directory does not contain a yolk.lua file at {}",
                self.script_path().display()
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
        fs_err::write(self.script_path(), DEFAULT_LUA).into_diagnostic()?;

        Ok(())
    }

    pub fn root_path(&self) -> &std::path::Path {
        &self.root_path
    }
    pub fn home_path(&self) -> &std::path::Path {
        &self.home
    }
    pub fn script_path(&self) -> std::path::PathBuf {
        self.root_path.join("yolk.lua")
    }
    pub fn eggs_dir_path(&self) -> std::path::PathBuf {
        self.root_path.join("eggs")
    }
    pub fn egg_path(&self, egg_name: &str) -> std::path::PathBuf {
        self.eggs_dir_path().join(egg_name)
    }

    pub fn get_egg(&self, name: &str) -> Result<Egg> {
        Egg::open(self.egg_path(name))
    }
    pub fn get_or_create_egg(&self, name: &str) -> Result<Egg> {
        let egg_path = self.egg_path(name);
        if !egg_path.exists() {
            fs_err::create_dir_all(&egg_path).into_diagnostic()?;
        }
        Egg::open(egg_path)
    }
}

pub struct Egg {
    egg_dir: PathBuf,
}

impl Egg {
    pub fn open(egg_path: PathBuf) -> Result<Self> {
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
        Ok(Egg { egg_dir: egg_path })
    }

    #[allow(unused)]
    pub fn path(&self) -> &Path {
        &self.egg_dir
    }

    pub fn templates_path(&self) -> PathBuf {
        self.egg_dir.join("yolk_templates")
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
    pub fn template_paths(&self) -> Result<Vec<PathBuf>> {
        let tmpl_list_file = self.egg_dir.join("yolk_templates");
        if !tmpl_list_file.is_file() {
            return Ok(vec![]);
        }
        let tmpl_paths = fs_err::read_to_string(tmpl_list_file).into_diagnostic()?;
        let tmpl_paths = tmpl_paths
            .lines()
            .map(|x| {
                Ok(self
                    .egg_dir
                    .join(x)
                    .fs_err_canonicalize()
                    .into_diagnostic()?)
            })
            .collect::<Result<_>>()?;
        Ok(tmpl_paths)
    }

    pub fn add_to_template_paths(&self, paths: &[PathBuf]) -> Result<()> {
        let yolk_templates_path = self.templates_path();
        if !yolk_templates_path.is_file() {
            fs_err::File::create(&yolk_templates_path).into_diagnostic()?;
        }
        let yolk_templates = fs_err::read_to_string(&yolk_templates_path).into_diagnostic()?;
        let mut yolk_templates: Vec<_> = yolk_templates.lines().map(|x| x.to_string()).collect();
        for path in paths {
            if !path.exists() {
                eprintln!("Warning: {} does not exist, skipping.", path.display());
                continue;
            }
            let path = path.fs_err_canonicalize().into_diagnostic()?;
            if !path.starts_with(&self.egg_dir) {
                return Err(miette::miette!(
                    "The given file path is not within {}",
                    self.egg_dir.display()
                ));
            }
            let path_relative = path.strip_prefix(&self.egg_dir).into_diagnostic()?;
            let path_str = path_relative.to_str().unwrap().to_string();
            yolk_templates.push(path_str);
        }
        fs_err::write(&yolk_templates_path, yolk_templates.join("\n")).into_diagnostic()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use assert_fs::{assert::PathAssert, prelude::PathChild};
    use predicates::path::exists;

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
}
