use fs_err::PathExt;
use miette::miette;
use miette::{Context, IntoDiagnostic, Result, Severity};

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::deploy::Deployer;
use crate::multi_error::MultiError;
use crate::{
    eggs_config::{DeploymentStrategy, EggConfig},
    script::{eval_ctx::EvalCtx, rhai_error::RhaiError, sysinfo::SystemInfo},
    templating::document::Document,
    util::{self, PathExt as _},
    yolk_paths::{Egg, YolkPaths},
};

const GITIGNORE_ENTRIES: &[&str] = &["/.git", "/.deployed_cache", "/.yolk_git"];

pub struct Yolk {
    yolk_paths: YolkPaths,
}

impl Yolk {
    pub fn new(yolk_paths: YolkPaths) -> Self {
        Self { yolk_paths }
    }

    /// Init the yolk directory, setting up the required git structure and files.
    ///
    /// `yolk_binary` is used as the path that git-filter uses when calling yolk to process the files.
    /// In most cases, the `yolk_binary` can be left to None.
    /// However, for tests, it should explicitly be provided to ensure that the correct yolk binary is being used.
    pub fn init_yolk(&self, yolk_binary: Option<&str>) -> Result<()> {
        self.yolk_paths.create()?;
        self.init_git_config(yolk_binary)?;
        Ok(())
    }

    #[tracing::instrument(skip_all, fields(yolk_dir = self.yolk_paths.root_path().abbr()))]
    pub fn init_git_config(&self, _yolk_binary: Option<&str>) -> Result<()> {
        // TODO: check if it's worthwhile to use the yolk-binary in some hooks.
        // if not, we can remove the yolk_binary argument
        if !self.yolk_paths.root_path().exists() {
            miette::bail!("Yolk directory is not initialized. Please run `yolk init` first.");
        }
        tracing::trace!("Ensuring git repo is initialized");

        if self.yolk_paths.active_yolk_git_dir().is_err() {
            std::process::Command::new("git")
                .arg("init")
                .current_dir(self.yolk_paths.root_path())
                .status()
                .into_diagnostic()?;
            self.yolk_paths.safeguard_git_dir()?;
        }
        tracing::trace!("Ensuring that git config is properly set up");
        util::ensure_file_contains_lines(
            self.paths().root_path().join(".gitignore"),
            GITIGNORE_ENTRIES,
        )
        .context("Failed to ensure .gitignore is configured correctly")?;

        // Remove git-filter configuration from gitattributes
        util::ensure_file_doesnt_contain_lines(
            self.paths().root_path().join(".gitattributes"),
            &["* filter=yolk"],
        )
        .context("Failed to clean up .gitattributes")?;

        Ok(())
    }

    pub fn paths(&self) -> &YolkPaths {
        &self.yolk_paths
    }

    /// Deploy a given [`Egg`]
    #[tracing::instrument(skip_all, fields(egg = ?egg.name()))]
    fn deploy_egg(
        &self,
        deployer: &mut Deployer,
        egg: &Egg,
        mappings: &HashMap<PathBuf, PathBuf>,
    ) -> Result<(), MultiError> {
        let mut errs = Vec::new();
        egg.config().unsafe_shell_hooks.run_pre_deploy()?;
        for (in_egg, deployed) in mappings {
            let mut deploy_mapping = || -> miette::Result<()> {
                match egg.config().strategy {
                    DeploymentStrategy::Merge => {
                        cov_mark::hit!(deploy_merge);
                        deployer.symlink_recursive(egg.path(), in_egg, deployed)?;
                    }
                    DeploymentStrategy::Put => {
                        cov_mark::hit!(deploy_put);
                        if deployed.is_symlink() {
                            let target = deployed.fs_err_read_link().into_diagnostic()?;
                            if target.starts_with(egg.path()) {
                                deployer.delete_symlink(deployed)?;
                                tracing::info!("Removed dead symlink {}", deployed.abbr());
                            }
                        }
                        if let Some(parent) = deployed.parent() {
                            fs_err::create_dir_all(parent).map_err(|e| {
                                miette!(
                                    severity = Severity::Warning,
                                    "Failed to create parent dir for deployment of {}: {e:?}",
                                    in_egg.abbr()
                                )
                            })?;
                        }
                        deployer.create_symlink(in_egg, deployed)?;
                    }
                }
                Result::Ok(())
            };
            if let Err(e) = deploy_mapping() {
                errs.push(e.wrap_err(format!("Failed to deploy {}", in_egg.abbr())));
            }
        }

        if !errs.is_empty() {
            return Err(MultiError::new(
                format!("Failed to deploy egg {}", egg.name()),
                errs,
            ));
        }
        debug_assert!(
            !errs.is_empty() || egg.is_deployed()?,
            "Egg::is_deployed should return true after deploying"
        );
        egg.config().unsafe_shell_hooks.run_post_deploy()?;
        Ok(())
    }

