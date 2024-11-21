use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context as _, Result};
use rhai::Dynamic;

use crate::{
    eval_ctx::{self, EvalCtx, SystemInfo},
    templating::document::Document,
    util,
    yolk_paths::YolkPaths,
};

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
    pub fn paths(&self) -> &YolkPaths {
        &self.yolk_paths
    }

    pub fn use_thing(&self, thing_name: &str) -> Result<()> {
        let thing_path = self.yolk_paths.local_thing_path(&thing_name);

        for entry in fs_err::read_dir(&thing_path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let relative_path = entry_path.strip_prefix(&thing_path)?;
            let new_path = self.yolk_paths.home_path().join(relative_path);
            if !new_path.exists() {
                util::create_symlink_dir(entry.path(), &new_path)?;
            } else {
                println!(
                    "Warning: file {} already exists, skipping...",
                    new_path.display()
                );
            }
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
        fs_err::create_dir_all(new_local_path.parent().unwrap())?;
        fs_err::rename(&original_path, &new_local_path)?;
        util::create_symlink_dir(new_local_path, original_path)?;
        Ok(())
    }

    pub fn sync(&self) -> Result<()> {
        let thing_paths = self.list_thing_paths()?;
        for thing_dir in thing_paths {
            let tmpl_list_file = thing_dir.join("yolk_templates");
            if tmpl_list_file.is_file() {
                let thing_canonical = thing_dir.canonicalize()?;
                let tmpl_paths = std::fs::read_to_string(tmpl_list_file)?;
                let tmpl_paths = tmpl_paths.lines().map(|x| thing_canonical.join(x));
                for templated_file in tmpl_paths {
                    if templated_file.is_file() {
                        self.sync_file(EvalMode::Local, &templated_file)
                            .with_context(|| {
                                format!("Failed to sync file {}", templated_file.display())
                            })?;
                    }
                    tracing::warn!(
                        "{} was specified as templated file, but doesn't exist",
                        templated_file.display()
                    );
                }
            }
        }
        Ok(())
    }

    pub fn prepare_eval_ctx(&self, mode: EvalMode, engine: &rhai::Engine) -> Result<EvalCtx> {
        let mut eval_ctx = EvalCtx::new(SystemInfo::generate());
        // TODO: deal with errors better
        let ast = engine
            .compile_file(self.yolk_paths.rhai_path())
            .map_err(|err| anyhow!("Failed to compile rhai file: {}", err.to_string()))?;
        let data: Result<Dynamic, _> = match mode {
            EvalMode::Canonical => engine.call_fn(eval_ctx.scope_mut(), &ast, "canonical_data", ()),
            EvalMode::Local => engine.call_fn(eval_ctx.scope_mut(), &ast, "local_data", ("TODO",)),
        };
        let data =
            data.map_err(|err| anyhow!("Failed to call data function: {}", err.to_string()))?;
        eval_ctx.scope_mut().push_constant("data", data);
        Ok(eval_ctx)
    }

    pub fn eval_rhai(&self, mode: EvalMode, expr: &str) -> Result<String> {
        let engine = eval_ctx::make_engine();
        let mut eval_ctx = self
            .prepare_eval_ctx(mode, &engine)
            .context("Failed to prepare eval_ctx")?;
        let result = engine
            .eval_expression_with_scope::<Dynamic>(eval_ctx.scope_mut(), expr)
            .map_err(|e| anyhow!(e.to_string()))
            .with_context(|| format!("Failed to evaluate: {}", expr))?;
        Ok(result.to_string())
    }

    pub fn eval_template_file(
        &self,
        mode: EvalMode,
        _path: impl AsRef<Path>,
        content: &str,
    ) -> Result<String> {
        let engine = eval_ctx::make_engine();
        let mut eval_ctx = self
            .prepare_eval_ctx(mode, &engine)
            .context("Failed to prepare eval_ctx")?;
        let doc = Document::parse_string(&content)?;
        Ok(doc.render(&mut eval_ctx)?)
    }

    pub fn sync_file(&self, mode: EvalMode, path: impl AsRef<Path>) -> Result<()> {
        let content = std::fs::read_to_string(&path)?;
        let rendered = self
            .eval_template_file(mode, &path, &content)
            .with_context(|| {
                format!("Failed to eval template file: {}", path.as_ref().display())
            })?;
        std::fs::write(&path, rendered)?;
        Ok(())
    }

    pub fn prepare_canonical(&self) -> Result<()> {
        let thing_paths = self.list_thing_paths()?;
        for thing_dir in thing_paths {
            let tmpl_list_file = thing_dir.join("yolk_templates");
            let tmpl_files = if tmpl_list_file.is_file() {
                let tmpl_paths = std::fs::read_to_string(tmpl_list_file)?;
                tmpl_paths
                    .lines()
                    .map(|x| thing_dir.join(x))
                    .collect::<Vec<_>>()
            } else {
                vec![]
            };
            dbg!(&tmpl_files);

            let thing_dir = thing_dir.canonicalize()?;
            let within_local = thing_dir.strip_prefix(self.yolk_paths.local_dir_path())?;
            copy_dir_via(
                &thing_dir,
                self.yolk_paths.canonical_dir_path().join(within_local),
                &|from, to| {
                    println!("Looking at copying {} to {}", from.display(), to.display());
                    // TODO: this to_path_buf seems unnecesarily inefficient.
                    if tmpl_files.contains(&from.to_path_buf()) {
                        println!("is in tmpl_paths");
                        let content = std::fs::read_to_string(&from)?;
                        let rendered =
                            self.eval_template_file(EvalMode::Canonical, &from, &content)?;
                        fs_err::write(&to, rendered)?;
                    } else {
                        fs_err::copy(from, to)?;
                    }

                    Ok(())
                },
            )?;
        }
        Ok(())
    }

    pub fn list_thing_paths(&self) -> Result<Vec<PathBuf>> {
        let entries = self.yolk_paths.local_dir_path().read_dir()?;
        Ok(entries
            .filter_map(|entry| entry.ok().map(|x| x.path()))
            .collect())
    }
}

