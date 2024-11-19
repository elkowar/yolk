use std::path::Path;

use anyhow::Result;

const DEFAULT_RHAI: &str = indoc::indoc! {r#"
    fn canonical_data() {
        #{}
    }
    fn local_data(machine_name) {
        canonical_data()
    }
"#};

pub struct YolkDots {
    /// Path to the yolk directory.
    path: std::path::PathBuf,
}

impl YolkDots {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let dots = YolkDots {
            path: path.as_ref().to_path_buf(),
        };
        if !dots.root_path().exists() {
            anyhow::bail!(
                "Yolk directory does not exist at {}",
                dots.root_path().display()
            );
        }
        if !dots.rhai_path().exists() {
            anyhow::bail!(
                "Yolk directory does not contain a .rhai file at {}",
                dots.rhai_path().display()
            );
        }
        if !dots.local_dir_path().exists() {
            anyhow::bail!(
                "Yolk directory does not contain a local directory at {}",
                dots.local_dir_path().display()
            );
        }
        if !dots.canonical_dir_path().exists() {
            anyhow::bail!(
                "Yolk directory does not contain a canonical directory at {}",
                dots.canonical_dir_path().display()
            );
        }
        Ok(dots)
    }
    pub fn create(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if path.exists() && path.is_dir() && path.read_dir()?.next().is_some() {
            anyhow::bail!("Yolk directory already exists at {}", path.display());
        }
        std::fs::create_dir_all(&path)?;
        let dots = YolkDots { path };
        std::fs::create_dir_all(&dots.local_dir_path())?;
        std::fs::create_dir_all(&dots.canonical_dir_path())?;
        std::fs::write(dots.root_path().join(".gitignore"), "/local")?;
        std::fs::write(dots.rhai_path(), DEFAULT_RHAI)?;

        Ok(dots)
    }

    pub fn root_path(&self) -> &std::path::Path {
        &self.path
    }
    pub fn rhai_path(&self) -> std::path::PathBuf {
        self.path.join("yolk.rhai")
    }
    pub fn local_dir_path(&self) -> std::path::PathBuf {
        self.path.join("local")
    }
    pub fn canonical_dir_path(&self) -> std::path::PathBuf {
        self.path.join("local")
    }
}

#[cfg(test)]
mod test {

    use tempdir::TempDir;

    use super::YolkDots;

    #[test]
    pub fn test_setup() {
        let root = TempDir::new("yolk-setup").unwrap();
        let root_path = root.path();
        let dots = YolkDots::create(&root).unwrap();
        let dots = YolkDots::load(&root).unwrap();
    }
}
