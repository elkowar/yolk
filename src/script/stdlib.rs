use miette::{IntoDiagnostic, Result};
use rhai::{Dynamic, EvalAltResult, ImmutableString, Map, NativeCallContext};
use std::path::PathBuf;

use regex::Regex;

use crate::{templating::template_error::TemplateError, yolk::EvalMode};

use super::eval_ctx::EvalCtx;
use cached::proc_macro::cached;

type IStr = ImmutableString;
type Ncc<'a> = NativeCallContext<'a>;

pub fn setup(eval_mode: EvalMode, eval_ctx: &mut EvalCtx) -> Result<(), TemplateError> {
    setup_environment_stuff(eval_mode, eval_ctx)?;
    setup_utilities(eval_ctx)?;
    setup_tag_functions(eval_ctx)?;
    Ok(())
}

fn setup_utilities(eval_ctx: &mut EvalCtx) -> Result<(), TemplateError> {
    eval_ctx.engine_mut().register_fn(
        "regex_match",
        |pattern: IStr, haystack: IStr| -> RhaiFnResult<_> {
            Ok(create_regex(&pattern)?.is_match(&haystack))
        },
    );

    eval_ctx.engine_mut().register_fn(
        "regex_replace",
        |pattern: IStr, haystack: IStr, replacement: IStr| -> RhaiFnResult<_> {
            Ok(create_regex(&pattern)?
                .replace_all(&haystack, &*replacement)
                .to_string())
        },
    );
    eval_ctx.engine_mut().register_fn(
        "regex_captures",
        |pattern: IStr, s: IStr| -> RhaiFnResult<_> {
            Ok(create_regex(&pattern)?.captures(s.as_str()).map(|caps| {
                (0..caps.len())
                    .map(|x| caps.get(x).unwrap().as_str().to_string())
                    .collect::<Vec<_>>()
            }))
        },
    );
    // eval_ctx
    //     .engine_mut()
    //     .register_fn("contains_value", |container: Dynamic, value: Dynamic| {
    //         let container = container
    //             .as_map_ref()
    //             .wrap_err("Not a container")
    //             .map_err(LuaError::Other)?;
    //         for pair in container.pairs::<Value, Value>() {
    //             if pair?.1.equals(&value)? {
    //                 return Ok(true);
    //             }
    //         }
    //         Ok(false)
    //     });
    // eval_ctx
    //     .engine_mut()
    //     .register_fn("contains_key", |container: Dynamic, value: Dynamic| {
    //         let container = container
    //             .as_table()
    //             .wrap_err("Not a container")
    //             .map_err(LuaError::Other)?;
    //         for pair in container.pairs::<Value, Value>() {
    //             if pair?.0.equals(&value)? {
    //                 return Ok(true);
    //             }
    //         }
    //         Ok(false)
    //     });

    // eval_ctx
    //     .engine_mut()
    //     .register_fn("from_json", |json: IStr| {
    //         let value: serde_json::Value = serde_json::from_str(&json)?;
    //         Ok(rhai.to_value(&value))
    //     });
    // eval_ctx
    //     .engine_mut()
    //     .register_fn("to_json", |value: Dynamic| {
    //         let json_value: serde_json::Value = lua.from_value(value)?;
    //         Ok(serde_json::to_string(&json_value).unwrap())
    //     });

    eval_ctx
        .engine_mut()
        .register_fn("color_hex_to_rgb", |hex_string: IStr| -> RhaiFnResult<_> {
            let (r, g, b, a) = color_hex_to_rgb(&hex_string)?;
            let mut map = Map::new();
            map.insert("r".to_string().into(), Dynamic::from_int(r as i64));
            map.insert("g".to_string().into(), Dynamic::from_int(g as i64));
            map.insert("b".to_string().into(), Dynamic::from_int(b as i64));
            map.insert("a".to_string().into(), Dynamic::from_int(a as i64));
            Ok(map)
        });

    eval_ctx.engine_mut().register_fn(
        "color_hex_to_rgb_str",
        |hex_string: IStr| -> RhaiFnResult<_> {
            let (r, g, b, _) = color_hex_to_rgb(&hex_string)?;
            Ok(format!("rgb({r}, {g}, {b})"))
        },
    );
    eval_ctx.engine_mut().register_fn(
        "color_hex_to_rgba_str",
        |hex_string: IStr| -> RhaiFnResult<_> {
            let (r, g, b, a) = color_hex_to_rgb(&hex_string)?;
            Ok(format!("rgba({r}, {g}, {b}, {a})"))
        },
    );
    eval_ctx
        .engine_mut()
        .register_fn("color_rgb_to_hex", |rgb_table: Map| -> RhaiFnResult<_> {
            let r = rgb_table
                .get("r")
                .map(dynamic_to_u8)
                .transpose()?
                .unwrap_or(0);
            let g = rgb_table
                .get("g")
                .map(dynamic_to_u8)
                .transpose()?
                .unwrap_or(0);
            let b = rgb_table
                .get("b")
                .map(dynamic_to_u8)
                .transpose()?
                .unwrap_or(0);
            let a = rgb_table.get("a").map(dynamic_to_u8).transpose()?;
            match a {
                Some(a) => Ok(format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)),
                None => Ok(format!("#{:02x}{:02x}{:02x}", r, g, b)),
            }
        });

    // TODO: Add deepcopy
    Ok(())
}

