use miette::{IntoDiagnostic, Result};
use std::path::PathBuf;

use miette::Context as _;
use mlua::Value;
use regex::Regex;

use super::eval_ctx::{EvalCtx, YOLK_TEXT_NAME};

pub fn setup_tag_functions(eval_ctx: &EvalCtx) -> miette::Result<()> {
    setup_pure_functions(eval_ctx)?;

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
                    "Refusing to run non-reversible replacement: {text} -> {after_replace}",
                );
                return Ok(text.to_string());
            }
        };
        Ok(after_replace.to_string())
    }
    eval_ctx.register_fn("replace", |lua, (regex, replacement): (String, String)| {
        let text = lua
            .globals()
            .get::<String>(YOLK_TEXT_NAME)
            .into_diagnostic()?;
        tag_text_replace(&text, &regex, &replacement)
    })?;
    eval_ctx.set_global("r", eval_ctx.get_global::<mlua::Function>("replace")?)?;

    eval_ctx.register_fn(
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
    eval_ctx.register_fn(
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
    eval_ctx.set_global("ri", eval_ctx.get_global::<mlua::Function>("replace_in")?)?;

    eval_ctx.register_fn("replace_color", |lua, replacement: String| {
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
    eval_ctx.set_global(
        "rc",
        eval_ctx.get_global::<mlua::Function>("replace_color")?,
    )?;

    Ok(())
}

pub fn setup_pure_functions(eval_ctx: &EvalCtx) -> Result<()> {
    let globals = eval_ctx.lua().globals();

    let inspect = include_str!("lua_lib/inspect.lua");
    let inspect = eval_ctx
        .lua()
        .load(inspect)
        .set_name("inspect.lua")
        .into_function()
        .into_diagnostic()?;
    let inspect = eval_ctx
        .lua()
        .load_from_function::<Value>("inspect", inspect)
        .into_diagnostic()?;
    globals.set("inspect", inspect).into_diagnostic()?;

    eval_ctx.register_fn(
        "regex_match",
        |_lua, (pattern, haystack): (String, String)| {
            Ok(regex::Regex::new(&pattern)
                .into_diagnostic()
                .with_context(|| format!("Invalid regex: {pattern}"))?
                .is_match(&haystack))
        },
    )?;

    eval_ctx.register_fn(
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

pub fn setup_impure_functions(eval_ctx: &EvalCtx) -> Result<()> {
    eval_ctx.register_fn("command_available", |_, name: String| {
        Ok(match which::which_all_global(name) {
            Ok(mut iter) => iter.next().is_some(),
            Err(err) => {
                tracing::warn!("Error checking if command is available: {}", err);
                false
            }
        })
    })?;
    eval_ctx.register_fn("env", |_, (name, default): (String, String)| {
        Ok(std::env::var(name).unwrap_or(default))
    })?;
    eval_ctx.register_fn("path_exists", |_, p: String| Ok(PathBuf::from(p).exists()))?;
    eval_ctx.register_fn("path_is_dir", |_, p: String| {
        Ok(fs_err::metadata(p).map(|m| m.is_dir()).unwrap_or(false))
    })?;
    eval_ctx.register_fn("path_is_file", |_, p: String| {
        Ok(fs_err::metadata(p).map(|m| m.is_file()).unwrap_or(false))
    })?;
    eval_ctx.register_fn("read_file", |_, p: String| {
        Ok(fs_err::read_to_string(p).unwrap_or_default())
    })?;
    Ok(())
}

#[cfg(test)]
mod test {
    use testresult::TestResult;

    use crate::script::eval_ctx::EvalCtx;

    #[test]
    pub fn test_inspect() -> TestResult {
        let eval_ctx = EvalCtx::new_empty();
        super::setup_pure_functions(&eval_ctx)?;
        assert_eq!(
            "{ 1, 2 }",
            eval_ctx.eval_lua::<String>("test", "inspect({1, 2})")?
        );

        Ok(())
    }

    #[test]
    pub fn test_replace() -> TestResult {
        let eval_ctx = EvalCtx::new_empty();
        super::setup_tag_functions(&eval_ctx)?;
        eval_ctx.set_global("YOLK_TEXT", "foo:'aaa'")?;
        assert_eq!(
            "foo:'xxx'",
            eval_ctx.eval_lua::<String>("test", "replace(`'.*'`, `'xxx'`)")?
        );
        assert_eq!(
            "foo:'aaa'",
            eval_ctx.eval_lua::<String>("test", "replace(`'.*'`, `xxx`)")?,
            "replace performed non-reversible replacement",
        );
        Ok(())
    }
    #[test]
    pub fn test_replace_in() -> TestResult {
        let eval_ctx = EvalCtx::new_empty();
        super::setup_tag_functions(&eval_ctx)?;
        eval_ctx.set_global("YOLK_TEXT", "foo:'aaa'")?;
        assert_eq!(
            "foo:'xxx'",
            eval_ctx.eval_lua::<String>("test", "replace_in(`'`, `xxx`)")?
        );
        assert_eq!(
            "foo:'aaa'",
            eval_ctx.eval_lua::<String>("test", "replace_in(`'`, `x'xx`)")?,
            "replace performed non-reversible replacement",
        );
        Ok(())
    }

    #[test]
    pub fn test_replace_color() -> TestResult {
        let eval_ctx = EvalCtx::new_empty();
        super::setup_tag_functions(&eval_ctx)?;
        eval_ctx.set_global("YOLK_TEXT", "foo: #ff0000")?;
        assert_eq!(
            "foo: #00ff00",
            eval_ctx.eval_lua::<String>("test", "replace_color(`#00ff00`)")?,
        );
        assert_eq!(
            "foo: #00ff0000",
            eval_ctx.eval_lua::<String>("test", "replace_color(`#00ff0000`)")?,
        );
        assert_eq!(
            "foo: #ff0000",
            eval_ctx.eval_lua::<String>("test", "replace_color(`00ff00`)")?,
            "replace_color performed non-reversible replacement",
        );
        assert_eq!(
            "foo: #ff0000",
            eval_ctx.eval_lua::<String>("test", "replace_color(`bad color`)")?,
            "replace_color performed non-reversible replacement",
        );
        Ok(())
    }
}