    fn undeploy_egg(
        &self,
        deployer: &mut Deployer,
        egg: &Egg,
        mappings: &HashMap<PathBuf, PathBuf>,
    ) -> Result<(), MultiError> {
        egg.config().unsafe_shell_hooks.run_pre_undeploy()?;
        let mut errs = Vec::new();
        for (in_egg, deployed) in mappings {
            if let Err(e) = deployer.remove_symlink_recursive(in_egg, &deployed) {
                errs.push(e.wrap_err(format!("Failed to remove deployment of {}", in_egg.abbr())));
            }
        }
        if !errs.is_empty() {
            return Err(MultiError::new(
                format!("Failed to undeploy egg {}", egg.name()),
                errs,
            ));
        }
        debug_assert!(
            !errs.is_empty() || !egg.is_deployed()?,
            "Egg::is_deployed should return false after undeploying"
        );
        egg.config().unsafe_shell_hooks.run_post_undeploy()?;

        Ok(())
    }

    /// Deploy or undeploy the given egg, depending on the current system state and the given Egg data.
    /// Returns true if the egg is now deployed, false if it is not.
    #[tracing::instrument(skip_all, fields(egg.name = %egg.name()))]
    pub fn sync_egg_deployment(&self, egg: &Egg) -> Result<bool, MultiError> {
        let deployed = egg
            .is_deployed()
            .with_context(|| format!("Failed to check deployment state for egg {}", egg.name()))?;
        tracing::debug!(
            egg.name = egg.name(),
            egg.deployed = deployed,
            egg.enabled = egg.config().enabled,
            "Checking egg deployment"
        );
        let mappings = egg
            .config()
            .targets_expanded(self.yolk_paths.home_path(), egg.path())
            .context("Failed to expand targets config for egg")?;

        if egg.config().enabled && !deployed {
            let mut deployer = Deployer::new();
            tracing::debug!("Deploying egg {}", egg.name());

            let result = self.deploy_egg(&mut deployer, egg, &mappings);
            if result.is_ok() {
                tracing::info!("Successfully deployed egg {}", egg.name());
            }

            let deployed_symlinks = deployer.created_symlinks().clone();
            if let Err(e) =
                self.cleanup_stale_symlinks_for(&mut deployer, egg.name(), &deployed_symlinks)
            {
                tracing::error!("{e:?}");
            }
            deployer.try_run_elevated()?;
            result.map(|()| true)
        } else if !egg.config().enabled && deployed {
            let mut deployer = Deployer::new();
            cov_mark::hit!(undeploy);
            tracing::debug!("Removing egg {}", egg.name());
            let result = self.undeploy_egg(&mut deployer, egg, &mappings);
            if result.is_ok() {
                tracing::info!("Successfully undeployed egg {}", egg.name());
            }

            deployer.try_run_elevated()?;
            let mut deployer = Deployer::new();
            if let Err(e) = self.cleanup_stale_symlinks_for(&mut deployer, egg.name(), &[]) {
                tracing::error!("{e:?}");
            }
            deployer.try_run_elevated()?;

            result.map(|()| false)
        } else {
            Ok(deployed)
        }
    }

    /// Check through the old symlinks from the cache file of a given egg,
    /// and remove any that are not included in the `deployed_symlinks` list.
    pub fn cleanup_stale_symlinks_for(
        &self,
        deployer: &mut Deployer,
        egg_name: &str,
        deployed_symlinks: &[PathBuf],
    ) -> Result<(), MultiError> {
        let mut errs = Vec::new();
        let old_symlinks_db = self.yolk_paths.previous_egg_deployment_locations_db()?;
        let old_symlinks = old_symlinks_db.read(egg_name)?;

        for old_symlink in old_symlinks {
            // TODO: This is very,.... reliant on the fact that paths are normalized.
            // Should be the case, but can we enforce this somehow?
            if !deployed_symlinks.contains(&old_symlink) {
                let is_symlink_to_egg = if old_symlink.exists() && old_symlink.is_symlink() {
                    match old_symlink.fs_err_read_link() {
                        Ok(x) => x.starts_with(self.paths().egg_path(egg_name)),
                        Err(e) => {
                            errs.push(miette::Report::from_err(e));
                            false
                        }
                    }
                } else {
                    false
                };
                if is_symlink_to_egg {
                    tracing::info!("Removing stale symlink at {}", old_symlink.abbr());
                    cov_mark::hit!(delete_stale_symlink);
                    if let Err(e) = deployer.delete_symlink(&old_symlink) {
                        errs.push(e.wrap_err(format!(
                            "Failed to remove old symlink {}",
                            old_symlink.abbr()
                        )));
                    }
                }
            }
        }
        old_symlinks_db.write(egg_name, deployed_symlinks)?;

        Ok(())
    }

