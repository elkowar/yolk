use std::path::{Path, PathBuf};

use fs_err::PathExt;
use miette::{Context, IntoDiagnostic, Result};
use mlua::{Function, Value};

use crate::{
    eval_ctx::EvalCtx,
    script::sysinfo::SystemInfo,
    templating::document::Document,
    util,
    yolk_paths::{Egg, YolkPaths},
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

    /// Recurse through a given `path`, assumed to be within the given eggs dir,
    /// and `use` that path.
    /// This means:
    /// - If it is a file, symlink.
    /// - If it is a directory that does not exist in the target location, symlink.
    /// - If it is a directory that already exists in the target location, recurse into it.
    fn use_recursively(&self, egg_name: &str, path: &impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let path = path.fs_err_canonicalize().into_diagnostic()?;
        tracing::debug!("use_recursively({}, {})", egg_name, path.display());
        let path_relative = path
            .strip_prefix(self.yolk_paths.egg_path(egg_name))
            .into_diagnostic()?;

        // Ensure that we skip the yolk_templates file, but only when it's a direct child of the egg dir.
        if path.file_name() == Some("yolk_templates".as_ref())
            && path.parent().unwrap() == self.yolk_paths.egg_path(egg_name)
        {
            return Ok(());
        }

        let in_home = self.yolk_paths.home_path().join(path_relative);
        if in_home.exists() {
            if in_home.is_symlink() && in_home.fs_err_read_link().into_diagnostic()? == path {
                return Ok(());
            } else if in_home.is_dir() && path.is_dir() {
                for entry in path.fs_err_read_dir().into_diagnostic()? {
                    let entry = entry.into_diagnostic()?;
                    self.use_recursively(egg_name, &entry.path())?;
                }
            } else if !in_home.is_symlink() {
                miette::bail!("File {} already exists", in_home.display());
            } else if in_home.is_dir() || path.is_dir() {
                miette::bail!(
                    "Conflicting file or directory {} with {}",
                    path.display(),
                    in_home.display()
                );
            }
        } else {
            util::create_symlink(path, in_home).into_diagnostic()?;
        }

        Ok(())
    }

    pub fn use_egg(&self, egg_name: &str) -> Result<()> {
        tracing::info!("Using egg {}", egg_name);
        let egg = self.yolk_paths.get_egg(egg_name)?;
        for entry in egg.entries()? {
            self.use_recursively(egg_name, &entry.path())?;
        }
        self.sync_to_mode(EvalMode::Local)?;
        Ok(())
    }

    pub fn add_egg(&self, egg_name: &str, path: impl AsRef<Path>) -> Result<()> {
        let original_path = fs_err::canonicalize(path.as_ref()).into_diagnostic()?;
        let Ok(relative_to_home) = original_path.strip_prefix(self.yolk_paths.home_path()) else {
            return Err(miette::miette!(
                "Path {} is not in the home directory {}",
                original_path.display(),
                self.yolk_paths.home_path().display()
            ));
        };
        let new_local_path = self.yolk_paths.egg_path(egg_name).join(relative_to_home);
        fs_err::create_dir_all(new_local_path.parent().unwrap()).into_diagnostic()?;
        fs_err::rename(&original_path, &new_local_path).into_diagnostic()?;
        // TODO: This can be optimized a lot, as we assume we only need to re-use that one entry we just added.
        // However, we can't just naively create a symlink, as we don't know on that dir level to start symlinking.
        self.use_egg(egg_name)?;
        Ok(())
    }

    pub fn sync_to_mode(&self, mode: EvalMode) -> Result<()> {
        let egg_paths = self.list_egg_paths()?;
        let mut eval_ctx = self
            .prepare_eval_ctx_for_templates(mode)
            .context("Failed to prepare eval_ctx")?;

        for egg_dir in egg_paths {
            let egg = Egg::open(egg_dir)?;
            let tmpl_paths = egg.template_paths()?;
            for templated_file in tmpl_paths {
                if templated_file.is_file() {
                    if let Err(err) = self.sync_template_file(&mut eval_ctx, &templated_file) {
                        eprintln!(
                            "Warning: Failed to sync templated file {}: {err:?}",
                            templated_file.display(),
                        );
                    }
                } else {
                    println!(
                        "Warning: {} was specified as templated file, but doesn't exist",
                        templated_file.display()
                    );
                }
            }
        }
        Ok(())
    }

    pub fn prepare_eval_ctx_for_templates(&self, mode: EvalMode) -> Result<EvalCtx> {
        let sysinfo = match mode {
            EvalMode::Canonical => SystemInfo::canonical(),
            EvalMode::Local => SystemInfo::generate(),
        };
        let eval_ctx = EvalCtx::new_for_tag()?;
        let yolk_file = fs_err::read_to_string(self.yolk_paths.script_path()).into_diagnostic()?;

        // TODO: In the future, parse the lua error message for line number
        // and show a proper error span
        eval_ctx
            .lua()
            .load(&yolk_file)
            .set_name("yolk.lua")
            .exec()
            .into_diagnostic()?;
        let globals = eval_ctx.lua().globals();
        let data: Result<Value, _> = match mode {
            EvalMode::Canonical => globals
                .get::<Function>("canonical_data")
                .into_diagnostic()
                .with_context(|| "Failed to get canonical_data function")?
                .call(()),
            EvalMode::Local => globals
                .get::<Function>("local_data")
                .into_diagnostic()
                .with_context(|| "Failed to get local_data function")?
                .call(sysinfo.clone()),
        };
        let data = data
            .into_diagnostic()
            .with_context(|| "Failed to call data function".to_string())?;
        globals.set("data", data).into_diagnostic()?;
        globals.set("system", sysinfo).into_diagnostic()?;
        Ok(eval_ctx)
    }

    /// Evaluate a lua expression as though it was included in a template tag.
    pub fn eval_template_lua(&self, mode: EvalMode, expr: &str) -> Result<String> {
        let eval_ctx = self
            .prepare_eval_ctx_for_templates(mode)
            .context("Failed to prepare eval_ctx")?;
        let result = eval_ctx
            .lua()
            .load(expr)
            .set_name("expr")
            .eval::<Value>()
            .into_diagnostic()?;
        result.to_string().into_diagnostic()
    }

    /// Evaluate a templated file
    pub fn eval_template(&self, eval_ctx: &mut EvalCtx, content: &str) -> Result<String> {
        let doc = Document::parse_string(content).context("Failed to parse document")?;
        doc.render(eval_ctx)
            .map_err(|e| e.with_source_code(content.to_string()))
            .context("Failed to render document")
    }

    pub fn sync_template_file(&self, eval_ctx: &mut EvalCtx, path: impl AsRef<Path>) -> Result<()> {
        tracing::info!("Syncing file {}", path.as_ref().display());
        let content = fs_err::read_to_string(&path).into_diagnostic()?;
        let rendered = self.eval_template(eval_ctx, &content).with_context(|| {
            format!("Failed to eval template file: {}", path.as_ref().display())
        })?;
        fs_err::write(&path, rendered).into_diagnostic()?;
        Ok(())
    }

    pub fn with_canonical_state<T>(&self, f: impl FnOnce() -> Result<T>) -> Result<T> {
        println!("Converting all templates into their canonical state");
        self.sync_to_mode(EvalMode::Canonical)?;
        let result = f();
        println!("Converting all templates back to the local state");
        self.sync_to_mode(EvalMode::Local)?;
        result
    }

    pub fn add_to_templated_files(&self, egg_name: &str, paths: &[PathBuf]) -> Result<()> {
        let egg = self.yolk_paths.get_egg(egg_name)?;
        egg.add_to_template_paths(paths)?;
        self.sync_to_mode(EvalMode::Local)?;
        Ok(())
    }

    pub fn list_egg_paths(&self) -> Result<Vec<PathBuf>> {
        let entries = self
            .yolk_paths
            .eggs_dir_path()
            .fs_err_read_dir()
            .into_diagnostic()?;
        Ok(entries
            .filter_map(|entry| entry.ok().map(|x| x.path()))
            .collect())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalMode {
    Local,
    Canonical,
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use assert_fs::{
        assert::PathAssert,
        prelude::{FileWriteStr, PathChild, PathCreateDir},
    };
    use p::path::{exists, is_dir, is_file, is_symlink};
    use predicates::{self as p, path::FileTypePredicate};
    use predicates::{
        boolean::{AndPredicate, NotPredicate},
        prelude::PredicateBooleanExt,
    };
    use testresult::TestResult;

    use crate::yolk_paths::YolkPaths;

    use super::{EvalMode, Yolk};

    fn is_direct_file(
    ) -> AndPredicate<FileTypePredicate, NotPredicate<FileTypePredicate, Path>, Path> {
        is_file().and(is_symlink().not())
    }

    #[test]
    fn test_setup() -> TestResult {
        let home = assert_fs::TempDir::new()?;
        home.child("config/foo.toml").write_str("")?;
        let yolk = Yolk::new(YolkPaths::new(home.join("yolk"), home.to_path_buf()));
        yolk.init_yolk()?;

        home.child("yolk/yolk.lua").assert(is_file());
        home.child("yolk/eggs").assert(is_dir());

        yolk.add_egg("foo", home.child("config/foo.toml"))?;

        home.child("yolk/eggs/foo/config/foo.toml")
            .assert(is_file());
        home.child("config/foo.toml").assert(is_symlink());

        fs_err::remove_file(home.child("config/foo.toml"))?;
        fs_err::remove_dir(home.child("config"))?;
        home.child("config").assert(exists().not());
        yolk.use_egg("foo")?;
        home.child("config").assert(is_symlink());
        Ok(())
    }

    #[test]
    fn test_add_multiple_eggs() -> TestResult {
        let home = assert_fs::TempDir::new()?;
        home.child("config/foo.toml").write_str("")?;
        home.child("config/bar.toml").write_str("")?;
        let yolk = Yolk::new(YolkPaths::new(home.join("yolk"), home.to_path_buf()));
        yolk.init_yolk()?;

        yolk.add_egg("foo", home.child("config/foo.toml"))?;
        yolk.add_egg("bar", home.child("config/bar.toml"))?;

        home.child("yolk/eggs/foo/config/foo.toml")
            .assert(is_file());
        home.child("yolk/eggs/bar/config/bar.toml")
            .assert(is_file());
        home.child("config/foo.toml").assert(is_symlink());
        home.child("config/bar.toml").assert(is_symlink());
        Ok(())
    }

    #[test]
    fn test_multiple_files_in_same_egg() -> TestResult {
        let home = assert_fs::TempDir::new()?;
        home.child("config/foo.toml").write_str("")?;
        home.child("config/foo2.toml").write_str("")?;
        let yolk = Yolk::new(YolkPaths::new(home.join("yolk"), home.to_path_buf()));
        yolk.init_yolk()?;

        yolk.add_egg("foo", home.child("config/foo.toml"))?;
        yolk.add_egg("foo", home.child("config/foo2.toml"))?;

        home.child("yolk/eggs/foo/config/foo.toml")
            .assert(is_direct_file());
        home.child("yolk/eggs/foo/config/foo2.toml")
            .assert(is_direct_file());
        home.child("config/foo.toml").assert(is_symlink());
        home.child("config/foo2.toml").assert(is_symlink());
        Ok(())
    }

    #[test]
    fn test_syncing() -> TestResult {
        let home = assert_fs::TempDir::new()?;
        // deliberately non-sense state -- both parts need to change at one point, depending on canonical vs local
        let foo_toml_initial = indoc::indoc! {r#"
            # {# replace(`'.*'`, `'{data.value}'`) #}
            value = 'foo'
        "#};
        home.child("config/foo.toml").write_str(foo_toml_initial)?;
        let yp = YolkPaths::new(home.join("yolk"), home.to_path_buf());
        let yolk = Yolk::new(yp);
        yolk.init_yolk()?;
        home.child("yolk/yolk.lua").write_str(indoc::indoc! {r#"
                function canonical_data() return {value = "canonical"} end
                function local_data(system) return {value = "local"} end
            "#})?;
        yolk.add_egg("foo", home.join("config").join("foo.toml"))?;
        home.child("yolk/eggs/foo/yolk_templates")
            .write_str("config/foo.toml")?;
        home.child("config/foo.toml").assert(foo_toml_initial);
        yolk.sync_to_mode(EvalMode::Local)?;
        home.child("config/foo.toml").assert(indoc::indoc! {r#"
            # {# replace(`'.*'`, `'{data.value}'`) #}
            value = 'local'
        "#});
        yolk.with_canonical_state(|| {
            home.child("yolk/eggs/foo/config/foo.toml")
                .assert(indoc::indoc! {r#"
                    # {# replace(`'.*'`, `'{data.value}'`) #}
                    value = 'canonical'
                "#});
            Ok(())
        })?;

        // Update the state, to see if applying again just works :tm:
        home.child("yolk/yolk.lua").write_str(indoc::indoc! {r#"
                function canonical_data() return {value = "new canonical"} end
                function local_data(system) return {value = "new local"} end
            "#})?;
        yolk.sync_to_mode(EvalMode::Local)?;
        home.child("config/foo.toml").assert(indoc::indoc! {r#"
            # {# replace(`'.*'`, `'{data.value}'`) #}
            value = 'new local'
        "#});
        yolk.with_canonical_state(|| {
            home.child("yolk/eggs/foo/config/foo.toml")
                .assert(indoc::indoc! {r#"
                # {# replace(`'.*'`, `'{data.value}'`) #}
                value = 'new canonical'
            "#});
            Ok(())
        })?;
        Ok(())
    }

    #[test]
    fn test_add_to_templated_files() -> TestResult {
        let home = assert_fs::TempDir::new()?;
        home.child("config/foo.toml")
            .write_str("# {# replace(`'.*'`, `bar`) #}\nvalue = 'foo'")?;
        let yolk = Yolk::new(YolkPaths::new(home.join("yolk"), home.to_path_buf()));
        yolk.init_yolk()?;
        yolk.add_egg("foo", home.child("config/foo.toml"))?;
        yolk.add_to_templated_files("foo", &[home.child("config/foo.toml").to_path_buf()])?;
        home.child("yolk/eggs/foo/yolk_templates")
            .assert("config/foo.toml");
        home.child("yolk_templates").assert(exists().not());
        yolk.use_egg("foo")?;
        home.child("yolk_templates").assert(exists().not());
        Ok(())
    }

    #[test]
    fn test_add_template_inexistant_egg() -> TestResult {
        let home = assert_fs::TempDir::new()?;
        home.child("config/foo.toml")
            .write_str("# {% replace /'.*'/ `bar` %}\nvalue = 'foo'")?;
        let yolk = Yolk::new(YolkPaths::new(home.join("yolk"), home.to_path_buf()));
        yolk.init_yolk()?;
        assert!(yolk
            .add_to_templated_files("foo", &[home.child("config/foo.toml").to_path_buf()])
            .is_err());
        home.child("config/foo.toml").assert(is_direct_file());
        Ok(())
    }

    #[test]
    fn test_re_use_egg() -> TestResult {
        let home = assert_fs::TempDir::new()?;
        let yolk = Yolk::new(YolkPaths::new(home.join("yolk"), home.to_path_buf()));
        yolk.init_yolk()?;
        home.child("foo.toml").write_str("")?;
        home.child("test/foo.toml").write_str("")?;
        yolk.add_egg("foo", home.child("foo.toml"))?;
        yolk.add_egg("foo", home.child("test"))?;
        home.child("yolk/eggs/foo/bar.toml").write_str("")?;
        home.child("yolk/eggs/foo/test/bar.toml").write_str("")?;
        yolk.use_egg("foo")?;
        home.child("bar.toml").assert(is_symlink());
        home.child("test").assert(is_symlink());
        home.child("test/foo.toml").assert(is_direct_file());
        home.child("test/bar.toml").assert(is_direct_file());
        Ok(())
    }

    #[test]
    fn test_add_to_existing_egg() -> TestResult {
        let home = assert_fs::TempDir::new()?;
        let yolk = Yolk::new(YolkPaths::new(home.join("yolk"), home.to_path_buf()));
        yolk.init_yolk()?;
        home.child("foo_dir").create_dir_all()?;
        home.child("foo_dir/foo").write_str("")?;
        yolk.add_egg("foo", home.child("foo_dir"))?;
        home.child("foo_dir").assert(is_symlink());
        home.child("foo_dir/foo").assert(is_direct_file());

        home.child("foo.toml").write_str("")?;
        yolk.add_egg("foo", home.child("foo.toml"))?;
        home.child("foo.toml").assert(is_symlink());

        Ok(())
    }

    #[test]
    fn test_use_logic() -> TestResult {
        let home = assert_fs::TempDir::new()?;
        home.child("existing-dir").create_dir_all()?;
        let yolk = Yolk::new(YolkPaths::new(home.join("yolk"), home.to_path_buf()));
        yolk.init_yolk()?;
        home.child("yolk/eggs/foo/new-dir/foo.toml").write_str("")?;
        home.child("yolk/eggs/foo/existing-dir/new-subdir/foo.toml")
            .write_str("")?;
        home.child("yolk/eggs/foo/existing-dir/new-file.toml")
            .write_str("")?;
        yolk.use_egg("foo")?;
        home.child("new-dir").assert(is_symlink());
        home.child("existing-dir")
            .assert(is_symlink().not().and(is_dir()));
        home.child("existing-dir/new-subdir").assert(is_symlink());
        home.child("existing-dir/new-file.toml")
            .assert(is_symlink());
        Ok(())
    }
}
