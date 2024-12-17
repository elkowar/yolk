use expanduser::expanduser;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use miette::{miette, IntoDiagnostic as _};
use rhai::Dynamic;

use crate::{script::rhai_error::RhaiError, util::PathExt as _};

macro_rules! rhai_error {
    ($($tt:tt)*) => {
        RhaiError::Other(miette!($($tt)*))
    };
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EggConfig {
    /// The targets map is a map from `path-relative-to-egg-dir` -> `path-where-it-should-go`.
    pub targets: HashMap<PathBuf, PathBuf>,
    pub enabled: bool,
    pub templates: HashSet<PathBuf>,
}

impl Default for EggConfig {
    fn default() -> Self {
        EggConfig {
            enabled: true,
            targets: HashMap::new(),
            templates: HashSet::new(),
        }
    }
}

impl EggConfig {
    pub fn new(in_egg: impl AsRef<Path>, deployed_to: impl AsRef<Path>) -> Self {
        EggConfig {
            enabled: true,
            targets: maplit::hashmap! {
                in_egg.as_ref().to_path_buf() => deployed_to.as_ref().to_path_buf()
            },
            templates: HashSet::new(),
        }
    }
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_template(mut self, template: impl AsRef<Path>) -> Self {
        self.templates.insert(template.as_ref().to_path_buf());
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
                let target = expanduser(target.to_string_lossy()).into_diagnostic()?;
                let target = if target.is_absolute() {
                    target
                } else {
                    home.as_ref().join(target)
                };
                Ok((source, target))
            })
            .collect()
    }

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
        if let Ok(target_path) = value.clone().into_string() {
            return Ok(EggConfig {
                enabled: true,
                targets: maplit::hashmap! {
                    PathBuf::from(".") => PathBuf::from(&*target_path)
                },
                templates: HashSet::new(),
            });
        }
        let Ok(map) = value.as_map_ref() else {
            return Err(rhai_error!("egg value must be a string or a map"));
        };
        let targets = map
            .get("targets")
            .ok_or_else(|| rhai_error!("egg table must contain a 'target' key"))?;

        let targets = if let Ok(targets) = targets.clone().into_immutable_string() {
            maplit::hashmap! { PathBuf::from(".") => PathBuf::from(&*targets) }
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

        let enabled = map
            .get("enabled")
            .map(|x| {
                x.as_bool()
                    .map_err(|t| rhai_error!("`enabled` must be a list, but got {t}"))
            })
            .transpose()?
            .unwrap_or(true);

        Ok(EggConfig {
            targets,
            enabled,
            templates,
        })
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashSet, path::PathBuf};

    use assert_fs::{
        prelude::{FileWriteStr as _, PathChild as _},
        TempDir,
    };
    use maplit::hashset;
    use miette::IntoDiagnostic as _;

    use crate::{eggs_config::EggConfig, util::TestResult};

    #[test]
    fn test_read_verbose_eggs_config() -> TestResult {
        let result = rhai::Engine::new().eval(indoc::indoc! {r#"
            #{
                enabled: false,
                targets: #{ "foo": "~/bar" },
                templates: ["foo"]
            }
        "#})?;
        assert_eq!(
            EggConfig::from_dynamic(result)?,
            EggConfig {
                enabled: false,
                targets: maplit::hashmap! {
                    PathBuf::from("foo") => PathBuf::from("~/bar")
                },
                templates: maplit::hashset! {
                    PathBuf::from("foo")
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_read_simple_eggs_config() -> TestResult {
        let result = rhai::Engine::new().eval(r#"#{ targets: "~/bar" }"#)?;
        assert_eq!(
            EggConfig::from_dynamic(result).unwrap(),
            EggConfig {
                enabled: true,
                targets: maplit::hashmap! {
                    PathBuf::from(".") => PathBuf::from("~/bar")
                },
                templates: HashSet::new(),
            }
        );
        Ok(())
    }

    #[test]
    fn test_read_minimal_eggs_config() -> TestResult {
        let result = rhai::Engine::new().eval(r#""~/bar""#)?;
        assert_eq!(
            EggConfig::from_dynamic(result)?,
            EggConfig {
                enabled: true,
                targets: maplit::hashmap! {
                    PathBuf::from(".") => PathBuf::from("~/bar")
                },
                templates: HashSet::new(),
            }
        );
        Ok(())
    }

    #[test]
    fn test_template_globbed() -> TestResult {
        let home = TempDir::new().into_diagnostic()?;
        let config = EggConfig::new(home.to_str().unwrap(), ".")
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
