use anyhow::Result;
use std::path::PathBuf;

use anyhow::Context as _;
use mlua::{Lua, Value};
use regex::Regex;

use super::eval_ctx::YOLK_TEXT_NAME;

pub fn setup_tag_functions(lua: &Lua) -> anyhow::Result<()> {
    setup_pure_functions(&lua)?;
    let globals = lua.globals();

    /// Simple regex replacement that will refuse to run a non-reversible replacement.
    /// If the replacement value is non-reversible, will return the original text and log a warning.
    fn tag_text_replace(text: &str, pattern: &str, replacement: &str) -> Result<String> {
        let pattern = Regex::new(&pattern).with_context(|| format!("Invalid regex: {pattern}"))?;
        let after_replace = pattern.replace(&text, replacement);
        if let Some(original_value) = pattern.find(text) {
            let original_value = original_value.as_str();
            let reversed = pattern.replace(&after_replace, original_value);
            if reversed != text {
                tracing::warn!(
                    "Refusing to run non-reversible replacement: {} -> {}",
                    text,
                    after_replace
                );
                return Ok(text.to_string());
            }
        };
        Ok(after_replace.to_string())
    }
    register_fn(
        lua,
        "replace",
        |lua, (regex, replacement): (String, String)| {
            let text = lua.globals().get::<String>(YOLK_TEXT_NAME)?;
            Ok(tag_text_replace(&text, &regex, &replacement)?)
        },
    )?;
    globals.set("r", globals.get::<mlua::Function>("replace")?)?;

    register_fn(
        lua,
        "replace_in",
        |lua, (between, replacement): (String, String)| {
            let text = lua.globals().get::<String>(YOLK_TEXT_NAME)?;
            let regex = format!("{between}[^{between}]*{between}");
            Ok(tag_text_replace(
                &text,
                &regex,
                &format!("{between}{replacement}{between}"),
            )?)
        },
    )?;
    globals.set("ri", globals.get::<mlua::Function>("replace_in")?)?;
    register_fn(lua, "replace_color", |lua, replacement: String| {
        let text = lua.globals().get::<String>(YOLK_TEXT_NAME)?;
        Ok(tag_text_replace(
            &text,
            r"#[\da-fA-F]{6}([\da-fA-F]{2})?",
            &format!("{replacement}"),
        )?)
    })?;
    globals.set("rc", globals.get::<mlua::Function>("replace_in")?)?;

    Ok(())
}

pub fn setup_pure_functions(lua: &Lua) -> anyhow::Result<()> {
    let globals = lua.globals();

    let inspect = include_str!("lua_lib/inspect.lua");
    let inspect = lua.load(inspect).set_name("inspect.lua").into_function()?;
    let inspect = lua.load_from_function::<Value>("inspect", inspect)?;
    globals.set("inspect", inspect)?;

    register_fn(
        lua,
        "regex_match",
        |_lua, (pattern, haystack): (String, String)| {
            Ok(regex::Regex::new(&pattern)
                .with_context(|| format!("Invalid regex: {pattern}"))?
                .is_match(&haystack))
        },
    )?;

    register_fn(
        lua,
        "regex_replace",
        |_lua, (pattern, haystack, replacement): (String, String, String)| {
            Ok(regex::Regex::new(&pattern)
                .with_context(|| format!("Invalid regex: {pattern}"))?
                .replace_all(&haystack, &replacement)
                .to_string())
        },
    )?;
    Ok(())
}

fn register_fn<F, A, R>(lua: &Lua, name: &str, func: F) -> mlua::Result<()>
where
    F: Fn(&Lua, A) -> mlua::Result<R> + mlua::MaybeSend + 'static,
    A: mlua::FromLuaMulti,
    R: mlua::IntoLuaMulti,
{
    lua.globals().set(name, lua.create_function(func)?)
}

pub fn setup_impure_functions(lua: &Lua) -> anyhow::Result<()> {
    register_fn(lua, "command_available", |_, name: String| {
        Ok(match which::which_all_global(name) {
            Ok(mut iter) => iter.next().is_some(),
            Err(err) => {
                tracing::warn!("Error checking if command is available: {}", err);
                false
            }
        })
    })?;
    register_fn(lua, "env", |_, (name, default): (String, String)| {
        Ok(std::env::var(name).unwrap_or(default))
    })?;
    register_fn(lua, "path_exists", |_, p: String| {
        Ok(PathBuf::from(p).exists())
    })?;
    register_fn(lua, "path_is_dir", |_, p: String| {
        Ok(fs_err::metadata(p).map(|m| m.is_dir()).unwrap_or(false))
    })?;
    register_fn(lua, "path_is_file", |_, p: String| {
        Ok(fs_err::metadata(p).map(|m| m.is_file()).unwrap_or(false))
    })?;
    register_fn(lua, "read_file", |_, p: String| {
        Ok(fs_err::read_to_string(p).unwrap_or_default())
    })?;
    Ok(())
}

#[cfg(test)]
mod test {
    use testresult::TestResult;

    #[test]
    pub fn test_inspect() -> TestResult {
        let lua = mlua::Lua::new();
        super::setup_pure_functions(&lua)?;
        assert_eq!("{ 1, 2 }", lua.load("inspect({1, 2})").eval::<String>()?);

        Ok(())
    }

    #[test]
    pub fn test_replace() -> TestResult {
        let lua = mlua::Lua::new();
        super::setup_tag_functions(&lua)?;
        lua.globals().set("YOLK_TEXT", "foo:'aaa'")?;
        assert_eq!(
            "foo:'xxx'",
            lua.load("replace(`'.*'`, `'xxx'`)").eval::<String>()?
        );
        assert_eq!(
            "foo:'aaa'",
            lua.load("replace(`'.*'`, `xxx`)").eval::<String>()?,
            "replace performed non-reversible replacement",
        );
        Ok(())
    }
    #[test]
    pub fn test_replace_in() -> TestResult {
        let lua = mlua::Lua::new();
        super::setup_tag_functions(&lua)?;
        lua.globals().set("YOLK_TEXT", "foo:'aaa'")?;
        assert_eq!(
            "foo:'xxx'",
            lua.load("replace_in(`'`, `xxx`)").eval::<String>()?
        );
        assert_eq!(
            "foo:'aaa'",
            lua.load("replace_in(`'`, `x'xx`)").eval::<String>()?,
            "replace performed non-reversible replacement",
        );
        Ok(())
    }

    #[test]
    pub fn test_replace_color() -> TestResult {
        let lua = mlua::Lua::new();
        super::setup_tag_functions(&lua)?;
        lua.globals().set("YOLK_TEXT", "foo: #ff0000")?;
        assert_eq!(
            "foo: #00ff00",
            lua.load("replace_color(`#00ff00`)").eval::<String>()?
        );
        assert_eq!(
            "foo: #00ff0000",
            lua.load("replace_color(`#00ff0000`)").eval::<String>()?
        );
        assert_eq!(
            "foo: #ff0000",
            lua.load("replace_color(`00ff00`)").eval::<String>()?,
            "replace_color performed non-reversible replacement",
        );
        assert_eq!(
            "foo: #ff0000",
            lua.load("replace_color(`bad color`)").eval::<String>()?,
            "replace_color performed non-reversible replacement",
        );
        Ok(())
    }
}
