use cached::proc_macro::cached;
use miette::{Context as _, IntoDiagnostic, Result};
use std::path::PathBuf;

use mlua::{LuaSerdeExt as _, Value};
use regex::Regex;

use crate::{script::lua_error::LuaError, yolk::EvalMode};

use super::eval_ctx::{EvalCtx, YOLK_TEXT_NAME};

pub fn setup_stdlib(eval_mode: EvalMode, eval_ctx: &EvalCtx) -> Result<(), LuaError> {
    setup_environment_stuff(eval_mode, eval_ctx)?;
    let globals = eval_ctx.lua().globals();

    let inspect = include_str!("lua_lib/inspect.lua");
    let inspect = eval_ctx
        .lua()
        .load(inspect)
        .set_name("inspect.lua")
        .into_function()?;
    let inspect = eval_ctx
        .lua()
        .load_from_function::<Value>("inspect", inspect)?;
    globals.set("inspect", inspect)?;

    eval_ctx.register_fn(
        "regex_match",
        |_lua, (pattern, haystack): (String, String)| {
            Ok(create_regex(&pattern)?.is_match(&haystack))
        },
    )?;

    eval_ctx.register_fn(
        "regex_replace",
        |_lua, (pattern, haystack, replacement): (String, String, String)| {
            Ok(create_regex(&pattern)?
                .replace_all(&haystack, &replacement)
                .to_string())
        },
    )?;
    eval_ctx.register_fn("regex_captures", |_lua, (pattern, s): (String, String)| {
        Ok(create_regex(&pattern)?.captures(s.as_str()).map(|caps| {
            (0..caps.len())
                .into_iter()
                .map(|x| caps.get(x).unwrap().as_str().to_string())
                .collect::<Vec<_>>()
        }))
    })?;
    eval_ctx.register_fn(
        "contains_value",
        |_lua, (container, value): (Value, Value)| {
            let container = container
                .as_table()
                .wrap_err("Not a container")
                .map_err(LuaError::Other)?;
            for pair in container.pairs::<Value, Value>() {
                if pair?.1.equals(&value)? {
                    return Ok(true);
                }
            }
            Ok(false)
        },
    )?;
    eval_ctx.register_fn(
        "contains_key",
        |_lua, (container, value): (Value, Value)| {
            let container = container
                .as_table()
                .wrap_err("Not a container")
                .map_err(LuaError::Other)?;
            for pair in container.pairs::<Value, Value>() {
                if pair?.0.equals(&value)? {
                    return Ok(true);
                }
            }
            Ok(false)
        },
    )?;

    eval_ctx.register_fn("from_json", |lua, json: String| {
        let value: serde_json::Value = serde_json::from_str(&json).map_err(LuaError::new_other)?;
        Ok(lua.to_value(&value))
    })?;
    eval_ctx.register_fn("to_json", |lua, value: Value| {
        let json_value: serde_json::Value = lua.from_value(value).map_err(LuaError::new_other)?;
        Ok(serde_json::to_string(&json_value).unwrap())
    })?;

    // TODO: Add deepcopy
    Ok(())
}

macro_rules! if_canonical_return {
    ($eval_mode:expr) => {
        if $eval_mode == EvalMode::Canonical {
            return Ok(Default::default());
        }
    };
    ($eval_mode:expr, $value:expr) => {
        if $eval_mode == EvalMode::Canonical {
            return Ok($value);
        }
    };
}

pub fn setup_environment_stuff(eval_mode: EvalMode, eval_ctx: &EvalCtx) -> Result<(), LuaError> {
    eval_ctx.register_fn("command_available", move |_, name: String| {
        if_canonical_return!(eval_mode);
        Ok(match which::which_all_global(name) {
            Ok(mut iter) => iter.next().is_some(),
            Err(err) => {
                tracing::warn!("Error checking if command is available: {}", err);
                false
            }
        })
    })?;
    eval_ctx.register_fn("env", move |_, (name, default): (String, String)| {
        if_canonical_return!(eval_mode);
        Ok(std::env::var(name).unwrap_or(default))
    })?;
    eval_ctx.register_fn("path_exists", move |_, p: String| {
        if_canonical_return!(eval_mode);
        Ok(PathBuf::from(p).exists())
    })?;
    eval_ctx.register_fn("path_is_dir", move |_, p: String| {
        if_canonical_return!(eval_mode);
        Ok(fs_err::metadata(p).map(|m| m.is_dir()).unwrap_or(false))
    })?;
    eval_ctx.register_fn("path_is_file", move |_, p: String| {
        if_canonical_return!(eval_mode);
        Ok(fs_err::metadata(p).map(|m| m.is_file()).unwrap_or(false))
    })?;
    eval_ctx.register_fn("read_file", move |_, p: String| {
        if_canonical_return!(eval_mode);
        Ok(fs_err::read_to_string(p).unwrap_or_default())
    })?;
    eval_ctx.register_fn("read_dir", move |_, p: String| -> Result<Vec<String>, _> {
        if_canonical_return!(eval_mode);
        Ok(fs_err::read_dir(p)
            .into_diagnostic()
            .map_err(|e| LuaError::Other(e))?
            .map(|x| {
                Ok(x.into_diagnostic()
                    .map_err(LuaError::Other)?
                    .path()
                    .to_string_lossy()
                    .to_string())
            })
            .collect::<Result<_, LuaError>>()?)
    })?;
    Ok(())
}

