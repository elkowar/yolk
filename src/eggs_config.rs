use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use miette::{miette, IntoDiagnostic as _};
use rhai::Dynamic;

use crate::script::lua_error::RhaiError;

macro_rules! rhai_error {
    ($($tt:tt)*) => {
        RhaiError::Other(miette!($($tt)*))
    };
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EggConfig {
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
    pub fn new(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Self {
        EggConfig {
            enabled: true,
            targets: maplit::hashmap! {
                from.as_ref().to_path_buf() => to.as_ref().to_path_buf()
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

    pub fn with_target(mut self, from: impl AsRef<Path>, to: impl AsRef<Path>) -> Self {
        self.targets
            .insert(from.as_ref().to_path_buf(), to.as_ref().to_path_buf());
        self
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
}
