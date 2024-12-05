use miette::Result;
use rhai::{EvalAltResult, ImmutableString, NativeCallContext};
use std::path::PathBuf;

use regex::Regex;

use super::eval_ctx::EvalCtx;

pub fn setup_tag_functions(eval_ctx: &mut EvalCtx) -> miette::Result<()> {
    setup_pure_functions(eval_ctx)?;

    /// Simple regex replacement that will refuse to run a non-reversible replacement.
    /// If the replacement value is non-reversible, will return the original text and log a warning.
    fn tag_text_replace(
        text: &str,
        pattern: &str,
        replacement: &str,
    ) -> Result<String, Box<EvalAltResult>> {
        let pattern = create_regex(pattern)?;
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
    eval_ctx.engine_mut().register_fn(
        "replace",
        |nc: rhai::NativeCallContext, regex: ImmutableString, replacement: ImmutableString| {
            let text: String = nc.call_native_fn("get_yolk_text", ())?;
            tag_text_replace(&text, &regex, &replacement)
        },
    );
    // eval_ctx.set_global("r", eval_ctx.get_global::<mlua::Function>("replace")?)?;

    eval_ctx.engine_mut().register_fn(
        "replace_in",
        |nc: NativeCallContext, between: ImmutableString, replacement: ImmutableString| {
            let text: String = nc.call_native_fn("get_yolk_text", ())?;
            let regex = format!("{between}[^{between}]*{between}");
            tag_text_replace(&text, &regex, &format!("{between}{replacement}{between}"))
        },
    );
    eval_ctx.engine_mut().register_fn(
        "replace_in",
        |nc: NativeCallContext, between: ImmutableString, replacement: ImmutableString| {
            let text: String = nc.call_native_fn("get_yolk_text", ())?;
            let regex = format!("{between}[^{between}]*{between}");
            tag_text_replace(&text, &regex, &format!("{between}{replacement}{between}"))
        },
    );
    // eval_ctx.set_global("ri", eval_ctx.get_global::<mlua::Function>("replace_in")?)?;

    eval_ctx.engine_mut().register_fn(
        "replace_color",
        |nc: NativeCallContext, replacement: String| {
            let text: String = nc.call_native_fn("get_yolk_text", ())?;
            tag_text_replace(
                &text,
                r"#[\da-fA-F]{6}([\da-fA-F]{2})?",
                &replacement.to_string(),
            )
        },
    );
    // eval_ctx.set_global(
    //     "rc",
    //     eval_ctx.get_global::<mlua::Function>("replace_color")?,
    // )?;

    Ok(())
}

fn create_regex(s: &str) -> Result<Regex, Box<EvalAltResult>> {
    Ok(Regex::new(s)
        .map_err(|_| Box::<EvalAltResult>::new(format!("Invalid regex: {s}").into()))?)
}

pub fn setup_pure_functions(eval_ctx: &mut EvalCtx) -> Result<()> {
    eval_ctx.engine_mut().register_fn(
        "regex_match",
        |pattern: ImmutableString, haystack: ImmutableString| -> Result<bool, Box<EvalAltResult>> {
            Ok(create_regex(&pattern)?.is_match(&haystack))
        },
    );

    eval_ctx.engine_mut().register_fn(
        "regex_replace",
        |pattern: ImmutableString,
         haystack: ImmutableString,
         replacement: ImmutableString|
         -> Result<String, Box<EvalAltResult>> {
            Ok(create_regex(&pattern)?
                .replace_all(&haystack, replacement.as_str())
                .to_string())
        },
    );
    Ok(())
}

pub fn setup_impure_functions(eval_ctx: &mut EvalCtx) -> Result<()> {
    eval_ctx.engine_mut().register_fn(
        "command_available",
        |name: String| -> Result<bool, Box<EvalAltResult>> {
            Ok(match which::which_all_global(name) {
                Ok(mut iter) => iter.next().is_some(),
                Err(err) => {
                    tracing::warn!("Error checking if command is available: {}", err);
                    false
                }
            })
        },
    );
    eval_ctx.engine_mut().register_fn(
        "env",
        |name: String, default: String| -> Result<String, Box<EvalAltResult>> {
            Ok(std::env::var(name).unwrap_or(default))
        },
    );
    eval_ctx
        .engine_mut()
        .register_fn("path_exists", |p: String| PathBuf::from(p).exists());
    eval_ctx
        .engine_mut()
        .register_fn("path_is_dir", |p: String| {
            fs_err::metadata(p).map(|m| m.is_dir()).unwrap_or(false)
        });
    eval_ctx
        .engine_mut()
        .register_fn("path_is_file", |p: String| {
            fs_err::metadata(p).map(|m| m.is_file()).unwrap_or(false)
        });
    eval_ctx.engine_mut().register_fn("read_file", |p: String| {
        fs_err::read_to_string(p).unwrap_or_default()
    });
    Ok(())
}

#[cfg(test)]
mod test {
    use testresult::TestResult;

    use crate::script::eval_ctx::EvalCtx;

    #[test]
    pub fn test_inspect() -> TestResult {
        let mut eval_ctx = EvalCtx::new_empty();
        super::setup_pure_functions(&mut eval_ctx)?;
        assert_eq!("[1, 2]", eval_ctx.eval_rhai("test", "[1, 2]")?.to_string());

        Ok(())
    }

    #[test]
    pub fn test_replace() -> TestResult {
        let mut eval_ctx = EvalCtx::new_empty();
        super::setup_tag_functions(&mut eval_ctx)?;
        assert_eq!(
            "foo:'xxx'",
            eval_ctx
                .eval_text_transformation("foo:'aaa'", "replace(`'.*'`, `'xxx'`)")?
                .to_string()
        );
        assert_eq!(
            "foo:'aaa'",
            eval_ctx
                .eval_text_transformation("foo:'aaa'", "replace(`'.*'`, `xxx`)")?
                .to_string(),
            "replace performed non-reversible replacement",
        );
        Ok(())
    }
    #[test]
    pub fn test_replace_in() -> TestResult {
        let mut eval_ctx = EvalCtx::new_empty();
        super::setup_tag_functions(&mut eval_ctx)?;
        assert_eq!(
            "foo:'xxx'",
            eval_ctx
                .eval_text_transformation("foo:'aaa'", "replace_in(`'`, `xxx`)")?
                .to_string()
        );
        assert_eq!(
            "foo:'aaa'",
            eval_ctx
                .eval_text_transformation("foo:'aaa'", "replace_in(`'`, `x'xx`)")?
                .to_string(),
            "replace performed non-reversible replacement",
        );
        Ok(())
    }

    #[test]
    pub fn test_replace_color() -> TestResult {
        let mut eval_ctx = EvalCtx::new_empty();
        super::setup_tag_functions(&mut eval_ctx)?;
        assert_eq!(
            "foo: #00ff00",
            eval_ctx
                .eval_text_transformation("foo: #ff0000", "replace_color(`#00ff00`)")?
                .to_string(),
        );
        assert_eq!(
            "foo: #00ff0000",
            eval_ctx
                .eval_text_transformation("foo: #ff0000", "replace_color(`#00ff0000`)")?
                .to_string(),
        );
        assert_eq!(
            "foo: #ff0000",
            eval_ctx
                .eval_text_transformation("foo: #ff0000", "replace_color(`00ff00`)")?
                .to_string(),
            "replace_color performed non-reversible replacement",
        );
        assert_eq!(
            "foo: #ff0000",
            eval_ctx
                .eval_text_transformation("foo: #ff0000", "replace_color(`bad color`)")?
                .to_string(),
            "replace_color performed non-reversible replacement",
        );
        Ok(())
    }
}
