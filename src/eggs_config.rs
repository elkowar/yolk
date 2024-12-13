use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use bon::Builder;
use miette::miette;
use mlua::{FromLua, Table, Value};

macro_rules! mlua_miette {
    ($($tt:tt)*) => {
        mlua::Error::external(miette!($($tt)*))
    };
}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct EggConfig {
    #[builder(default = true)]
    pub enabled: bool,
    pub targets: HashMap<PathBuf, PathBuf>,
    #[builder(default)]
    pub templates: HashSet<PathBuf>,
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

    pub fn stow_like(home: impl AsRef<Path>) -> Self {
        Self::new(".", home)
    }
}

impl FromLua for EggConfig {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        if let Some(target_path) = value.as_string() {
            return Ok(EggConfig {
                enabled: true,
                targets: maplit::hashmap! {
                    PathBuf::from(".") => PathBuf::from(target_path.to_string_lossy())
                },
                templates: HashSet::new(),
            });
        }
        let Some(table) = value.as_table() else {
            return Err(mlua_miette!("egg value must be a string or a table"));
        };
        let targets = table
            .get::<Value>("targets")
            .map_err(|_| mlua_miette!("egg table must contain a 'target' key"))?;

        let targets = if let Some(targets) = targets.as_string() {
            maplit::hashmap! { PathBuf::from(".") => PathBuf::from(targets.to_string_lossy()) }
        } else if let Some(targets) = targets.as_table() {
            targets
                .pairs::<String, String>()
                .map(|x| {
                    let (k, v) = x?;
                    mlua::Result::Ok((PathBuf::from(k), PathBuf::from(v)))
                })
                .collect::<Result<_, _>>()?
        } else {
            return Err(mlua_miette!("egg 'targets' must be a string or a table"));
        };

        let templates = if table.contains_key("templates")? {
            table
                .get::<Table>("templates")
                .and_then(|templates| {
                    templates
                        .sequence_values::<String>()
                        .map(|x| x.map(PathBuf::from))
                        .collect::<Result<HashSet<_>, _>>()
                })
                .unwrap_or_default()
        } else {
            HashSet::new()
        };

        let enabled = dbg!(table.get::<Option<bool>>("enabled"))?.unwrap_or(true);

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

    use crate::eggs_config::EggConfig;

    #[test]
    fn test_read_verbose_eggs_config() {
        let lua = mlua::Lua::new();
        let config = lua
            .load(indoc::indoc! {r#"
                {
                    enabled = false,
                    targets = { ["foo"] = "~/bar" },
                    templates = { "foo" }
                }
            "#})
            .eval::<EggConfig>()
            .unwrap();
        assert_eq!(
            config,
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
    }

    #[test]
    fn test_read_simple_eggs_config() {
        let lua = mlua::Lua::new();
        let config = lua
            .load(r#"{ targets = "~/bar" }"#)
            .eval::<EggConfig>()
            .unwrap();
        assert_eq!(
            config,
            EggConfig {
                enabled: true,
                targets: maplit::hashmap! {
                    PathBuf::from(".") => PathBuf::from("~/bar")
                },
                templates: HashSet::new(),
            }
        );
    }
    #[test]
    fn test_read_minimal_eggs_config() {
        let lua = mlua::Lua::new();
        let config = lua.load(r#""~/bar""#).eval::<EggConfig>().unwrap();
        assert_eq!(
            config,
            EggConfig {
                enabled: true,
                targets: maplit::hashmap! {
                    PathBuf::from(".") => PathBuf::from("~/bar")
                },
                templates: HashSet::new(),
            }
        );
    }
}
