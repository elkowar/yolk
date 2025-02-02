use normalize_path::NormalizePath;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    str::FromStr,
};

use miette::{miette, IntoDiagnostic as _};
use rhai::Dynamic;

use crate::{script::rhai_error::RhaiError, util::PathExt as _};

macro_rules! rhai_error {
    ($($tt:tt)*) => {
        RhaiError::Other(miette!($($tt)*))
    };
}

/// How the contents of an egg should be deployed.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum DeploymentStrategy {
    /// Recursively traverse the directory structure until a directory / file doesn't exist yet, then symlink there.
    /// This allows stow-like behavior.
    Merge,
    /// Simply deploy to the given target, or fail.
    #[default]
    Put,
}

impl FromStr for DeploymentStrategy {
    type Err = miette::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "merge" => Ok(DeploymentStrategy::Merge),
            "put" => Ok(DeploymentStrategy::Put),
            _ => miette::bail!(
                help = "strategy must be one of 'merge' or 'put'",
                "Invalid deployment strategy {}",
                s
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EggConfig {
    /// The targets map is a map from `path-relative-to-egg-dir` -> `path-where-it-should-go`.
    pub targets: HashMap<PathBuf, PathBuf>,
    pub enabled: bool,
    pub templates: HashSet<PathBuf>,
    /// The "main" file of this egg -- currently used to determine which path should be opened by `yolk edit`.
    pub main_file: Option<PathBuf>,
    pub strategy: DeploymentStrategy,
}

impl Default for EggConfig {
    fn default() -> Self {
        EggConfig {
            enabled: true,
            targets: HashMap::new(),
            templates: HashSet::new(),
            main_file: None,
            strategy: Default::default(),
        }
    }
}

impl EggConfig {
    pub fn new(in_egg: impl AsRef<Path>, deployed_to: impl AsRef<Path>) -> Self {
        let in_egg = in_egg.as_ref();
        EggConfig {
            enabled: true,
            targets: maplit::hashmap! {
                in_egg.to_path_buf() => deployed_to.as_ref().to_path_buf()
            },
            templates: HashSet::new(),
            main_file: None,
            strategy: DeploymentStrategy::default(),
        }
    }
    pub fn new_merge(in_egg: impl AsRef<Path>, deployed_to: impl AsRef<Path>) -> Self {
        Self::new(in_egg, deployed_to).with_strategy(DeploymentStrategy::Merge)
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_template(mut self, template: impl AsRef<Path>) -> Self {
        self.templates.insert(template.as_ref().to_path_buf());
        self
    }

    pub fn with_strategy(mut self, strategy: DeploymentStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn with_main_file(mut self, main_file: impl AsRef<Path>) -> Self {
        self.main_file = Some(main_file.as_ref().to_path_buf());
        self
    }

    /// Add a new target from a path inside the egg dir to the path it should be deployed as.
    pub fn with_target(mut self, in_egg: impl AsRef<Path>, deploy_to: impl AsRef<Path>) -> Self {
        self.targets.insert(
            in_egg.as_ref().to_path_buf(),
            deploy_to.as_ref().to_path_buf(),
        );
        self
    }

    /// Returns the targets map, but with any `~` expanded to the home directory.
    ///
    /// The targets map is a map from `path-relative-to-egg-dir` -> `path-where-it-should-go`.
    pub fn targets_expanded(
        &self,
        home: impl AsRef<Path>,
        egg_root: impl AsRef<Path>,
    ) -> miette::Result<HashMap<PathBuf, PathBuf>> {
        let egg_root = egg_root.as_ref();
        self.targets
            .iter()
            .map(|(source, target)| {
                let source = egg_root.canonical()?.join(source);
                let target = target.expanduser();
                let target = if target.is_absolute() {
                    target
                } else {
                    home.as_ref().join(target)
                };
                Ok((source.normalize(), target.normalize()))
            })
            .collect()
    }

    /// Expand the glob patterns in the `templates` field to a list of paths.
    /// The globbed paths are considered relative to `in_dir`. The resulting list of paths will contain absolute paths.
    pub fn templates_globexpanded(&self, in_dir: impl AsRef<Path>) -> miette::Result<Vec<PathBuf>> {
        let in_dir = in_dir.as_ref();
        let mut paths = Vec::new();
        for globbed in &self.templates {
            let expanded = glob::glob(&in_dir.join(globbed).to_string_lossy()).into_diagnostic()?;
            for path in expanded {
                paths.push(path.into_diagnostic()?);
            }
        }
        Ok(paths)
    }

    pub fn from_dynamic(value: Dynamic) -> Result<Self, RhaiError> {
        if let Ok(target_path) = value.as_immutable_string_ref() {
            return Ok(EggConfig::new(".", target_path.to_string()));
        }
        let Ok(map) = value.as_map_ref() else {
            return Err(rhai_error!("egg value must be a string or a map"));
        };
        let empty_map = Dynamic::from(rhai::Map::new());
        let targets = map.get("targets").unwrap_or(&empty_map);

        let targets = if let Ok(targets) = targets.as_immutable_string_ref() {
            maplit::hashmap! { PathBuf::from(".") => PathBuf::from(targets.to_string()) }
        } else if let Ok(targets) = targets.as_map_ref() {
            targets
                .clone()
                .into_iter()
                .map(|(k, v)| {
                    Ok::<_, RhaiError>((
                        PathBuf::from(&*k),
                        PathBuf::from(&v.into_string().map_err(|e| {
                            rhai_error!("target file value must be a path, but got {e}")
                        })?),
                    ))
                })
                .collect::<Result<_, _>>()?
        } else {
            return Err(rhai_error!("egg `targets` must be a string or a map"));
        };

        let main_file = match map.get("main_file") {
            Some(path) => Some(
                path.as_immutable_string_ref()
                    .map_err(|e| rhai_error!("main_file must be a path, but got {e}"))?
                    .to_string()
                    .into(),
            ),
            None => None,
        };

        let strategy = match map.get("strategy") {
            Some(strategy) => {
                DeploymentStrategy::from_str(&strategy.to_string()).map_err(RhaiError::Other)?
            }
            None => DeploymentStrategy::default(),
        };

        let templates =
            if let Some(templates) = map.get("templates") {
                templates
                    .as_array_ref()
                    .map_err(|t| rhai_error!("`templates` must be a list, but got {t}"))?
                    .iter()
                    .map(|x| {
                        Ok::<_, RhaiError>(PathBuf::from(x.clone().into_string().map_err(|e| {
                            rhai_error!("template entry must be a path, but got {e}")
                        })?))
                    })
                    .collect::<Result<HashSet<_>, _>>()?
            } else {
                HashSet::new()
            };

        let enabled = if let Some(x) = map.get("enabled") {
            x.as_bool()
                .map_err(|t| rhai_error!("`enabled` must be a list, but got {t}"))?
        } else {
            true
        };
        Ok(EggConfig {
            targets,
            enabled,
            templates,
            main_file,
            strategy,
        })
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use assert_fs::{
        prelude::{FileWriteStr as _, PathChild as _},
        TempDir,
    };
    use maplit::hashset;
    use miette::IntoDiagnostic as _;
    use pretty_assertions::assert_eq;

    use crate::{
        eggs_config::{DeploymentStrategy, EggConfig},
        util::test_util::TestResult,
    };

    use rstest::rstest;

    #[rstest]
    #[case(
        indoc::indoc! {r#"
            #{
                enabled: false,
                targets: #{ "foo": "~/bar" },
                templates: ["foo"],
                main_file: "foo",
                strategy: "merge",
            }
        "#},
        EggConfig::new_merge("foo", "~/bar")
            .with_enabled(false)
            .with_template("foo")
            .with_strategy(DeploymentStrategy::Merge)
            .with_main_file("foo")
    )]
    #[case(r#"#{ targets: "~/bar" }"#, EggConfig::new(".", "~/bar"))]
    #[case(r#""~/bar""#, EggConfig::new(".", "~/bar"))]
    fn test_read_eggs_config(#[case] input: &str, #[case] expected: EggConfig) -> TestResult {
        let result = rhai::Engine::new().eval(input)?;
        assert_eq!(EggConfig::from_dynamic(result)?, expected);
        Ok(())
    }

    #[test]
    fn test_template_globbed() -> TestResult {
        let home = TempDir::new().into_diagnostic()?;
        let config = EggConfig::new_merge(home.to_str().unwrap(), ".")
            .with_template("foo")
            .with_template("**/*.foo");
        home.child("foo").write_str("a")?;
        home.child("bar/baz/a.foo").write_str("a")?;
        home.child("bar/a.foo").write_str("a")?;
        home.child("bar/foo").write_str("a")?;
        let result = config.templates_globexpanded(&home)?;

        assert_eq!(
            result.into_iter().collect::<HashSet<_>>(),
            hashset![
                home.child("foo").path().to_path_buf(),
                home.child("bar/baz/a.foo").path().to_path_buf(),
                home.child("bar/a.foo").path().to_path_buf(),
            ]
        );
        Ok(())
    }
}
