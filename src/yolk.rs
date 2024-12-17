use fs_err::PathExt as _;
use miette::{Context, IntoDiagnostic, NamedSource, Result, Severity};
use normalize_path::NormalizePath;
use std::{collections::HashMap, path::Path};

use crate::{
    eggs_config::{DeploymentStrategy, EggConfig},
    script::{eval_ctx::EvalCtx, rhai_error::RhaiError, sysinfo::SystemInfo},
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

    /// Deploy or un-deploy a given [`Egg`]
    #[tracing::instrument(skip_all, fields(egg = ?egg.name()))]
    pub fn sync_egg_deployment(&self, egg: &Egg) -> Result<()> {
        let deployed = egg
            .is_deployed()
            .with_context(|| format!("Failed to check deployment state for egg {}", egg.name()))?;
        tracing::debug!(
            egg.deployed = deployed,
            egg.enabled = egg.config().enabled,
            "Syncing egg deployment"
        );
        if egg.config().enabled && !deployed {
            tracing::info!("Deploying egg {}", egg.name());
            let mappings = egg
                .config()
                .targets_expanded(self.yolk_paths.home_path(), egg.path())
                .context("Failed to expand targets config for egg")?;
            let mut did_fail = false;
            for (in_egg, deployed) in &mappings {
                let deploy_mapping = || -> miette::Result<()> {
                    match egg.config().strategy {
                        DeploymentStrategy::Merge => {
                            cov_mark::hit!(deploy_merge);
                            symlink_recursive(egg.path(), in_egg, &deployed)?;
                        }
                        DeploymentStrategy::Put => {
                            cov_mark::hit!(deploy_put);
                            if let Some(parent) = deployed.parent() {
                                fs_err::create_dir_all(parent).map_err(|e| {
                                    miette::miette!(
                                        severity = Severity::Warning,
                                        "Warning: Failed to create parent dir for deployment of {}: {e:?}",
                                        in_egg.to_abbrev_str()
                                    )
                                })?;
                            }
                            util::create_symlink(in_egg, deployed)?;
                        }
                    }
                    Result::Ok(())
                };
                if let Err(e) = deploy_mapping() {
                    tracing::warn!(
                        "Warning: Failed to deploy {}: {e:?}",
                        in_egg.to_abbrev_str()
                    );
                    did_fail = true;
                }
            }
            debug_assert!(
                did_fail || egg.is_deployed()?,
                "Egg::is_deployed should return true after deploying"
            );
        } else if !egg.config().enabled && deployed {
            cov_mark::hit!(undeploy);
            tracing::debug!("Removing egg {}", egg.name());
            let mut did_fail = false;
            let mappings = egg
                .config()
                .targets_expanded(self.yolk_paths.home_path(), egg.path())?;
            for (in_egg, deployed) in &mappings {
                if let Err(e) = remove_symlink_recursive(in_egg, &deployed) {
                    did_fail = true;
                    tracing::warn!(
                        "Warning: Failed to remove deployment of {}: {e:?}",
                        in_egg.to_abbrev_str()
                    );
                }
            }
            debug_assert!(
                did_fail || !egg.is_deployed()?,
                "Egg::is_deployed should return false after undeploying"
            );
        }
        Ok(())
    }

    /// fetch the `eggs` variable from a given EvalCtx.
    pub fn load_egg_configs(&self, eval_ctx: &mut EvalCtx) -> Result<HashMap<String, EggConfig>> {
        let eggs_map = eval_ctx
            .yolk_file_module()
            .expect("Tried to load egg configs before loading yolk file. This is a bug.")
            .get_var_value::<rhai::Map>("eggs")
            .ok_or_else(|| miette::miette!("Could not find an `eggs` variable in scope"))?;
        Ok(eggs_map
            .into_iter()
            .map(|(x, v)| Ok((x.into(), EggConfig::from_dynamic(v)?)))
            .collect::<Result<HashMap<_, _>, RhaiError>>()?)
    }

    /// First, sync the deployment of all eggs to the local system.
    /// Then, update any templated files in the eggs to the given mode.
    pub fn sync_to_mode(&self, mode: EvalMode) -> Result<()> {
        let mut eval_ctx = self
            .prepare_eval_ctx_for_templates(mode)
            .context("Failed to prepare evaluation context")?;

        let egg_configs = self.load_egg_configs(&mut eval_ctx)?;

        for (name, egg_config) in egg_configs.into_iter() {
            if let Err(e) = self.sync_egg_to_mode(&mut eval_ctx, &name, egg_config) {
                tracing::warn!("Warning: Failed to sync egg {name}: {e:?}");
            }
        }
        Ok(())
    }

    fn sync_egg_to_mode(
        &self,
        eval_ctx: &mut EvalCtx,
        name: &str,
        egg_config: EggConfig,
    ) -> Result<()> {
        let egg = self.yolk_paths.get_egg(name, egg_config)?;
        self.sync_egg_deployment(&egg)?;
        let templates_expanded = egg.config().templates_globexpanded(egg.path())?;
        for tmpl_path in templates_expanded {
            if tmpl_path.is_file() {
                self.sync_template_file(eval_ctx, &tmpl_path)?;
            } else if !tmpl_path.exists() {
                tracing::warn!(
                    "{} was specified as templated file, but doesn't exist",
                    tmpl_path.to_abbrev_str()
                );
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
        eval_ctx.load_as_global_module(&yolk_file).map_err(|e| {
            miette::Report::from(e)
                .with_source_code(
                    NamedSource::new(
                        self.yolk_paths.yolk_rhai_path().to_string_lossy(),
                        yolk_file,
                    )
                    .with_language("Rust"),
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
        let path = path.as_ref();
        tracing::debug!("Syncing file {}", path.to_abbrev_str());
        let content = fs_err::read_to_string(path).into_diagnostic()?;
        let rendered = self
            .eval_template(eval_ctx, &path.to_string_lossy(), &content)
            .with_context(|| format!("Failed to eval template file: {}", path.to_abbrev_str()))?;
        if rendered == content {
            tracing::debug!("No changes needed in {}", path.to_abbrev_str());
            return Ok(());
        }
        fs_err::write(path, rendered).into_diagnostic()?;
        tracing::info!("Synced templated file {}", path.to_abbrev_str());
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

    pub fn list_eggs(&self) -> Result<Vec<Egg>> {
        let mut eval_ctx = self.prepare_eval_ctx_for_templates(EvalMode::Local)?;
        let egg_configs = self.load_egg_configs(&mut eval_ctx)?;
        egg_configs
            .into_iter()
            .map(|(name, config)| self.yolk_paths.get_egg(&name, config))
            .collect()
    }

    /// Run the yolk.rhai script, load the egg configs and return the requested egg.
    pub fn get_egg(&self, egg_name: &str) -> Result<Egg> {
        let mut eval_ctx = self.prepare_eval_ctx_for_templates(EvalMode::Local)?;
        let mut egg_configs = self.load_egg_configs(&mut eval_ctx)?;
        egg_configs
            .remove(egg_name)
            .ok_or_else(|| miette::miette!("No egg with name {egg_name}"))
            .and_then(|config| self.yolk_paths.get_egg(egg_name, config))
    }
}

/// Set up a symlink from the given `link_path` to the given `actual_path`, recursively.
/// Also takes the `egg_root` dir, to ensure we can safely delete any stale symlinks on the way there.
///
/// Requires all paths to be absolute.
///
/// This means:
/// - If `link_path` exists and is a file, abort
/// - If `link_path` exists and is a symlink into the egg dir, remove the symlink and then continue.
/// - If `actual_path` is a file, symlink.
/// - If `actual_path` is a directory that does not exist in `link_path`, symlink it.
/// - If `actual_path` is a directory that already exists in `link_path`, recurse into it and `symlink_recursive` `actual_path`s children.
fn symlink_recursive(
    egg_root: impl AsRef<Path>,
    actual_path: impl AsRef<Path>,
    link_path: &impl AsRef<Path>,
) -> Result<()> {
    let actual_path = actual_path.as_ref().normalize();
    let link_path = link_path.as_ref().normalize();
    let egg_root = egg_root.as_ref().normalize();
    assert!(
        link_path.is_absolute(),
        "link_ path must be absolute, but was {}",
        link_path.display()
    );
    assert!(
        actual_path.is_absolute(),
        "actual_path must be absolute, but was {}",
        actual_path.display()
    );
    assert!(
        actual_path.starts_with(&egg_root),
        "actual_path must be inside egg_root: {} not in {}",
        actual_path.display(),
        egg_root.display(),
    );
    tracing::debug!(
        "symlink_recursive({}, {})",
        actual_path.to_abbrev_str(),
        link_path.to_abbrev_str()
    );

    let actual_path = actual_path.canonical()?;

    tracing::trace!("Checking {}", link_path.to_abbrev_str());
    if link_path.is_symlink() {
        let link_target = link_path.fs_err_read_link().into_diagnostic()?;
        tracing::trace!(
            "link_path exists as symlink at {} -> {}",
            link_path.to_abbrev_str(),
            link_target.to_abbrev_str()
        );
        if link_target == actual_path {
            return Ok(());
        } else if link_target.exists() {
            miette::bail!(
                "Failed to create symlink {} -> {}, as a file already exists there",
                link_path.to_abbrev_str(),
                actual_path.to_abbrev_str(),
            );
        } else if link_target.starts_with(&egg_root) {
            tracing::info!(
                "Removing dead symlink {} -> {}",
                link_path.to_abbrev_str(),
                link_target.to_abbrev_str()
            );
            fs_err::remove_file(&link_path).into_diagnostic()?;
            cov_mark::hit!(remove_dead_symlink);
            // After we've removed that file, creating the symlink later will succeed!
        } else {
            miette::bail!(
                "Encountered dead symlink, but it doesn't target the egg dir: {}",
                link_path.to_abbrev_str(),
            );
        }
    } else if link_path.exists() {
        tracing::trace!(
            "link_path exists as non-symlink {}",
            link_path.to_abbrev_str(),
        );
        if link_path.is_dir() && actual_path.is_dir() {
            for entry in actual_path.fs_err_read_dir().into_diagnostic()? {
                let entry = entry.into_diagnostic()?;
                symlink_recursive(&egg_root, entry.path(), &link_path.join(entry.file_name()))?;
            }
            return Ok(());
        } else if link_path.is_dir() || actual_path.is_dir() {
            miette::bail!(
                "Conflicting file or directory {} with {}",
                actual_path.to_abbrev_str(),
                link_path.to_abbrev_str()
            );
        }
    }
    util::create_symlink(&actual_path, &link_path)?;
    tracing::info!(
        "created symlink {} -> {}",
        link_path.to_abbrev_str(),
        actual_path.to_abbrev_str(),
    );
    Ok(())
}

#[tracing::instrument(skip(actual_path, link_path), fields(
    actual_path = actual_path.as_ref().to_abbrev_str(),
    link_path = link_path.as_ref().to_abbrev_str()
))]
fn remove_symlink_recursive(
    actual_path: impl AsRef<Path>,
    link_path: &impl AsRef<Path>,
) -> Result<()> {
    let actual_path = actual_path.as_ref();
    let link_path = link_path.as_ref();
    if link_path.is_symlink() && link_path.canonical()? == actual_path {
        tracing::info!(
            "Removing symlink {} -> {}",
            link_path.to_abbrev_str(),
            actual_path.to_abbrev_str(),
        );
        fs_err::remove_file(link_path).into_diagnostic()?;
    } else if link_path.is_dir() && actual_path.is_dir() {
        for entry in actual_path.fs_err_read_dir().into_diagnostic()? {
            let entry = entry.into_diagnostic()?;
            remove_symlink_recursive(entry.path(), &link_path.join(entry.file_name()))?;
        }
    } else if link_path.exists() {
        miette::bail!(
            "Tried to remove deployment of {}, but {} doesn't link to it",
            actual_path.to_abbrev_str(),
            link_path.to_abbrev_str()
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

    use std::path::PathBuf;

    use crate::{
        eggs_config::DeploymentStrategy,
        util::{setup_and_init_test_yolk, TestResult},
        yolk_paths::Egg,
    };
    use assert_fs::{
        assert::PathAssert,
        prelude::{FileWriteStr, PathChild, PathCreateDir},
    };
    use p::path::{exists, is_dir, is_symlink};
    use predicates::prelude::PredicateBooleanExt;
    use predicates::{self as p};
    use test_log::test;

    use crate::eggs_config::EggConfig;

    use super::EvalMode;

    // fn is_direct_file(
    // ) -> AndPredicate<FileTypePredicate, NotPredicate<FileTypePredicate, Path>, Path> {
    //     is_file().and(is_symlink().not())
    // }

    #[test]
    fn test_deploy_egg_config() -> TestResult {
        cov_mark::check_count!(deploy_merge, 1);
        let (home, yolk, eggs) = setup_and_init_test_yolk()?;
        eggs.child("foo/foo.toml").write_str("")?;
        eggs.child("foo/thing/thing.toml").write_str("")?;
        yolk.sync_egg_deployment(&Egg::open(
            home.to_path_buf(),
            eggs.child("foo").to_path_buf(),
            EggConfig::default()
                .with_target("foo.toml", home.child("foo.toml"))
                .with_target("thing", home.child("thing")),
        )?)?;
        home.child("foo.toml").assert(is_symlink());
        home.child("thing").assert(is_symlink());

        // Verify stow-style usage works
        home.child(".config").create_dir_all()?;
        eggs.child("bar/.config/thing.toml").write_str("")?;
        yolk.sync_egg_deployment(&Egg::open(
            home.to_path_buf(),
            eggs.child("bar").to_path_buf(),
            EggConfig::new_merge(".", &home),
        )?)?;

        home.child(".config").assert(is_dir());
        home.child(".config/thing.toml").assert(is_symlink());
        Ok(())
    }

    #[test]
    fn test_deploy_put_mode() -> TestResult {
        cov_mark::check_count!(deploy_put_symlink_failed, 0);
        cov_mark::check_count!(deploy_put, 2);
        let (home, yolk, eggs) = setup_and_init_test_yolk()?;
        eggs.child("foo/foo.toml").write_str("")?;
        eggs.child("foo/thing/thing.toml").write_str("")?;
        yolk.sync_egg_deployment(&Egg::open(
            home.to_path_buf(),
            eggs.child("foo").to_path_buf(),
            EggConfig::default()
                .with_target("foo.toml", home.child("foo.toml"))
                .with_target("thing", home.child("thing"))
                .with_strategy(DeploymentStrategy::Put),
        )?)?;
        home.child("foo.toml").assert(is_symlink());
        home.child("thing").assert(is_symlink());
        Ok(())
    }

    #[test]
    fn test_deploy_put_mode_fails_with_stowy_usage() -> TestResult {
        cov_mark::check_count!(deploy_put, 1);
        let (home, yolk, eggs) = setup_and_init_test_yolk()?;
        home.child(".config").create_dir_all()?;
        eggs.child("bar/.config/thing.toml").write_str("")?;
        yolk.sync_egg_deployment(&Egg::open(
            home.to_path_buf(),
            eggs.child("bar").to_path_buf(),
            EggConfig::new_put(".", &home),
        )?)?;
        home.child(".config").assert(is_dir());
        home.child(".config/thing.toml").assert(exists().not());
        Ok(())
    }

    #[test]
    fn test_deploy_put_creates_parent_dir() -> TestResult {
        cov_mark::check_count!(deploy_put, 1);
        cov_mark::check_count!(deploy_put_symlink_failed, 0);
        let (home, yolk, eggs) = setup_and_init_test_yolk()?;
        eggs.child("foo/foo.toml").write_str("")?;
        yolk.sync_egg_deployment(&Egg::open(
            home.to_path_buf(),
            eggs.child("foo").to_path_buf(),
            EggConfig::new_put("foo.toml", home.child("a/a/a/foo.toml")),
        )?)?;
        home.child("a/a/a").assert(is_dir().and(is_symlink().not()));
        home.child("a/a/a/foo.toml").assert(is_symlink());
        Ok(())
    }

    #[test]
    fn test_undeploy() -> TestResult {
        cov_mark::check!(undeploy);
        let (home, yolk, eggs) = setup_and_init_test_yolk()?;
        home.child(".config").create_dir_all()?;
        eggs.child("foo/foo.toml").write_str("")?;
        eggs.child("bar/.config/thing.toml").write_str("")?;

        let mut egg = Egg::open(
            home.to_path_buf(),
            eggs.child("foo").to_path_buf(),
            EggConfig::new_merge("foo.toml", home.child("foo.toml")),
        )?;

        yolk.sync_egg_deployment(&egg)?;
        home.child("foo.toml").assert(is_symlink());

        egg.config_mut().enabled = false;
        yolk.sync_egg_deployment(&egg)?;
        home.child("foo.toml").assert(exists().not());

        // Verify stow-style usage works
        home.child(".config").create_dir_all()?;
        eggs.child("bar/.config/thing.toml").write_str("")?;
        let mut egg = Egg::open(
            home.to_path_buf(),
            eggs.child("bar").to_path_buf(),
            EggConfig::new_merge(".", &home),
        )?;
        yolk.sync_egg_deployment(&egg)?;
        home.child(".config/thing.toml").assert(is_symlink());
        egg.config_mut().enabled = false;
        yolk.sync_egg_deployment(&egg)?;
        home.child(".config/thing.toml").assert(exists().not());
        home.child(".config").assert(is_dir());
        Ok(())
    }

    /// Test that sync_egg_deployment after moving something in the egg dir and changing the deployment configuration,
    /// When encountering old, dead symlinks into the same egg, deletes those dead symlinks.
    #[test]
    fn test_deploy_after_moving_overrides_old_dead_symlinks() -> TestResult {
        cov_mark::check_count!(remove_dead_symlink, 1);

        let (home, yolk, eggs) = setup_and_init_test_yolk()?;
        // We start out with a stow-style situation, where we have eggs/alacritty/.config/alacritty/alacritty.toml
        home.child(".config").create_dir_all()?;
        eggs.child("alacritty/.config/alacritty/alacritty.toml")
            .write_str("")?;
        let mut egg = Egg::open(
            home.to_path_buf(),
            eggs.child("alacritty").to_path_buf(),
            EggConfig::new_merge(".", &home),
        )?;
        yolk.sync_egg_deployment(&egg)?;
        home.child(".config/alacritty").assert(is_symlink());

        // now we want to change to a simpler structure, where we explicitly declare the target dir for the files.
        // the user first moves the files inside the egg dir
        fs_err::rename(
            eggs.child("alacritty/.config/alacritty/alacritty.toml"),
            eggs.child("alacritty/alacritty.toml"),
        )?;
        // deletes the now empty .config structure
        fs_err::remove_dir(eggs.child("alacritty/.config/alacritty/"))?;
        fs_err::remove_dir(eggs.child("alacritty/.config/"))?;

        // He now updates his egg configuration to make the alacritty egg dir deploy to .config/alacritty
        egg.config_mut().targets = maplit::hashmap! {
            PathBuf::from(".") => PathBuf::from(".config/alacritty/")
        };
        // And syncs
        yolk.sync_egg_deployment(&egg)?;

        home.child(".config/alacritty").assert(is_symlink());
        Ok(())
    }

    #[test]
    fn test_syncing() -> TestResult {
        let (home, yolk, eggs) = setup_and_init_test_yolk()?;
        let foo_toml_initial = "{# data.value #}\nfoo";
        home.child("yolk/yolk.rhai").write_str(indoc::indoc! {r#"
            export const data = if LOCAL { #{value: "local"} } else { #{value: "canonical"} };
            export let eggs = #{foo: #{ targets: `~`, strategy: "merge"}};
        "#})?;
        eggs.child("foo/foo.toml").write_str(foo_toml_initial)?;
        yolk.sync_to_mode(EvalMode::Local)?;
        // No template set in eggs.rhai, so no templating should happen
        home.child("foo.toml").assert(is_symlink());
        eggs.child("foo/foo.toml").assert(foo_toml_initial);

        // Now we make the file a template, so it should be updated
        home.child("yolk/yolk.rhai").write_str(indoc::indoc! {r#"
            export const data = if LOCAL {#{value: "local"}} else {#{value: "canonical"}};
            export let eggs = #{foo: #{targets: `~`, templates: ["foo.toml"], strategy: "merge"}};
        "#})?;

        yolk.sync_to_mode(EvalMode::Local)?;
        eggs.child("foo/foo.toml").assert("{# data.value #}\nlocal");

        // Update the state, to see if applying again just works :tm:
        home.child("yolk/yolk.rhai").write_str(indoc::indoc! {r#"
            export const data = if LOCAL {#{value: "new local"}} else {#{value: "new canonical"}};
            export let eggs = #{foo: #{targets: `~`, templates: ["foo.toml"], strategy: "merge"}};
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

    #[test]
    fn test_access_sysinfo() -> TestResult {
        let (home, yolk, eggs) = setup_and_init_test_yolk()?;
        home.child("yolk/yolk.rhai").write_str(
            r#"
            export const hostname = SYSTEM.hostname;
            export let eggs = #{foo: #{targets: `~`, templates: ["foo.toml"]}};
        "#,
        )?;
        eggs.child("foo/foo.toml")
            .write_str("{< `host=${hostname}|${SYSTEM.hostname}` >}")?;
        yolk.sync_to_mode(EvalMode::Local)?;
        eggs.child("foo/foo.toml").assert(
            "host=canonical-hostname|canonical-hostname{< `host=${hostname}|${SYSTEM.hostname}` >}",
        );
        Ok(())
    }
}