    /// fetch the `eggs` variable from a given EvalCtx.
    pub fn load_egg_configs(&self, eval_ctx: &mut EvalCtx) -> Result<HashMap<String, EggConfig>> {
        let (_, yolk_file_module) = eval_ctx
            .yolk_file_module()
            .expect("Tried to load egg configs before loading yolk file. This is a bug.");
        let eggs_map = yolk_file_module
            .get_var_value::<rhai::Map>("eggs")
            .ok_or_else(|| miette!("Could not find an `eggs` variable in scope"))?;
        Ok(eggs_map
            .into_iter()
            .map(|(x, v)| Ok((x.into(), EggConfig::from_dynamic(v)?)))
            .collect::<Result<HashMap<_, _>, RhaiError>>()?)
    }

    /// First, sync the deployment of all eggs to the local system.
    /// Then, update any templated files in the eggs to the given mode.
    #[tracing::instrument(skip_all, fields(?mode, %update_deployments))]
    pub fn sync_to_mode(&self, mode: EvalMode, update_deployments: bool) -> Result<(), MultiError> {
        tracing::debug!("Syncing eggs to {mode:?}");
        let mut eval_ctx = self.prepare_eval_ctx_for_templates(mode)?;

        let mut errs = Vec::new();
        let egg_configs = self.load_egg_configs(&mut eval_ctx)?;

        for (name, egg_config) in egg_configs.into_iter() {
            if let Err(e) = self
                .sync_egg_to_mode(&mut eval_ctx, &name, egg_config, update_deployments)
                .wrap_err_with(|| format!("Failed to sync egg `{name}`"))
            {
                errs.push(e);
            }
        }
        if errs.is_empty() {
            Ok(())
        } else {
            Err(MultiError::new("Failed to sync some eggs", errs))
        }
    }

