use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::{util, yolk_paths::YolkPaths};

pub struct Yolk {
    yolk_paths: YolkPaths,
}

impl Yolk {
    pub fn new(yolk_paths: YolkPaths) -> Self {
        Self { yolk_paths }
    }

    pub fn init_yolk(&self) -> Result<()> {
        self.yolk_paths.create()?;
        Ok(())
    }

    pub fn use_thing(&self, thing_name: &str) -> Result<()> {
        let thing_path = self.yolk_paths.local_thing_path(&thing_name);

        for entry in fs_err::read_dir(&thing_path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let relative_path = entry_path.strip_prefix(&thing_path)?;
            let new_path = self.yolk_paths.home_path().join(relative_path);
            util::create_symlink_dir(entry.path(), &new_path)?;
        }
        Ok(())
    }

    pub fn add_thing(&self, name: &str, path: impl AsRef<Path>) -> Result<()> {
        let original_path = fs_err::canonicalize(path.as_ref())?;
        let Ok(relative_to_home) = original_path.strip_prefix(self.yolk_paths.home_path()) else {
            anyhow::bail!(
                "Path {} is not in the home directory {}",
                original_path.display(),
                self.yolk_paths.home_path().display()
            );
        };
        let new_local_path = self
            .yolk_paths
            .local_thing_path(name)
            .join(relative_to_home);

        let new_canonical_path = self
            .yolk_paths
            .canonical_dir_path()
            .join(name)
            .join(relative_to_home);
        fs_err::create_dir_all(new_local_path.parent().unwrap())?;
        fs_err::create_dir_all(new_canonical_path.parent().unwrap())?;
        fs_err::rename(&original_path, &new_local_path)?;
        fs_err::copy(&new_local_path, &new_canonical_path)?;
        util::create_symlink_dir(new_local_path, original_path)?;
        Ok(())
    }

    pub fn list_group_paths(&self) -> Result<Vec<PathBuf>> {
        let entries = self.yolk_paths.local_dir_path().read_dir()?;
        Ok(entries
            .filter_map(|entry| entry.ok().map(|x| x.path()))
            .collect())
    }
}

#[cfg(test)]
mod test {
    use assert_fs::{
        assert::PathAssert,
        prelude::{FileWriteStr, PathChild},
    };
    use predicates as p;
    use predicates::prelude::PredicateBooleanExt;

    use crate::yolk_paths::YolkPaths;

    use super::Yolk;

    #[test]
    fn test_stuff() {
        let home = assert_fs::TempDir::new().unwrap();
        home.child("config/foo.toml").write_str("").unwrap();
        let yp = YolkPaths::new(home.join("yolk"), home.to_path_buf());
        let yolk = Yolk::new(yp);
        yolk.init_yolk().unwrap();

        home.child("yolk/yolk.rhai").assert(p::path::is_file());
        home.child("yolk/local").assert(p::path::is_dir());

        yolk.add_thing("foo", home.join("config").join("foo.toml"))
            .unwrap();

        home.child("yolk/local/foo/config/foo.toml")
            .assert(p::path::is_file());
        home.child("yolk/canonical/foo/config/foo.toml")
            .assert(p::path::is_file());
        home.child("config/foo.toml").assert(p::path::is_symlink());

        fs_err::remove_file(home.join("config").join("foo.toml")).unwrap();
        home.child("config/foo.toml")
            .assert(p::path::exists().not());
        yolk.use_thing("foo").unwrap();
        home.child("config/foo.toml").assert(p::path::is_symlink());
    }
}
