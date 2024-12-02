use miette::{IntoDiagnostic, Result};
use std::path::PathBuf;

use miette::Context as _;
use mlua::{ExternalResult, Lua, Value};
use regex::Regex;

use super::eval_ctx::YOLK_TEXT_NAME;

pub fn setup_tag_functions(lua: &Lua) -> miette::Result<()> {
    setup_pure_functions(lua)?;
    let globals = lua.globals();

    /// Simple regex replacement that will refuse to run a non-reversible replacement.
    /// If the replacement value is non-reversible, will return the original text and log a warning.
    fn tag_text_replace(text: &str, pattern: &str, replacement: &str) -> Result<String> {
        let pattern = Regex::new(pattern)
            .into_diagnostic()
            .wrap_err_with(|| format!("Invalid regex: {pattern}"))?;
        let after_replace = pattern.replace(text, replacement);
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
            let text = lua
                .globals()
                .get::<String>(YOLK_TEXT_NAME)
                .into_diagnostic()?;
            tag_text_replace(&text, &regex, &replacement)
        },
    )?;
    globals
        .set(
            "r",
            globals.get::<mlua::Function>("replace").into_diagnostic()?,
        )
        .into_diagnostic()?;

    register_fn(
        lua,
        "replace_in",
        |lua, (between, replacement): (String, String)| {
            let text = lua
                .globals()
                .get::<String>(YOLK_TEXT_NAME)
                .into_diagnostic()?;
            let regex = format!("{between}[^{between}]*{between}");
            tag_text_replace(&text, &regex, &format!("{between}{replacement}{between}"))
        },
    )?;
    globals
        .set(
            "ri",
            globals
                .get::<mlua::Function>("replace_in")
                .into_diagnostic()?,
        )
        .into_diagnostic()?;
    register_fn(lua, "replace_color", |lua, replacement: String| {
        let text = lua
            .globals()
            .get::<String>(YOLK_TEXT_NAME)
            .into_diagnostic()?;
        tag_text_replace(
            &text,
            r"#[\da-fA-F]{6}([\da-fA-F]{2})?",
            &replacement.to_string(),
        )
    })?;
    globals
        .set(
            "rc",
            globals
                .get::<mlua::Function>("replace_in")
                .into_diagnostic()?,
        )
        .into_diagnostic()?;

    Ok(())
}

pub fn setup_pure_functions(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    let inspect = include_str!("lua_lib/inspect.lua");
    let inspect = lua
        .load(inspect)
        .set_name("inspect.lua")
        .into_function()
        .into_diagnostic()?;
    let inspect = lua
        .load_from_function::<Value>("inspect", inspect)
        .into_diagnostic()?;
    globals.set("inspect", inspect).into_diagnostic()?;

    register_fn(
        lua,
        "regex_match",
        |_lua, (pattern, haystack): (String, String)| {
            Ok(regex::Regex::new(&pattern)
                .into_diagnostic()
                .with_context(|| format!("Invalid regex: {pattern}"))?
                .is_match(&haystack))
        },
    )?;

    register_fn(
        lua,
        "regex_replace",
        |_lua, (pattern, haystack, replacement): (String, String, String)| {
            Ok(regex::Regex::new(&pattern)
                .into_diagnostic()
                .with_context(|| format!("Invalid regex: {pattern}"))?
                .replace_all(&haystack, &replacement)
                .to_string())
        },
    )?;
    Ok(())
}

fn register_fn<F, A, R>(lua: &Lua, name: &str, func: F) -> Result<()>
where
    F: Fn(&Lua, A) -> miette::Result<R> + mlua::MaybeSend + 'static + Send + Sync,
    A: mlua::FromLuaMulti,
    R: mlua::IntoLuaMulti,
{
    lua.globals()
        .set(
            name,
            lua.create_function(move |lua, x| func(lua, x).into_lua_err())
                .into_diagnostic()?,
        )
        .into_diagnostic()
}

pub fn setup_impure_functions(lua: &Lua) -> Result<()> {
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
