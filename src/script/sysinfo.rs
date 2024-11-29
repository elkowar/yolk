use mlua::IntoLua;

#[derive(Debug, Clone)]
pub struct SystemInfo {
    hostname: String,
    username: String,
}
impl IntoLua for SystemInfo {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        table.set("hostname", self.hostname)?;
        table.set("username", self.username)?;
        Ok(mlua::Value::Table(table))
    }
}

impl SystemInfo {
    #[cfg(test)]
    pub fn generate() -> Self {
        Self {
            hostname: "test-hostname".to_string(),
            username: "test-username".to_string(),
        }
    }

    #[cfg(not(test))]
    pub fn generate() -> Self {
        // lmao make this not garbage
        Self {
            hostname: std::env::var("HOSTNAME").unwrap_or("no-hostname".to_string()),
            username: std::env::var("USER").unwrap_or("no-username".to_string()),
        }
    }

    pub fn canonical() -> Self {
        Self {
            hostname: "canonical-hostname".to_string(),
            username: "canonical-username".to_string(),
        }
    }
}