pub fn setup_tag_functions(eval_ctx: &EvalCtx) -> Result<(), LuaError> {
    /// Simple regex replacement that will refuse to run a non-reversible replacement.
    /// If the replacement value is non-reversible, will return the original text and log a warning.
    fn tag_text_replace(text: &str, pattern: &str, replacement: &str) -> Result<String, LuaError> {
        let pattern = create_regex(pattern)?;
        let after_replace = pattern.replace(text, replacement);
        if let Some(original_value) = pattern.find(text) {
            let original_value = original_value.as_str();
            let reversed = pattern.replace(&after_replace, original_value);
            if reversed != text {
                return Err(LuaError::Other(miette::miette!(
                    "Refusing to run non-reversible replacement: {text} -> {after_replace}",
                )));
            }
        };
        Ok(after_replace.to_string())
    }

    eval_ctx.register_fn(
        "replace_re",
        |lua, (regex, replacement): (String, String)| {
            let text = lua.globals().get::<String>(YOLK_TEXT_NAME)?;
            tag_text_replace(&text, &regex, &replacement)
        },
    )?;
    eval_ctx.set_global("rr", eval_ctx.get_global::<mlua::Function>("replace_re")?)?;

    eval_ctx.register_fn(
        "replace_in",
        |lua, (between, replacement): (String, String)| {
            let text = lua.globals().get::<String>(YOLK_TEXT_NAME)?;
            let regex = format!("{between}[^{between}]*{between}");
            tag_text_replace(&text, &regex, &format!("{between}{replacement}{between}"))
        },
    )?;
    eval_ctx.set_global("rin", eval_ctx.get_global::<mlua::Function>("replace_in")?)?;
    eval_ctx.register_fn(
        "replace_between",
        |lua, (left, right, replacement): (String, String, String)| {
            let text = lua.globals().get::<String>(YOLK_TEXT_NAME)?;
            let regex = format!("{left}[^{right}]*{right}");
            tag_text_replace(&text, &regex, &format!("{left}{replacement}{right}"))
        },
    )?;
    eval_ctx.set_global(
        "rbet",
        eval_ctx.get_global::<mlua::Function>("replace_between")?,
    )?;

    eval_ctx.register_fn("replace_color", |lua, replacement: String| {
        let text = lua.globals().get::<String>(YOLK_TEXT_NAME)?;
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

    eval_ctx.register_fn("replace_number", |lua, replacement: String| {
        let text = lua.globals().get::<String>(YOLK_TEXT_NAME)?;
        tag_text_replace(&text, r"-?\d+(?:\.\d+)?", &replacement.to_string())
    })?;
    eval_ctx.set_global(
        "rnum",
        eval_ctx.get_global::<mlua::Function>("replace_number")?,
    )?;

    eval_ctx.register_fn("replace_quoted", |lua, replacement: String| {
        let text = lua.globals().get::<String>(YOLK_TEXT_NAME)?;
        let mut result = tag_text_replace(&text, r#"".*""#, &format!("\"{replacement}\""))?;
        if result == text {
            result = tag_text_replace(&text, r#"`.*`"#, &format!("`{replacement}`"))?;
        }
        if result == text {
            result = tag_text_replace(&text, r#"'.*'"#, &format!("'{replacement}'"))?;
        }
        Ok(result)
    })?;
    eval_ctx.set_global(
        "rq",
        eval_ctx.get_global::<mlua::Function>("replace_quoted")?,
    )?;

    eval_ctx.register_fn("replace_value", |lua, replacement: String| {
        let text = lua.globals().get::<String>(YOLK_TEXT_NAME)?;
        let regex = create_regex(r"([=:])( *)([^\s]+)").unwrap();

        if let Some(caps) = regex.captures(&text) {
            let full_match = caps.get(0).unwrap();
            let equals = caps.get(1).unwrap();
            let space = caps.get(2).unwrap();
            let new_value = regex.replace(
                &text,
                format!("{}{}{}", equals.as_str(), space.as_str(), replacement),
            );
            if regex.replace(&new_value, full_match.as_str()) == text {
                Ok(new_value.to_string())
            } else {
                Err(LuaError::Other(miette::miette!(
                    "Refusing to run non-reversible replacement: {text} -> {new_value}",
                )))
            }
        } else {
            Ok(text)
        }
    })?;
    eval_ctx.set_global(
        "rv",
        eval_ctx.get_global::<mlua::Function>("replace_value")?,
    )?;
    Ok(())
}

#[cached(key = "String", convert = r#"{s.to_string()}"#, result)]
fn create_regex(s: &str) -> Result<Regex, LuaError> {
    Regex::new(&s).into_diagnostic().map_err(LuaError::Other)
}

#[cfg(test)]
mod test {
    use mlua::FromLuaMulti;
    use testresult::TestResult;

    use crate::{
        script::{eval_ctx::EvalCtx, lua_error::LuaError},
        yolk::EvalMode,
    };

    pub fn run_lua<T: FromLuaMulti>(lua: &str) -> miette::Result<T> {
        let eval_ctx = EvalCtx::new_in_mode(EvalMode::Local)?;
        Ok(eval_ctx.eval_template_lua::<T>("test", lua)?)
    }
    pub fn run_tag_lua(text: &str, lua: &str) -> Result<String, LuaError> {
        let eval_ctx = EvalCtx::new_empty();
        super::setup_tag_functions(&eval_ctx)?;
        eval_ctx.set_global("YOLK_TEXT", text)?;
        Ok(eval_ctx.eval_template_lua::<String>("test", lua)?)
    }

    #[test]
    pub fn test_regex_captures() -> TestResult {
        assert_eq!(
            vec!["<aaaXb>".to_string(), "aaa".to_string(), "b".to_string()],
            run_lua::<Vec<String>>("regex_captures(`<(.*)X(.)>`, `foo <aaaXb> bar`)")?
        );
        assert_eq!(
            None,
            run_lua::<Option<Vec<String>>>("regex_captures(`<(.*)X(.)>`, `asdf`)")?
        );
        Ok(())
    }

    #[test]
    pub fn test_to_json() -> TestResult {
        assert_eq!(
            r#"{"a":2,"b":[1,2,3]}"#,
            run_lua::<String>("to_json({a = 2, b = {1, 2, 3}})")?,
        );
        assert_eq!(
            r#"{ 1, 2 }"#,
            run_lua::<String>(r#"inspect.inspect(from_json('[1, 2]'))"#)?,
        );
        Ok(())
    }

    #[test]
    pub fn test_inspect() -> TestResult {
        let eval_ctx = EvalCtx::new_empty();
        super::setup_stdlib(EvalMode::Local, &eval_ctx)?;
        assert_eq!(
            "{ 1, 2 }",
            eval_ctx.eval_template_lua::<String>("test", "inspect({1, 2})")?
        );
        Ok(())
    }

    #[test]
    pub fn test_replace() -> TestResult {
        assert_eq!(
            "foo:'xxx'",
            run_tag_lua("foo:'aaa'", "replace_re(`'.*'`, `'xxx'`)")?
        );
        assert!(
            run_tag_lua("foo:'aaa'", "replace_re(`'.*'`, `xxx`)").is_err(),
            "replace performed non-reversible replacement",
        );
        Ok(())
    }
    #[test]
    pub fn test_replace_in() -> TestResult {
        assert_eq!(
            "foo:'xxx'",
            run_tag_lua("foo:'aaa'", "replace_in(`'`, `xxx`)")?
        );
        assert!(
            run_tag_lua("foo:'aaa'", "replace_in(`'`, `x'xx`)").is_err(),
            "replace performed non-reversible replacement",
        );
        Ok(())
    }

    #[test]
    pub fn test_replace_color() -> TestResult {
        assert_eq!(
            "foo: #00ff00",
            run_tag_lua("foo: #ff0000", "replace_color(`#00ff00`)")?,
        );
        assert_eq!(
            "foo: #00ff0000",
            run_tag_lua("foo: #ff0000", "replace_color(`#00ff0000`)")?,
        );
        assert!(
            run_tag_lua("foo: #ff0000", "replace_color(`00ff00`)").is_err(),
            "replace_color performed non-reversible replacement",
        );
        assert!(
            run_tag_lua("foo: #ff0000", "replace_color(`bad color`)").is_err(),
            "replace_color performed non-reversible replacement",
        );
        Ok(())
    }

    #[test]
    pub fn test_replace_quoted() -> TestResult {
        assert_eq!(
            "foo: 'new'",
            run_tag_lua("foo: 'old'", "replace_quoted(`new`)")?,
        );
        assert_eq!(
            "foo: \"new\"",
            run_tag_lua("foo: \"old\"", "replace_quoted(`new`)")?,
        );
        assert_eq!(
            "foo: `new`",
            run_tag_lua("foo: `old`", "replace_quoted(`new`)")?,
        );
        Ok(())
    }

    #[test]
    pub fn test_replace_value() -> TestResult {
        assert_eq!(
            "foo: xxx # baz",
            run_tag_lua("foo: bar # baz", "replace_value(`xxx`)")?,
        );
        assert!(
            run_tag_lua("foo: bar # baz", "replace_value(`x xx`)").is_err(),
            "replace_value performed non-reversible replacement",
        );
        Ok(())
    }

    #[test]
    pub fn test_replace_number() -> TestResult {
        assert_eq!(
            "foo 999 bar",
            run_tag_lua("foo 123 bar", "replace_number(999)")?,
        );
        assert_eq!(
            "foo 99.9 bar",
            run_tag_lua("foo 1.23 bar", "replace_number(99.9)")?,
        );
        assert!(
            run_tag_lua("foo 99.9 bar", "replace_number(`hi`)").is_err(),
            "replace_value performed non-reversible replacement",
        );
        Ok(())
    }
}
