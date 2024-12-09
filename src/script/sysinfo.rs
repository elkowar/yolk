use std::path::PathBuf;

use mlua::{IntoLua, LuaSerdeExt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemInfo {
    hostname: Option<String>,
    username: String,
    distro: String,
    device_name: Option<String>,
    arch: String,
    desktop_env: String,
    platform: String,
    paths: SystemInfoPaths,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemInfoPaths {
    cache_dir: Option<PathBuf>,
    config_dir: Option<PathBuf>,
    home_dir: Option<PathBuf>,
    yolk_dir: PathBuf,
}

impl IntoLua for SystemInfo {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self)
    }
}

impl IntoLua for SystemInfoPaths {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self)
    }
}

impl SystemInfo {
    pub fn generate() -> Self {
        #[cfg(test)]
        return Self::canonical();
        #[cfg(not(test))]
        Self {
            hostname: whoami::fallible::hostname().ok(),
            username: whoami::username(),
            distro: whoami::distro().to_string(),
            device_name: whoami::fallible::devicename().ok(),
            arch: whoami::arch().to_string(),
            desktop_env: whoami::desktop_env().to_string(),
            platform: whoami::platform().to_string(),
            paths: SystemInfoPaths {
                cache_dir: dirs::cache_dir(),
                config_dir: dirs::config_dir(),
                home_dir: dirs::home_dir(),
                yolk_dir: default_yolk_dir(),
            },
        }
    }

    pub fn canonical() -> Self {
        Self {
            hostname: Some("canonical-hostname".to_string()),
            username: "canonical-username".to_string(),
            paths: SystemInfoPaths {
                cache_dir: Some(PathBuf::from("/canonical/cache")),
                config_dir: Some(PathBuf::from("/canonical/config")),
                home_dir: Some(PathBuf::from("/canonical/home")),
                yolk_dir: PathBuf::from("/canonical/yolk"),
            },
            distro: "distro".to_string(),
            device_name: None,
            arch: "x86_64".to_string(),
            desktop_env: "gnome".to_string(),
            platform: "linux".to_string(),
        }
    }
}

#[allow(unused)]
fn default_yolk_dir() -> PathBuf {
    dirs::config_dir()
        .expect("No config dir found")
        .join("yolk")
}
