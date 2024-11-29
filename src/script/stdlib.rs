use anyhow::Context as _;
use mlua::Lua;
use regex::Regex;

use super::eval_ctx::YOLK_TEXT_NAME;

pub fn setup_tag_functions(lua: &Lua) -> anyhow::Result<()> {
    let globals = lua.globals();
    globals.set(
        "reverse",
        lua.create_function(|lua, ()| {
            let text = lua.globals().get::<String>(YOLK_TEXT_NAME)?;
            Ok(text.chars().rev().collect::<String>())
        })?,
    )?;
    globals.set(
        "replace",
        lua.create_function(|lua, (regex, replacement): (String, String)| {
            let text = lua.globals().get::<String>(YOLK_TEXT_NAME)?;
            let pattern = Regex::new(&regex).with_context(|| format!("Invalid regex: {regex}"))?;
            Ok(pattern.replace_all(&text, replacement).to_string())
        })?,
    )?;
    Ok(())
}
/*

// TODO: Potentially turn this into a rhai module instead
pub fn register(engine: &mut rhai::Engine) {
    engine
        .register_fn("command_available", command_available)
        .register_fn("env", |name: &str, default: String| {
            std::env::var(name).unwrap_or(default)
        })
        .register_fn("path_exists", |path: &str| PathBuf::from(path).exists())
        .register_fn("path_is_dir", |path: &str| {
            fs_err::metadata(path).map(|m| m.is_dir()).unwrap_or(false)
        })
        .register_fn("path_is_file", |path: &str| {
            fs_err::metadata(path).map(|m| m.is_file()).unwrap_or(false)
        })
        .register_fn("read_file", |path: &str| {
            fs_err::read_to_string(path).unwrap_or_default()
        })
        .register_fn(
            "regex_match",
            |pattern: &str, haystack: &str| -> Result<bool, Box<EvalAltResult>> {
                regex::Regex::new(pattern)
                    .map(|x| x.is_match(haystack))
                    .map_err(|err| err.to_string().into())
            },
        )
        .register_fn(
            "regex_replace",
            |haystack: &str,
             pattern: &str,
             replacement: &str|
             -> Result<String, Box<EvalAltResult>> {
                regex::Regex::new(pattern)
                    .map(|x| x.replace_all(haystack, replacement))
                    .map(|x| x.to_string())
                    .map_err(|err| err.to_string().into())
            },
        );
}
pub fn command_available(command: &str) -> bool {
    match which::which_all_global(command) {
        Ok(mut iter) => iter.next().is_some(),
        Err(err) => {
            tracing::warn!("Error checking if command is available: {}", err);
            false
        }
    }
}

*/