fn dynamic_to_u8(x: &Dynamic) -> RhaiFnResult<u8> {
    let int = x
        .as_int()
        .map_err(|actual| format!("Failed to convert {actual} to int"))?;
    let int = int
        .try_into()
        .map_err(|_| format!("Failed to convert {int} to u8"))?;
    Ok(int)
}

fn color_hex_to_rgb(hex_string: &str) -> Result<(u8, u8, u8, u8), Box<EvalAltResult>> {
    let hex = hex_string.trim_start_matches('#');
    if hex.len() != 6 && hex.len() != 8 {
        return Err(format!("Invalid hex color: {}", hex_string).into());
    }
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
    let a = if hex.len() == 8 {
        u8::from_str_radix(&hex[6..8], 16).map_err(|e| e.to_string())?
    } else {
        255
    };
    Ok((r, g, b, a))
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

type RhaiFnResult<T> = Result<T, Box<EvalAltResult>>;

pub fn setup_environment_stuff(
    eval_mode: EvalMode,
    eval_ctx: &mut EvalCtx,
) -> Result<(), TemplateError> {
    eval_ctx
        .engine_mut()
        .register_fn("command_available", move |name: IStr| -> RhaiFnResult<_> {
            if_canonical_return!(eval_mode);
            Ok(match which::which_all_global(&*name) {
                Ok(mut iter) => iter.next().is_some(),
                Err(err) => {
                    tracing::warn!("Error checking if command is available: {}", err);
                    false
                }
            })
        });
    eval_ctx
        .engine_mut()
        .register_fn("env", move |name: IStr, default: IStr| -> RhaiFnResult<_> {
            if_canonical_return!(eval_mode);
            Ok(std::env::var(&*name).map(|x| x.into()).unwrap_or(default))
        });
    eval_ctx
        .engine_mut()
        .register_fn("path_exists", move |p: IStr| -> RhaiFnResult<_> {
            if_canonical_return!(eval_mode);
            Ok(PathBuf::from(&*p).exists())
        });
    eval_ctx
        .engine_mut()
        .register_fn("path_is_dir", move |p: String| -> RhaiFnResult<_> {
            if_canonical_return!(eval_mode);
            Ok(fs_err::metadata(p).map(|m| m.is_dir()).unwrap_or(false))
        });
    eval_ctx
        .engine_mut()
        .register_fn("path_is_file", move |p: String| -> RhaiFnResult<_> {
            if_canonical_return!(eval_mode);
            Ok(fs_err::metadata(p).map(|m| m.is_file()).unwrap_or(false))
        });
    eval_ctx
        .engine_mut()
        .register_fn("read_file", move |p: String| -> RhaiFnResult<_> {
            if_canonical_return!(eval_mode);
            Ok(fs_err::read_to_string(p).unwrap_or_default())
        });
    eval_ctx.engine_mut().register_fn(
        "read_dir",
        move |p: String| -> Result<Vec<_>, Box<EvalAltResult>> {
            if_canonical_return!(eval_mode);
            fs_err::read_dir(p)
                .into_diagnostic()
                .map_err(|e| e.to_string())?
                .map(|x| {
                    Ok(x.map_err(|e| e.to_string())?
                        .path()
                        .to_string_lossy()
                        .to_string())
                })
                .collect()
        },
    );
    Ok(())
}

fn setup_tag_functions(eval_ctx: &mut EvalCtx) -> Result<(), TemplateError> {
    /// Simple regex replacement that will refuse to run a non-reversible replacement.
    /// If the replacement value is non-reversible, will return the original text and log a warning.
    fn tag_text_replace(text: &str, pattern: &str, replacement: &str) -> RhaiFnResult<String> {
        let pattern = create_regex(pattern)?;
        let after_replace = pattern.replace(text, replacement);
        if let Some(original_value) = pattern.find(text) {
            let original_value = original_value.as_str();
            let reversed = pattern.replace(&after_replace, original_value);
            if reversed != text {
                return Err(format!(
                    "Refusing to run non-reversible replacement: {text} -> {after_replace}",
                )
                .into());
            }
        };
        Ok(after_replace.to_string())
    }

    let f = |ctx: Ncc, regex: IStr, replacement: IStr| -> RhaiFnResult<_> {
        let text: IStr = ctx.call_fn("get_yolk_text", ())?;
        tag_text_replace(&text, &regex, &replacement)
    };
    eval_ctx.engine_mut().register_fn("replace_re", f);
    eval_ctx.engine_mut().register_fn("rr", f);

    let f = |ctx: Ncc, between: IStr, replacement: IStr| -> RhaiFnResult<_> {
        let text: IStr = ctx.call_fn("get_yolk_text", ())?;
        let regex = format!("{between}[^{between}]*{between}");
        tag_text_replace(&text, &regex, &format!("{between}{replacement}{between}"))
    };
    eval_ctx.engine_mut().register_fn("replace_in", f);
    eval_ctx.engine_mut().register_fn("rin", f);

    let f = |ctx: Ncc, left: IStr, right: IStr, replacement: IStr| -> RhaiFnResult<_> {
        let text: IStr = ctx.call_fn("get_yolk_text", ())?;
        let regex = format!("{left}[^{right}]*{right}");
        tag_text_replace(&text, &regex, &format!("{left}{replacement}{right}"))
    };
    eval_ctx.engine_mut().register_fn("replace_between", f);
    eval_ctx.engine_mut().register_fn("rbet", f);

    let f = |ctx: Ncc, replacement: IStr| -> RhaiFnResult<_> {
        let text: IStr = ctx.call_fn("get_yolk_text", ())?;
        tag_text_replace(
            &text,
            r"#[\da-fA-F]{6}([\da-fA-F]{2})?",
            replacement.as_ref(),
        )
    };
    eval_ctx.register_fn("replace_color", f);
    eval_ctx.register_fn("rcol", f);

    let f = |ctx: Ncc, replacement: Dynamic| -> RhaiFnResult<_> {
        let text: IStr = ctx.call_fn("get_yolk_text", ())?;
        tag_text_replace(&text, r"-?\d+(?:\.\d+)?", &replacement.to_string())
    };
    eval_ctx.register_fn("replace_number", f);
    eval_ctx.register_fn("rnum", f);

    let f = |ctx: Ncc, replacement: IStr| -> RhaiFnResult<_> {
        let text: IStr = ctx.call_fn("get_yolk_text", ())?;
        let mut result = tag_text_replace(&text, r#"".*""#, &format!("\"{replacement}\""))?;
        if result == text {
            result = tag_text_replace(&text, r#"`.*`"#, &format!("`{replacement}`"))?;
        }
        if result == text {
            result = tag_text_replace(&text, r#"'.*'"#, &format!("'{replacement}'"))?;
        }
        Ok(result)
    };
    eval_ctx.register_fn("replace_quoted", f);
    eval_ctx.register_fn("rqot", f);

    eval_ctx.register_fn(
        "replace_value",
        |ctx: Ncc, replacement: IStr| -> RhaiFnResult<_> {
            let text: IStr = ctx.call_fn("get_yolk_text", ())?;
            let regex = create_regex(r"([=:])( *)([^\s]+)").unwrap();

            if let Some(caps) = regex.captures(&text) {
                let full_match = caps.get(0).unwrap();
                let equals = caps.get(1).unwrap();
                let space = caps.get(2).unwrap();
                let new_value = regex.replace(
                    &text,
                    format!("{}{}{}", equals.as_str(), space.as_str(), replacement),
                );
                if regex.replace(&new_value, full_match.as_str()) == *text {
                    Ok(new_value.to_string())
                } else {
                    Err(format!(
                        "Refusing to run non-reversible replacement: {text} -> {new_value}",
                    )
                    .into())
                }
            } else {
                Ok(text.into())
            }
        },
    );
    // eval_ctx.set_global(
    //     "rv",
    //     eval_ctx.get_global::<mlua::Function>("replace_value")?,
    // )?;
    Ok(())
}

#[cached(key = "String", convert = r#"{s.to_string()}"#, result)]
fn create_regex(s: &str) -> RhaiFnResult<Regex> {
    Ok(Regex::new(s).map_err(|e| e.to_string())?)
}

#[cfg(test)]
mod test {
    use crate::util::TestResult;
    use miette::IntoDiagnostic;
    use rhai::Variant;

    use crate::{script::eval_ctx::EvalCtx, yolk::EvalMode};

    pub fn run_expr<T: Variant + Clone>(lua: &str) -> miette::Result<T> {
        let mut eval_ctx = EvalCtx::new_in_mode(EvalMode::Local)?;
        Ok(eval_ctx.eval_rhai::<T>(lua)?)
    }
    pub fn run_tag_expr(text: &str, lua: &str) -> miette::Result<String> {
        let text = text.to_string();
        let mut eval_ctx = EvalCtx::new_in_mode(EvalMode::Local)?;
        eval_ctx
            .engine_mut()
            .register_fn("get_yolk_text", move || text.clone());
        eval_ctx.eval_rhai::<String>(lua).into_diagnostic()
    }

    #[test]
    pub fn test_regex_captures() -> TestResult {
        assert_eq!(
            Some(vec![
                "<aaaXb>".to_string(),
                "aaa".to_string(),
                "b".to_string()
            ]),
            run_expr::<Option<Vec<String>>>("regex_captures(`<(.*)X(.)>`, `foo <aaaXb> bar`)")?
        );
        assert_eq!(
            None,
            run_expr::<Option<Vec<String>>>("regex_captures(`<(.*)X(.)>`, `asdf`)")?
        );
        Ok(())
    }

    // #[test]
    // pub fn test_to_json() -> TestResult {
    //     assert_eq!(
    //         r#"{"a":2,"b":[1,2,3]}"#,
    //         run_lua::<String>("to_json(#{a: 2, b: [1, 2, 3]})")?,
    //     );
    //     assert_eq!(
    //         r#"{ 1, 2 }"#,
    //         run_lua::<String>(r#"inspect.inspect(from_json('[1, 2]'))"#)?,
    //     );
    //     Ok(())
    // }

    #[test]
    pub fn test_replace() -> TestResult {
        assert_eq!(
            "foo:'xxx'",
            run_tag_expr("foo:'aaa'", "replace_re(`'.*'`, `'xxx'`)")?
        );
        assert!(
            run_tag_expr("foo:'aaa'", "replace_re(`'.*'`, `xxx`)").is_err(),
            "replace performed non-reversible replacement",
        );
        Ok(())
    }

    #[test]
    pub fn test_replace_in() -> TestResult {
        assert_eq!(
            "foo:'xxx'",
            run_tag_expr("foo:'aaa'", "replace_in(`'`, `xxx`)")?
        );
        assert!(
            run_tag_expr("foo:'aaa'", "replace_in(`'`, `x'xx`)").is_err(),
            "replace performed non-reversible replacement",
        );
        Ok(())
    }

    #[test]
    pub fn test_replace_color() -> TestResult {
        assert_eq!(
            "foo: #00ff00",
            run_tag_expr("foo: #ff0000", "replace_color(`#00ff00`)")?,
        );
        assert_eq!(
            "foo: #00ff0000",
            run_tag_expr("foo: #ff0000", "replace_color(`#00ff0000`)")?,
        );
        assert!(
            run_tag_expr("foo: #ff0000", "replace_color(`00ff00`)").is_err(),
            "replace_color performed non-reversible replacement",
        );
        assert!(
            run_tag_expr("foo: #ff0000", "replace_color(`bad color`)").is_err(),
            "replace_color performed non-reversible replacement",
        );
        Ok(())
    }

    #[test]
    pub fn test_replace_quoted() -> TestResult {
        assert_eq!(
            "foo: 'new'",
            run_tag_expr("foo: 'old'", "replace_quoted(`new`)")?,
        );
        assert_eq!(
            "foo: \"new\"",
            run_tag_expr("foo: \"old\"", "replace_quoted(`new`)")?,
        );
        assert_eq!(
            "foo: `new`",
            run_tag_expr("foo: `old`", "replace_quoted(`new`)")?,
        );
        Ok(())
    }

    #[test]
    pub fn test_replace_value() -> TestResult {
        assert_eq!(
            "foo: xxx # baz",
            run_tag_expr("foo: bar # baz", "replace_value(`xxx`)")?,
        );
        assert!(
            run_tag_expr("foo: bar # baz", "replace_value(`x xx`)").is_err(),
            "replace_value performed non-reversible replacement",
        );
        Ok(())
    }

    #[test]
    pub fn test_replace_number() -> TestResult {
        assert_eq!(
            "foo 999 bar",
            run_tag_expr("foo 123 bar", "replace_number(999)")?,
        );
        assert_eq!(
            "foo 99.9 bar",
            run_tag_expr("foo 1.23 bar", "replace_number(99.9)")?,
        );
        assert!(
            run_tag_expr("foo 99.9 bar", "replace_number(`hi`)").is_err(),
            "replace_value performed non-reversible replacement",
        );
        Ok(())
    }
}