/// Recursively copy a directory using a user-provided file copy function.
fn copy_dir_via<F: Fn(&Path, &Path) -> Result<()>>(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    copy_file: &F,
) -> Result<()> {
    fs_err::create_dir_all(&dst)?;
    for entry in fs_err::read_dir(src.as_ref())? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            copy_dir_via(
                entry.path(),
                dst.as_ref().join(entry.file_name()),
                copy_file,
            )?;
        } else {
            copy_file(&entry.path(), &dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalMode {
    Local,
    Canonical,
}

#[cfg(test)]
mod test {
    use assert_fs::{
        assert::PathAssert,
        prelude::{FileWriteStr, PathChild},
    };
    use p::path::{exists, is_dir, is_file, is_symlink};
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

        home.child("yolk/yolk.rhai").assert(is_file());
        home.child("yolk/local").assert(is_dir());

        yolk.add_thing("foo", home.child("config/foo.toml"))
            .unwrap();

        home.child("yolk/local/foo/config/foo.toml")
            .assert(is_file());
        home.child("config/foo.toml").assert(is_symlink());

        fs_err::remove_file(home.join("config").join("foo.toml")).unwrap();
        home.child("config/foo.toml").assert(exists().not());
        yolk.use_thing("foo").unwrap();
        home.child("config/foo.toml").assert(is_symlink());
    }

    #[test]
    fn test_syncing() {
        let home = assert_fs::TempDir::new().unwrap();
        // deliberately non-sense state -- both parts need to change at one point, depending on canonical vs local
        let foo_toml_initial = indoc::indoc! {r#"
            # {% replace /'.*'/ `'${data.value}'` %}
            value = 'local'
        "#};
        home.child("config/foo.toml")
            .write_str(&foo_toml_initial)
            .unwrap();
        let yp = YolkPaths::new(home.join("yolk"), home.to_path_buf());
        let yolk = Yolk::new(yp);
        yolk.init_yolk().unwrap();
        home.child("yolk/yolk.rhai")
            .write_str(indoc::indoc! {r#"
                fn canonical_data() { #{value: "canonical"} }
                fn local_data(machine_name) { #{value: "local"} }
            "#})
            .unwrap();
        yolk.add_thing("foo", home.join("config").join("foo.toml"))
            .unwrap();
        home.child("yolk/local/foo/yolk_templates")
            .write_str("config/foo.toml")
            .unwrap();
        home.child("config/foo.toml").assert(foo_toml_initial);
        yolk.sync().unwrap();
        home.child("config/foo.toml").assert(indoc::indoc! {r#"
            # {% replace /'.*'/ `'${data.value}'` %}
            value = 'local'
        "#});
        yolk.prepare_canonical().unwrap();
        home.child("yolk/canonical/foo/config/foo.toml")
            .assert(indoc::indoc! {r#"
                # {% replace /'.*'/ `'${data.value}'` %}
                value = 'canonical'
            "#});

        // Update the state, to see if applying again just works :tm:
        home.child("yolk/yolk.rhai")
            .write_str(indoc::indoc! {r#"
                fn canonical_data() { #{value: "new canonical"} }
                fn local_data(machine_name) { #{value: "new local"} }
            "#})
            .unwrap();
        yolk.sync().unwrap();
        home.child("config/foo.toml").assert(indoc::indoc! {r#"
            # {% replace /'.*'/ `'${data.value}'` %}
            value = 'new local'
        "#});
        yolk.prepare_canonical().unwrap();
        home.child("yolk/canonical/foo/config/foo.toml")
            .assert(indoc::indoc! {r#"
                # {% replace /'.*'/ `'${data.value}'` %}
                value = 'new canonical'
            "#});
    }
}
