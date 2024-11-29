use std::path::PathBuf;

use anyhow::Result;

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
            anyhow::bail!(
                "Yolk directory does not exist at {}",
                self.root_path().display()
            );
        }
        if !self.script_path().exists() {
            anyhow::bail!(
                "Yolk directory does not contain a yolk.lua file at {}",
                self.script_path().display()
            );
        }
        if !self.eggs_dir_path().exists() {
            anyhow::bail!(
                "Yolk directory does not contain an eggs directory at {}",
                self.eggs_dir_path().display()
            );
        }
        Ok(())
    }

    pub fn create(&self) -> Result<()> {
        let path = self.root_path();
        if path.exists() && path.is_dir() && fs_err::read_dir(path)?.next().is_some() {
            anyhow::bail!("Yolk directory already exists at {}", path.display());
        }
        fs_err::create_dir_all(path)?;
        fs_err::create_dir_all(self.eggs_dir_path())?;
        fs_err::write(self.script_path(), DEFAULT_LUA)?;

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
    pub fn yolk_templates_file_path_for(&self, egg_name: &str) -> std::path::PathBuf {
        // TODO: Do we like this being next to regular directories, and just being treated magically based on the name?
        self.egg_path(egg_name).join("yolk_templates")
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
