use std::{collections::HashMap, path::Path};

use expanduser::expanduser;
use fs_err::PathExt as _;
use miette::{Context, IntoDiagnostic, NamedSource, Result};
use mlua::Value;

use crate::{
    eggs_config::EggConfig,
    eval_ctx::EvalCtx,
    script::sysinfo::SystemInfo,
    templating::{document::Document, template_error::TemplateError},
    util::{self, PathExt as _},
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

    pub fn eval_eggs_lua(&self, eval_ctx: &mut EvalCtx) -> Result<HashMap<String, EggConfig>> {
        let eggs_lua_path = self.yolk_paths.eggs_lua_path();
        let eggs_lua_content = fs_err::read_to_string(&eggs_lua_path).into_diagnostic()?;
        eval_ctx
            .eval_lua::<HashMap<String, EggConfig>>(
                &eggs_lua_path.to_string_lossy(),
                &eggs_lua_content,
            )
            .map_err(|e| {
                miette::Report::from(e)
                    .with_source_code(
                        NamedSource::new(eggs_lua_path.to_string_lossy(), eggs_lua_content)
                            .with_language("lua"),
                    )
                    .wrap_err("Failed to execute eggs.lua")
            })
    }

    /// Execute the `eggs.lua` script and deploy the resulting eggs.
    pub fn deploy(&self) -> Result<()> {
        let mut eval_ctx = self
            .prepare_eval_ctx_for_templates(EvalMode::Local)
            .context("Failed to prepare evaluation context")?;
        let deployment_config = self.eval_eggs_lua(&mut eval_ctx)?;
        for (egg_name, egg_config) in deployment_config {
            self.deploy_egg(&egg_name, &egg_config)?;
        }
        Ok(())
    }

    /// Deploy a given of [`EggConfig`]
    pub fn deploy_egg(&self, name: &str, config: &EggConfig) -> Result<()> {
        tracing::info!("Deploying egg {name}");
        if config.enabled {
            for (source, target) in &config.targets {
                let source = self.yolk_paths.egg_path(&name).canonical()?.join(source);
                let target = expanduser(target.to_string_lossy()).into_diagnostic()?;
                let target = if target.is_absolute() {
                    target
                } else {
                    self.paths().home_path().join(target)
                };
                symlink_recursive(source, &target)?;
            }
        } else {
            for (source, target) in &config.targets {
                let source = self.yolk_paths.egg_path(&name).canonical()?.join(source);
                let target = expanduser(target.to_string_lossy()).into_diagnostic()?;
                let target = if target.is_absolute() {
                    target
                } else {
                    self.paths().home_path().join(target)
                };
                remove_symlink_recursive(source, &target)?;
            }
        }
        Ok(())
    }

    pub fn sync_to_mode(&self, mode: EvalMode) -> Result<()> {
        let mut eval_ctx = self
            .prepare_eval_ctx_for_templates(mode)
            .context("Failed to prepare evaluation context")?;

        let egg_configs = self.eval_eggs_lua(&mut eval_ctx)?;

        for egg in self.list_eggs()? {
            let egg = egg?;
            let egg_config = egg_configs
                .get(&egg.name().to_string())
                .ok_or_else(|| miette::miette!("Egg {} not found in eggs.lua", egg.name()))?;

            for templated_file in &egg_config.templates {
                let templated_file = egg.path().join(templated_file);
                if templated_file.is_file() {
                    if let Err(err) = self.sync_template_file(&mut eval_ctx, &templated_file) {
                        eprintln!(
                            "Warning: Failed to sync templated file {}: {err:?}",
                            templated_file.display(),
                        );
                    }
                    tracing::info!("Synced templated file {}", templated_file.display());
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
        let eval_ctx = EvalCtx::new_in_mode(mode)?;
        let yolk_file =
            fs_err::read_to_string(self.yolk_paths.yolk_lua_path()).into_diagnostic()?;

        let globals = eval_ctx.lua().globals();
        globals.set("SYSTEM", sysinfo).into_diagnostic()?;
        globals
            .set("LOCAL", mode == EvalMode::Local)
            .into_diagnostic()?;
        eval_ctx.exec_lua("yolk.lua", &yolk_file).map_err(|e| {
            miette::Report::from(e)
                .with_source_code(
                    NamedSource::new(self.yolk_paths.yolk_lua_path().to_string_lossy(), yolk_file)
                        .with_language("lua"),
                )
                .wrap_err("Failed to execute yolk.lua")
        })?;
        Ok(eval_ctx)
    }

    /// Evaluate a lua expression as though it was included in a template tag.
    pub fn eval_template_lua(&self, mode: EvalMode, expr: &str) -> Result<String> {
        let eval_ctx = self
            .prepare_eval_ctx_for_templates(mode)
            .context("Failed to prepare evaluation context")?;
        tracing::debug!("Evaluating lua expression: {}", expr);
        eval_ctx
            .eval_template_lua::<Value>("expr", expr)
            .map_err(|e| TemplateError::from_lua_error(e, 0..expr.len()))
            .map_err(|e| miette::Report::from(e).with_source_code(expr.to_string()))?
            .to_string()
            .into_diagnostic()
    }

    /// Evaluate a templated file and return the rendered content.
    ///
    /// The `file_path` is just used for error reporting.
    pub fn eval_template(
        &self,
        eval_ctx: &mut EvalCtx,
        file_path: &str,
        content: &str,
    ) -> Result<String> {
        let doc = Document::parse_string_named(file_path, content)
            .with_context(|| format!("Failed to parse document `{file_path}`"))?;
        tracing::debug!("Rendering template");
        doc.render(eval_ctx)
            .with_context(|| format!("Failed to render document `{file_path}`"))
    }

    /// Sync a single template file in place on the filesystem.
    pub fn sync_template_file(&self, eval_ctx: &mut EvalCtx, path: impl AsRef<Path>) -> Result<()> {
        tracing::info!("Syncing file {}", path.as_ref().display());
        let content = fs_err::read_to_string(&path).into_diagnostic()?;
        let rendered = self
            .eval_template(eval_ctx, &path.as_ref().to_string_lossy(), &content)
            .with_context(|| {
                format!("Failed to eval template file: {}", path.as_ref().display())
            })?;
        if rendered == content {
            return Ok(());
        }
        fs_err::write(&path, rendered).into_diagnostic()?;
        Ok(())
    }

    /// Run a given closure with all templates in their canonical state.
    ///
    /// First syncs them to canonical then runs the closure, then syncs them back to local.
    pub fn with_canonical_state<T>(&self, f: impl FnOnce() -> Result<T>) -> Result<T> {
        // TODO: Consider using a pre_commit and post_commit hook instead of doing all this stuff.
        tracing::info!("Converting all templates into their canonical state");
        self.sync_to_mode(EvalMode::Canonical)?;
        let result = f();
        tracing::info!("Converting all templates back to the local state");
        self.sync_to_mode(EvalMode::Local)?;
        result
    }

    pub fn list_eggs(&self) -> Result<impl Iterator<Item = Result<Egg>> + '_> {
        self.yolk_paths.list_eggs()
    }
}

/// Set up a symlink from the given `source` to the given `target`, recursively.
///
/// Requires both paths to be absolute.
///
/// This means:
/// - If `source` is a file, symlink.
/// - If `source` is a directory that does not exist in `target`, symlink it.
/// - If `source` is a directory that already exists in `target`, recurse into it and `symlink_recursive` `source`s children.
fn symlink_recursive(source: impl AsRef<Path>, target: &impl AsRef<Path>) -> Result<()> {
    let source = source.as_ref();
    let target = target.as_ref();
    assert!(
        source.is_absolute(),
        "source path must be absolute, but was {}",
        target.display()
    );
    assert!(
        target.is_absolute(),
        "target path must be absolute, but was {}",
        target.display()
    );
    tracing::debug!(
        "symlink_recursive({}, {})",
        source.display(),
        target.display()
    );

    if target.exists() {
        if target.is_symlink() && target.fs_err_read_link().into_diagnostic()? == source {
            return Ok(());
        } else if target.is_dir() && source.is_dir() {
            for entry in source.fs_err_read_dir().into_diagnostic()? {
                let entry = entry.into_diagnostic()?;
                symlink_recursive(entry.path(), &target.join(entry.file_name()))?;
            }
        } else if !target.is_symlink() {
            miette::bail!("File {} already exists", target.display());
        } else if target.is_dir() || source.is_dir() {
            miette::bail!(
                "Conflicting file or directory {} with {}",
                source.display(),
                target.display()
            );
        }
    } else {
        util::create_symlink(&source, &target)?;
        println!("Symlinked {} to {}", source.display(), target.display());
    }
    Ok(())
}

fn remove_symlink_recursive(source: impl AsRef<Path>, target: &impl AsRef<Path>) -> Result<()> {
    let source = source.as_ref();
    let target = target.as_ref();
    if target.is_symlink() && target.canonical()? == source {
        tracing::info!(
            "Removing symlink {} -> {}",
            source.display(),
            target.display()
        );
        fs_err::remove_file(&target).into_diagnostic()?;
    } else if target.is_dir() && source.is_dir() {
        for entry in source.fs_err_read_dir().into_diagnostic()? {
            let entry = entry.into_diagnostic()?;
            remove_symlink_recursive(entry.path(), &target.join(entry.file_name()))?;
        }
    } else if target.exists() {
        tracing::info!(
            "Mapping {} -> {} was not matched by target file, skipping...",
            source.display(),
            target.display()
        );
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
        prelude::{FileWriteStr, PathChild, PathCreateDir},
    };
    use miette::IntoDiagnostic as _;
    use p::path::{exists, is_dir, is_symlink};
    use predicates::prelude::PredicateBooleanExt;
    use predicates::{self as p};
    use testresult::TestResult;

    use crate::{eggs_config::EggConfig, yolk_paths::YolkPaths};

    use super::{EvalMode, Yolk};

    fn setup_and_init() -> miette::Result<(assert_fs::TempDir, Yolk, assert_fs::fixture::ChildPath)>
    {
        let home = assert_fs::TempDir::new().into_diagnostic()?;
        let yolk = Yolk::new(YolkPaths::new(home.join("yolk"), home.to_path_buf()));
        let eggs = home.child("yolk/eggs");
        yolk.init_yolk()?;
        Ok((home, yolk, eggs))
    }

    // fn is_direct_file(
    // ) -> AndPredicate<FileTypePredicate, NotPredicate<FileTypePredicate, Path>, Path> {
    //     is_file().and(is_symlink().not())
    // }

    #[test]
    fn test_deploy_egg_config() -> TestResult {
        let (home, yolk, eggs) = setup_and_init()?;
        eggs.child("foo/foo.toml").write_str("")?;
        eggs.child("foo/thing/thing.toml").write_str("")?;
        yolk.deploy_egg(
            "foo",
            &EggConfig::builder()
                .target("foo.toml", home.child("foo.toml"))
                .target("thing", home.child("thing"))
                .build(),
        )?;
        home.child("foo.toml").assert(is_symlink());
        home.child("thing").assert(is_symlink());

        // Verify stow-style usage works
        home.child(".config").create_dir_all()?;
        eggs.child("bar/.config/thing.toml").write_str("")?;
        yolk.deploy_egg("bar", &EggConfig::builder().target(".", &home).build())?;
        home.child(".config").assert(is_dir());
        home.child(".config/thing.toml").assert(is_symlink());
        Ok(())
    }

    #[test]
    fn test_undeploy() -> TestResult {
        let (home, yolk, eggs) = setup_and_init()?;
        home.child(".config").create_dir_all()?;
        eggs.child("foo/foo.toml").write_str("")?;
        eggs.child("bar/.config/thing.toml").write_str("")?;
        yolk.deploy_egg(
            "foo",
            &EggConfig::builder()
                .target("foo.toml", home.child("foo.toml"))
                .build(),
        )?;
        home.child("foo.toml").assert(is_symlink());
        yolk.deploy_egg(
            "foo",
            &EggConfig::builder()
                .target("foo.toml", home.child("foo.toml"))
                .enabled(false)
                .build(),
        )?;
        home.child("foo.toml").assert(exists().not());

        // Verify stow-style usage works
        home.child(".config").create_dir_all()?;
        eggs.child("bar/.config/thing.toml").write_str("")?;
        yolk.deploy_egg("bar", &EggConfig::builder().target(".", &home).build())?;
        home.child(".config/thing.toml").assert(is_symlink());
        yolk.deploy_egg(
            "bar",
            &EggConfig::builder()
                .target(".", &home)
                .enabled(false)
                .build(),
        )?;
        home.child(".config/thing.toml").assert(exists().not());
        home.child(".config").assert(is_dir());
        Ok(())
    }

    #[test]
    fn test_syncing() -> TestResult {
        let (home, yolk, eggs) = setup_and_init()?;
        let foo_toml_initial = "{# data.value #}\nfoo";
        home.child("yolk/yolk.lua").write_str(indoc::indoc! {r#"
            data = if LOCAL then {value = "local"} else {value = "canonical"}
        "#})?;
        home.child("yolk/eggs.lua")
            .write_str(&format!("return {{ foo = `{}` }}", home.display()))?;
        eggs.child("foo/foo.toml").write_str(foo_toml_initial)?;
        yolk.deploy()?;
        yolk.sync_to_mode(EvalMode::Local)?;
        // No template set in eggs.lua, so no templating should happen
        home.child("foo.toml").assert(is_symlink());
        eggs.child("foo/foo.toml").assert(foo_toml_initial);

        // Now we make the file a template, so it should be updated
        home.child("yolk/eggs.lua").write_str(&format!(
            r#"return {{ foo = {{targets = "{}", templates = {{"foo.toml"}} }} }}"#,
            home.display()
        ))?;
        yolk.sync_to_mode(EvalMode::Local)?;
        eggs.child("foo/foo.toml").assert("{# data.value #}\nlocal");

        // Update the state, to see if applying again just works :tm:
        home.child("yolk/yolk.lua").write_str(indoc::indoc! {r#"
            data = if LOCAL then {value = "new local"} else {value = "new canonical"}
        "#})?;
        yolk.sync_to_mode(EvalMode::Local)?;
        home.child("foo.toml").assert("{# data.value #}\nnew local");
        yolk.with_canonical_state(|| {
            eggs.child("foo/foo.toml")
                .assert("{# data.value #}\nnew canonical");
            Ok(())
        })?;
        Ok(())
    }

    // #[test]
    // fn test_re_use_egg() -> TestResult {
    //     let home = assert_fs::TempDir::new()?;
    //     let yolk = Yolk::new(YolkPaths::new(home.join("yolk"), home.to_path_buf()));
    //     yolk.init_yolk()?;
    //     todo!("Write test");
    //     Ok(())
    // }

    // #[test]
    // fn test_add_to_existing_egg() -> TestResult {
    //     let home = assert_fs::TempDir::new()?;
    //     let yolk = Yolk::new(YolkPaths::new(home.join("yolk"), home.to_path_buf()));
    //     yolk.init_yolk()?;
    //     todo!("Write test");

    //     Ok(())
    // }

    // #[test]
    // fn test_use_logic() -> TestResult {
    //     let home = assert_fs::TempDir::new()?;
    //     home.child("existing-dir").create_dir_all()?;
    //     let yolk = Yolk::new(YolkPaths::new(home.join("yolk"), home.to_path_buf()));
    //     yolk.init_yolk()?;
    //     home.child("yolk/eggs/foo/new-dir/foo.toml").write_str("")?;
    //     home.child("yolk/eggs/foo/existing-dir/new-subdir/foo.toml")
    //         .write_str("")?;
    //     home.child("yolk/eggs/foo/existing-dir/new-file.toml")
    //         .write_str("")?;
    //     todo!("Rewrite this test without dependency on yolk");
    //     home.child("new-dir").assert(is_symlink());
    //     home.child("existing-dir")
    //         .assert(is_symlink().not().and(is_dir()));
    //     home.child("existing-dir/new-subdir").assert(is_symlink());
    //     home.child("existing-dir/new-file.toml")
    //         .assert(is_symlink());
    //     Ok(())
    // }
}
