use std::{collections::HashMap, path::Path};

use expanduser::expanduser;
use fs_err::PathExt as _;
use miette::{Context, IntoDiagnostic, NamedSource, Result};

use crate::{
    eggs_config::EggConfig,
    eval_ctx::EvalCtx,
    script::{lua_error::RhaiError, sysinfo::SystemInfo},
    templating::document::Document,
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

    /// Deploy or un-deploy a given [`EggConfig`]
    pub fn sync_egg_deployment(&self, name: &str, config: &EggConfig) -> Result<()> {
        let egg = self.yolk_paths.get_egg(name)?;
        let deployed = egg.is_deployed()?;
        if config.enabled && !deployed {
            tracing::info!("Deploying egg {name}");
            for (source, target) in &config.targets {
                let source = self.yolk_paths.egg_path(&name).canonical()?.join(source);
                let target = expanduser(target.to_string_lossy()).into_diagnostic()?;
                let target = if target.is_absolute() {
                    target
                } else {
                    self.paths().home_path().join(target)
                };
                if let Err(e) = symlink_recursive(&source, &target) {
                    eprintln!(
                        "Warning: Failed to deploy {}: {e:?}",
                        source.to_abbrev_str()
                    );
                }
            }
        } else if !config.enabled && deployed {
            tracing::debug!("Removing egg {name}");
            for (source, target) in &config.targets {
                let source = self.yolk_paths.egg_path(&name).canonical()?.join(source);
                let target = expanduser(target.to_string_lossy()).into_diagnostic()?;
                let target = if target.is_absolute() {
                    target
                } else {
                    self.paths().home_path().join(target)
                };

                if let Err(e) = remove_symlink_recursive(&source, &target) {
                    eprintln!(
                        "Warning: Failed to remove deployment of {}: {e:?}",
                        source.to_abbrev_str()
                    );
                }
            }
        }
        Ok(())
    }

    /// First, sync the deployment of all eggs to the local system.
    /// Then, update any templated files in the eggs to the given mode.
    pub fn sync_to_mode(&self, mode: EvalMode) -> Result<()> {
        let mut eval_ctx = self
            .prepare_eval_ctx_for_templates(mode)
            .context("Failed to prepare evaluation context")?;

        let eggs_map = eval_ctx
            .scope_mut()
            .get_value::<rhai::Map>("eggs")
            .ok_or_else(|| miette::miette!("yolk.rhai did not define a global `eggs` variable"))?;
        let egg_configs: HashMap<String, EggConfig> = eggs_map
            .into_iter()
            .map(|(x, v)| Ok((x.into(), EggConfig::from_dynamic(v)?)))
            .collect::<Result<HashMap<_, _>, RhaiError>>()?;

        for (egg, egg_config) in &egg_configs {
            let egg = match self.yolk_paths.get_egg(egg) {
                Ok(egg) => egg,
                Err(e) => {
                    tracing::warn!("Warning: Failed to open egg {egg}: {e}");
                    continue;
                }
            };
            self.sync_egg_deployment(&egg.name(), &egg_config)?;

            for tmpl_path_glob in &egg_config.templates {
                let tmpl_path_glob = egg.path().join(tmpl_path_glob);
                let glob_paths = match glob::glob(&tmpl_path_glob.to_string_lossy()) {
                    Ok(x) => x,
                    Err(err) => {
                        tracing::warn!(
                            "Failed to glob for templated file {}: {err:?}",
                            tmpl_path_glob.to_abbrev_str(),
                        );
                        continue;
                    }
                };
                for path_result in glob_paths {
                    let tmpl_path = match path_result {
                        Ok(x) => x,
                        Err(err) => {
                            tracing::warn!(
                                "Failed to glob for templated file {}: {err:?}",
                                tmpl_path_glob.to_abbrev_str(),
                            );
                            continue;
                        }
                    };
                    if tmpl_path.is_file() {
                        if let Err(err) = self.sync_template_file(&mut eval_ctx, &tmpl_path) {
                            tracing::warn!(
                                "Failed to sync templated file {}: {err:?}",
                                tmpl_path.to_abbrev_str(),
                            );
                        }
                        tracing::info!("Synced templated file {}", tmpl_path.to_abbrev_str());
                    } else if !tmpl_path.exists() {
                        tracing::warn!(
                            "{} was specified as templated file, but doesn't exist",
                            tmpl_path.to_abbrev_str()
                        );
                    }
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
        let mut eval_ctx = EvalCtx::new_in_mode(mode)?;
        let yolk_file =
            fs_err::read_to_string(self.yolk_paths.yolk_rhai_path()).into_diagnostic()?;

        eval_ctx.set_global("SYSTEM", sysinfo);
        eval_ctx.set_global("LOCAL", mode == EvalMode::Local);
        eval_ctx.set_and_run_header_ast(&yolk_file).map_err(|e| {
            miette::Report::from(e)
                .with_source_code(
                    NamedSource::new(
                        self.yolk_paths.yolk_rhai_path().to_string_lossy(),
                        yolk_file,
                    )
                    .with_language("rust"),
                )
                .wrap_err("Failed to execute yolk.rhai")
        })?;

        Ok(eval_ctx)
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
        tracing::info!("Syncing file {}", path.as_ref().to_abbrev_str());
        let content = fs_err::read_to_string(&path).into_diagnostic()?;
        let rendered = self
            .eval_template(eval_ctx, &path.as_ref().to_string_lossy(), &content)
            .with_context(|| {
                format!(
                    "Failed to eval template file: {}",
                    path.as_ref().to_abbrev_str()
                )
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
        source.to_abbrev_str(),
        target.to_abbrev_str()
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
            miette::bail!("File {} already exists", target.to_abbrev_str());
        } else if target.is_dir() || source.is_dir() {
            miette::bail!(
                "Conflicting file or directory {} with {}",
                source.to_abbrev_str(),
                target.to_abbrev_str()
            );
        }
    } else {
        util::create_symlink(&source, &target)?;
        println!(
            "Symlinked {} to {}",
            source.to_abbrev_str(),
            target.to_abbrev_str()
        );
    }
    Ok(())
}

fn remove_symlink_recursive(source: impl AsRef<Path>, target: &impl AsRef<Path>) -> Result<()> {
    let source = source.as_ref();
    let target = target.as_ref();
    if target.is_symlink() && target.canonical()? == source {
        tracing::info!(
            "Removing symlink {} -> {}",
            source.to_abbrev_str(),
            target.to_abbrev_str()
        );
        fs_err::remove_file(&target).into_diagnostic()?;
    } else if target.is_dir() && source.is_dir() {
        for entry in source.fs_err_read_dir().into_diagnostic()? {
            let entry = entry.into_diagnostic()?;
            remove_symlink_recursive(entry.path(), &target.join(entry.file_name()))?;
        }
    } else if target.exists() {
        miette::bail!(
            "Tried to remove deployment of {}, but {} doesn't link to it",
            source.to_abbrev_str(),
            target.to_abbrev_str()
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

    use crate::util::TestResult;
    use assert_fs::{
        assert::PathAssert,
        prelude::{FileWriteStr, PathChild, PathCreateDir},
    };
    use miette::IntoDiagnostic as _;
    use p::path::{exists, is_dir, is_symlink};
    use predicates::prelude::PredicateBooleanExt;
    use predicates::{self as p};

    use crate::{eggs_config::EggConfig, yolk_paths::YolkPaths};

    use super::{EvalMode, Yolk};

    fn setup_and_init() -> miette::Result<(assert_fs::TempDir, Yolk, assert_fs::fixture::ChildPath)>
    {
        let home = assert_fs::TempDir::new().into_diagnostic()?;
        let yolk = Yolk::new(YolkPaths::new(home.join("yolk"), home.to_path_buf()));
        std::env::set_var("HOME", home.to_string_lossy().to_string());
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
        yolk.sync_egg_deployment(
            "foo",
            &EggConfig::default()
                .with_target("foo.toml", home.child("foo.toml"))
                .with_target("thing", home.child("thing")),
        )?;
        home.child("foo.toml").assert(is_symlink());
        home.child("thing").assert(is_symlink());

        // Verify stow-style usage works
        home.child(".config").create_dir_all()?;
        eggs.child("bar/.config/thing.toml").write_str("")?;
        yolk.sync_egg_deployment("bar", &EggConfig::new(".", &home))?;
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
        yolk.sync_egg_deployment("foo", &EggConfig::new("foo.toml", home.child("foo.toml")))?;
        home.child("foo.toml").assert(is_symlink());
        yolk.sync_egg_deployment(
            "foo",
            &EggConfig::new("foo.toml", home.child("foo.toml")).with_enabled(false),
        )?;
        home.child("foo.toml").assert(exists().not());

        // Verify stow-style usage works
        home.child(".config").create_dir_all()?;
        eggs.child("bar/.config/thing.toml").write_str("")?;
        yolk.sync_egg_deployment("bar", &EggConfig::new(".", &home))?;
        home.child(".config/thing.toml").assert(is_symlink());
        yolk.sync_egg_deployment("bar", &EggConfig::new(".", &home).with_enabled(false))?;
        home.child(".config/thing.toml").assert(exists().not());
        home.child(".config").assert(is_dir());
        Ok(())
    }

    #[test]
    fn test_syncing() -> TestResult {
        let (home, yolk, eggs) = setup_and_init()?;
        let foo_toml_initial = "{# data.value #}\nfoo";
        home.child("yolk/yolk.rhai").write_str(&indoc::indoc! {r#"
            const data = if LOCAL { #{value: "local"} } else { #{value: "canonical"} };
            let eggs = #{foo: `~`};
        "#})?;
        eggs.child("foo/foo.toml").write_str(foo_toml_initial)?;
        yolk.sync_to_mode(EvalMode::Local)?;
        // No template set in eggs.rhai, so no templating should happen
        home.child("foo.toml").assert(is_symlink());
        eggs.child("foo/foo.toml").assert(foo_toml_initial);

        // Now we make the file a template, so it should be updated
        home.child("yolk/yolk.rhai").write_str(&indoc::indoc! {r#"
            const data = if LOCAL {#{value: "local"}} else {#{value: "canonical"}};
            let eggs = #{foo: #{targets: `~`, templates: ["foo.toml"]}};
        "#})?;

        yolk.sync_to_mode(EvalMode::Local)?;
        eggs.child("foo/foo.toml").assert("{# data.value #}\nlocal");

        // Update the state, to see if applying again just works :tm:
        home.child("yolk/yolk.rhai").write_str(&indoc::indoc! {r#"
                const data = if LOCAL {#{value: "new local"}} else {#{value: "new canonical"}};
                let eggs = #{foo: #{targets: `~`, templates: ["foo.toml"]}};
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
}

// TODO: write test to verify that hostname can be accessed from within templates and the scripts
