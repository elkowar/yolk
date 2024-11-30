use std::path::PathBuf;

use mlua::IntoLua;

#[derive(Debug, Clone)]
pub struct SystemInfo {
    hostname: String,
    username: String,
    paths: SystemInfoPaths,
}

impl IntoLua for SystemInfo {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        table.set("hostname", self.hostname)?;
        table.set("username", self.username)?;
        table.set("paths", self.paths)?;
        Ok(mlua::Value::Table(table))
    }
}

#[derive(Debug, Clone)]
pub struct SystemInfoPaths {
    cache_dir: PathBuf,
    config_dir: PathBuf,
    home_dir: PathBuf,
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
            hostname: std::env::var("HOSTNAME").unwrap_or("no-hostname".to_string()),
            username: std::env::var("USER").unwrap_or("no-username".to_string()),
            paths: SystemInfoPaths {
                cache_dir: PathBuf::from("/test/cache"),
                config_dir: PathBuf::from("/test/config"),
                home_dir: PathBuf::from("/test/home"),
                yolk_dir: PathBuf::from("/test/yolk"),
            },
        }
    }

    pub fn canonical() -> Self {
        Self {
            hostname: "canonical-hostname".to_string(),
            username: "canonical-username".to_string(),
            paths: SystemInfoPaths {
                cache_dir: PathBuf::from("/canonical/cache"),
                config_dir: PathBuf::from("/canonical/config"),
                home_dir: PathBuf::from("/canonical/home"),
                yolk_dir: PathBuf::from("/canonical/yolk"),
            },
        }
    }
}
