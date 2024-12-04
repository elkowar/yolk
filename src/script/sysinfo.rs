use std::path::PathBuf;

use mlua::IntoLua;

#[derive(Debug, Clone)]
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

impl IntoLua for SystemInfo {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        table.set("hostname", self.hostname)?;
        table.set("username", self.username)?;
        table.set("paths", self.paths)?;
        table.set("distro", self.distro)?;
        table.set("device_name", self.device_name)?;
        table.set("arch", self.arch)?;
        table.set("desktop_env", self.desktop_env)?;
        table.set("platform", self.platform)?;
        Ok(mlua::Value::Table(table))
    }
}

#[derive(Debug, Clone)]
pub struct SystemInfoPaths {
    cache_dir: Option<PathBuf>,
    config_dir: Option<PathBuf>,
    home_dir: Option<PathBuf>,
    yolk_dir: PathBuf,
}

impl IntoLua for SystemInfoPaths {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        table.set("cache_dir", self.cache_dir)?;
        table.set("config_dir", self.config_dir)?;
        table.set("home_dir", self.home_dir)?;
        table.set("yolk_dir", self.yolk_dir)?;
        Ok(mlua::Value::Table(table))
    }
}

impl SystemInfo {
    pub fn generate() -> Self {
        #[cfg(test)]
        return Self::canonical();
        // lmao make this not garbage
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