    #[tracing::instrument(skip_all, fields(%name, %sync_deployment, ?egg_config))]
    fn sync_egg_to_mode(
        &self,
        eval_ctx: &mut EvalCtx,
        name: &str,
        egg_config: EggConfig,
        sync_deployment: bool,
    ) -> Result<()> {
        let egg = self.yolk_paths.get_egg(name, egg_config)?;
        if sync_deployment {
            self.sync_egg_deployment(&egg)?;
        }
        let templates_expanded = egg.config().templates_globexpanded(egg.path())?;
        for tmpl_path in templates_expanded {
            if tmpl_path.is_file() {
                self.sync_template_file(eval_ctx, &tmpl_path)?;
            } else if !tmpl_path.exists() {
                tracing::warn!(
                    "{} was specified as templated file, but doesn't exist",
                    tmpl_path.abbr()
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
        eval_ctx.set_module_path(self.yolk_paths.root_path());
        let yolk_file = fs_err::read_to_string(self.yolk_paths.yolk_rhai_path())
            .into_diagnostic()
            .context("Failed to read yolk.rhai")?;

        eval_ctx.set_global("SYSTEM", sysinfo);
        eval_ctx.set_global("LOCAL", mode == EvalMode::Local);
        eval_ctx.load_rhai_file_to_module(&yolk_file).map_err(|e| {
            e.into_report(
                self.yolk_paths.yolk_rhai_path().to_string_lossy(),
                yolk_file,
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
        tracing::debug!("Syncing file {}", path.abbr());
        let content = fs_err::read_to_string(path).into_diagnostic()?;
        let rendered = self
            .eval_template(eval_ctx, &path.to_string_lossy(), &content)
            .with_context(|| format!("Failed to eval template file: {}", path.abbr()))?;
        if rendered == content {
            tracing::debug!("No changes needed in {}", path.abbr());
            return Ok(());
        }
        fs_err::write(path, rendered).into_diagnostic()?;
        tracing::info!("Synced templated file {}", path.abbr());
        Ok(())
    }

    /// Run a given closure with all templates in their canonical state.
    ///
    /// First syncs them to canonical then runs the closure, then syncs them back to local.
    pub fn with_canonical_state<T>(&self, f: impl FnOnce() -> Result<T>) -> Result<T> {
        tracing::info!("Converting all templates into their canonical state");
        self.sync_to_mode(EvalMode::Canonical, false)?;
        let result = f();
        tracing::info!("Converting all templates back to the local state");
        self.sync_to_mode(EvalMode::Local, false)?;
        result
    }

    /// Run the yolk.rhai script, load the egg configs and return a list of all eggs.
    pub fn list_eggs(&self) -> Result<Vec<Egg>> {
        let mut eval_ctx = self.prepare_eval_ctx_for_templates(EvalMode::Local)?;
        let egg_configs = self.load_egg_configs(&mut eval_ctx)?;
        let eggs: Vec<Egg> = egg_configs
            .into_iter()
            .map(|(name, config)| self.yolk_paths.get_egg(&name, config))
            .collect::<Result<_>>()
            .context("Failed to find egg that was configured in yolk.rhai")?;
        Ok(eggs)
    }

    /// Run the yolk.rhai script, load the egg configs and return a list of all template file paths.
    pub fn list_templates(&self) -> Result<Vec<PathBuf>> {
        let mut eval_ctx = self.prepare_eval_ctx_for_templates(EvalMode::Local)?;
        let egg_configs = self.load_egg_configs(&mut eval_ctx)?;
        let template_paths: Vec<PathBuf> = egg_configs
            .into_iter()
            .map(|(name, config)| {
                self.yolk_paths
                    .get_egg(&name, config)
                    .with_context(|| format!("Failed to find egg dir for configured egg {}", name))
            })
            .map(|egg| {
                egg.and_then(|egg| {
                    egg.config()
                        .templates_globexpanded(egg.path())
                        .with_context(|| {
                            format!("Failed to globexpand template dirs for egg {}", egg.name())
                        })
                })
            })
            .collect::<Result<Vec<Vec<PathBuf>>>>()?
            .into_iter()
            .flatten()
            .collect();
        Ok(template_paths)
    }

    /// Run the yolk.rhai script, load the egg configs and return the requested egg.
    pub fn load_egg(&self, egg_name: &str) -> Result<Egg> {
        let mut eval_ctx = self.prepare_eval_ctx_for_templates(EvalMode::Local)?;
        let mut egg_configs = self.load_egg_configs(&mut eval_ctx)?;
        egg_configs
            .remove(egg_name)
            .ok_or_else(|| miette!("No egg with name {egg_name}"))
            .and_then(|config| self.yolk_paths.get_egg(egg_name, config))
    }

    pub fn validate_config_invariants(&self) -> Result<()> {
        let mut eval_ctx_local = self.prepare_eval_ctx_for_templates(EvalMode::Local)?;
        let local_egg_configs = self.load_egg_configs(&mut eval_ctx_local)?;
        let mut eval_ctx_canonical = self.prepare_eval_ctx_for_templates(EvalMode::Canonical)?;
        let canonical_egg_configs = self.load_egg_configs(&mut eval_ctx_canonical)?;
        // When listing eggs, let's ensure that the eggs directory exactly matches the configured eggs.
        let eggs_dir_entries = self
            .yolk_paths
            .eggs_dir_path()
            .fs_err_read_dir()
            .into_diagnostic()?;
        let mut count = 0;
        for dir in eggs_dir_entries {
            count += 1;
            let dir = dir.into_diagnostic()?;
            miette::ensure!(
                local_egg_configs.contains_key(&dir.file_name().to_string_lossy().to_string()),
                "Egg {} is not configured in local yolk.rhai",
                dir.file_name().to_string_lossy()
            );
            miette::ensure!(
                canonical_egg_configs.contains_key(&dir.file_name().to_string_lossy().to_string()),
                "Egg {} is not configured in canonical yolk.rhai",
                dir.file_name().to_string_lossy()
            );
        }
        miette::ensure!(
            local_egg_configs.len() == canonical_egg_configs.len() ,
            help = "Always configure all eggs regardless of the LOCAL/CANONICAL state. Use the `enabled` field in the egg config to toggle eggs on and off instead.",
            "canonical and local version of yolk.rhai have different egg configurations.",
        );
        miette::ensure!(
            count == local_egg_configs.len(),
            "Not all configured eggs were found in the eggs directory"
        );
        for (name, local_config) in local_egg_configs {
            let canonical_config = canonical_egg_configs
                .get(&name)
                .ok_or_else(|| miette!("Egg {name} was not found in canonical yolk.rhai"))?;
            miette::ensure!(
                local_config.templates == canonical_config.templates ,
                help = "Make sure that template configuration does not depend on the LOCAL or CANONICAL mode",
                "Egg {name} has a different set of templated files in canonical mode compared to local mode"
            );
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalMode {
    Local,
    Canonical,
}
