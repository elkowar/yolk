use miette::{IntoDiagnostic, Result};
use rhai::{Dynamic, EvalAltResult, ImmutableString, Map, NativeCallContext};
use rhai::{FuncRegistration, Module};
use std::path::PathBuf;

use regex::Regex;

use crate::yolk::EvalMode;

use cached::proc_macro::cached;

use super::sysinfo::{SystemInfo, SystemInfoPaths};

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

type IStr = ImmutableString;
type Ncc<'a> = NativeCallContext<'a>;

pub fn global_stuff() -> Module {
    let mut module = Module::new();

    FuncRegistration::new("to_string")
        .in_global_namespace()
        .set_into_module(&mut module, |x: &mut SystemInfo| format!("{x:#?}"));
    FuncRegistration::new("to_debug")
        .in_global_namespace()
        .set_into_module(&mut module, |x: &mut SystemInfo| format!("{x:?}"));
    FuncRegistration::new("to_string")
        .in_global_namespace()
        .set_into_module(&mut module, |x: &mut SystemInfoPaths| format!("{x:#?}"));
    FuncRegistration::new("to_debug")
        .in_global_namespace()
        .set_into_module(&mut module, |x: &mut SystemInfoPaths| format!("{x:?}"));
    module
}

pub fn utils_module() -> Module {
    let mut module = Module::new();
    module.set_doc(indoc::indoc! {r"
        # Utility functions

        A collection of utility functions
    "});

    let regex_match = |pattern: String, haystack: String| -> Result<bool, Box<EvalAltResult>> {
        Ok(create_regex(&pattern)?.is_match(&haystack))
    };
    FuncRegistration::new("regex_match")
        .with_comments(["/// Check if a given string matches a given regex pattern."])
        .with_params_info(["pattern: &str", "haystack: &str", "Result<bool>"])
        .in_global_namespace()
        .set_into_module(&mut module, regex_match);

    let regex_replace = |pattern: String,
                         haystack: String,
                         replacement: String|
     -> Result<String, Box<EvalAltResult>> {
        Ok(create_regex(&pattern)?
            .replace_all(&haystack, &*replacement)
            .to_string())
    };
    FuncRegistration::new("regex_replace")
        .with_comments(["/// Replace a regex pattern in a string with a replacement."])
        .with_params_info([
            "pattern: &str",
            "haystack: &str",
            "replacement: &str",
            "Result<String>",
        ])
        .in_global_namespace()
        .set_into_module(&mut module, regex_replace);

    let regex_captures =
        |pattern: String, s: String| -> Result<Option<Vec<String>>, Box<EvalAltResult>> {
            Ok(create_regex(&pattern)?.captures(s.as_str()).map(|caps| {
                (0..caps.len())
                    .map(|x| caps.get(x).unwrap().as_str().to_string())
                    .collect::<Vec<_>>()
            }))
        };
    FuncRegistration::new("regex_captures")
        .with_comments([
            "/// Match a string against a regex pattern and return the capture groups as a list.",
        ])
        .with_params_info(["pattern: &str", "s: &str", "Result<Option<Vec<String>>>"])
        .in_global_namespace()
        .set_into_module(&mut module, regex_captures);

    let rhai_color_hex_to_rgb = |hex_string: String| -> Result<Map, Box<EvalAltResult>> {
        let (r, g, b, a) = color_hex_to_rgb(&hex_string)?;
        let mut map = Map::new();
        map.insert("r".to_string().into(), Dynamic::from_int(r as i64));
        map.insert("g".to_string().into(), Dynamic::from_int(g as i64));
        map.insert("b".to_string().into(), Dynamic::from_int(b as i64));
        map.insert("a".to_string().into(), Dynamic::from_int(a as i64));
        Ok(map)
    };
    FuncRegistration::new("color_hex_to_rgb")
        .with_comments(["/// Convert a hex color string to an RGB map."])
        .with_params_info(["hex_string: &str", "Result<Map>"])
        .in_global_namespace()
        .set_into_module(&mut module, rhai_color_hex_to_rgb);

    let color_hex_to_rgb_str = |hex_string: String| -> Result<String, Box<EvalAltResult>> {
        let (r, g, b, _) = color_hex_to_rgb(&hex_string)?;
        Ok(format!("rgb({r}, {g}, {b})"))
    };
    FuncRegistration::new("color_hex_to_rgb_str")
        .with_comments(["/// Convert a hex color string to an RGB string."])
        .with_params_info(["hex_string: &str", "Result<String>"])
        .in_global_namespace()
        .set_into_module(&mut module, color_hex_to_rgb_str);

    let color_hex_to_rgba_str = |hex_string: String| -> Result<String, Box<EvalAltResult>> {
        let (r, g, b, a) = color_hex_to_rgb(&hex_string)?;
        Ok(format!("rgba({r}, {g}, {b}, {a})"))
    };
    FuncRegistration::new("color_hex_to_rgba_str")
        .with_comments(["/// Convert a hex color string to an RGBA string."])
        .with_params_info(["hex_string: &str", "Result<String>"])
        .in_global_namespace()
        .set_into_module(&mut module, color_hex_to_rgba_str);

    let color_rgb_to_hex = |rgb_table: Map| -> Result<String, Box<EvalAltResult>> {
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
    };
    FuncRegistration::new("color_rgb_to_hex")
        .with_comments(["/// Convert an RGB map to a hex color string."])
        .with_params_info(["rgb_table: Map", "Result<String>"])
        .in_global_namespace()
        .set_into_module(&mut module, color_rgb_to_hex);

    module
}

pub fn io_module(eval_mode: EvalMode) -> Module {
    use which::which_all_global;
    let mut module = Module::new();
    module.set_doc(indoc::indoc! {r"
        # IO Functions

        A collection of functions that can read the environment and filesystem.
        These return standardized values in canonical mode.
    "});

    let command_available = move |name: IStr| -> RhaiFnResult<bool> {
        if_canonical_return!(eval_mode, false);
        Ok(match which_all_global(&*name) {
            Ok(mut iter) => iter.next().is_some(),
            Err(err) => {
                tracing::warn!("Error checking if command is available: {}", err);
                false
            }
        })
    };
    FuncRegistration::new("command_available")
        .with_comments(["/// Check if a given command is available"])
        .with_params_info(["name: &str", "Result<bool>"])
        .set_into_module(&mut module, command_available);

    let env = move |name: IStr, def: IStr| -> RhaiFnResult<IStr> {
        if_canonical_return!(eval_mode, def.clone());
        Ok(std::env::var(&*name).map(|x| x.into()).unwrap_or(def))
    };
    FuncRegistration::new("env")
        .with_comments(["/// Read an environment variable, or return the given default"])
        .with_params_info(["name: &str", "def: &str", "Result<String>"])
        .set_into_module(&mut module, env);

    let path_exists = move |p: IStr| -> RhaiFnResult<bool> {
        if_canonical_return!(eval_mode, false);
        Ok(PathBuf::from(&*p).exists())
    };
    FuncRegistration::new("path_exists")
        .with_comments(["/// Check if something exists at a given path"])
        .with_params_info(["p: &str", "Result<bool>"])
        .set_into_module(&mut module, path_exists);

    let path_is_dir = move |p: String| -> RhaiFnResult<bool> {
        if_canonical_return!(eval_mode, false);
        Ok(fs_err::metadata(p).map(|m| m.is_dir()).unwrap_or(false))
    };
    FuncRegistration::new("path_is_dir")
        .with_comments(["/// Check if the given path is a directory"])
        .with_params_info(["p: &str", "Result<bool>"])
        .set_into_module(&mut module, path_is_dir);

    let path_is_file = move |p: String| -> RhaiFnResult<bool> {
        if_canonical_return!(eval_mode, false);
        Ok(fs_err::metadata(p).map(|m| m.is_file()).unwrap_or(false))
    };
    FuncRegistration::new("path_is_file")
        .with_comments(["/// Check if the given path is a file"])
        .with_params_info(["p: &str", "Result<bool>"])
        .set_into_module(&mut module, path_is_file);

    let read_file = move |p: String| -> RhaiFnResult<String> {
        if_canonical_return!(eval_mode, String::new());
        Ok(fs_err::read_to_string(p).unwrap_or_default())
    };
    FuncRegistration::new("read_file")
        .with_comments(["/// Read the contents of a given file"])
        .with_params_info(["p: &str", "Result<String>"])
        .set_into_module(&mut module, read_file);

    let read_dir = move |p: String| -> RhaiFnResult<Vec<String>> {
        if_canonical_return!(eval_mode, vec![]);
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
    };
    FuncRegistration::new("read_dir")
        .with_comments(["/// Read the children of a given dir"])
        .with_params_info(["p: &str", "Result<Vec<String>>"])
        .set_into_module(&mut module, read_dir);

    module
}

pub fn tag_module() -> Module {
    use indoc::indoc;
    let mut module = rhai::Module::new();
    module.set_doc(indoc::indoc! {r"
        # Template tag functions

        Yolk template tags simply execute rhai functions that transform the block of text the tag operates on.

        Quick reminder: Yolk has three different types of tags, that differ only in what text they operate on:

        - Next-line tags (`{# ... #}`): These tags operate on the line following the tag.
        - Inline tags (`{< ... >}`): These tags operate on everything before the tag within the same line.
        - Block tags (`{% ... %} ... {% end %}`): These tags operate on everything between the tag and the corresponding `{% end %}` tag.

        Inside these tags, you can call any of Yolks template tag functions (Or, in fact, any rhai expression that returns a string).
    "});

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
    FuncRegistration::new("replace_re")
        .with_comments([indoc! {"
            /// **shorthand**: `rr`.
            ///
            /// Replaces all occurrences of a Regex `pattern` with `replacement` in the text.
            ///
            /// #### Example
            ///
            /// ```handlebars
            /// ui_font = \"Arial\" # {< replace_re(`\".*\"`, `\"{data.font.ui}\"`) >}
            /// ```
            ///
            /// Note that the replacement value needs to contain the quotes, as those are also matched against in the regex pattern.
            /// Otherwise, we would end up with invalid toml.
        "}])
        .with_params_info(["regex: &str", "replacement: &str", "Result<String>"])
        .set_into_module(&mut module, f);
    FuncRegistration::new("rr").set_into_module(&mut module, f);

    let f = |ctx: Ncc, between: IStr, replacement: IStr| -> RhaiFnResult<_> {
        let text: IStr = ctx.call_fn("get_yolk_text", ())?;
        let regex = format!("{between}[^{between}]*{between}");
        tag_text_replace(&text, &regex, &format!("{between}{replacement}{between}"))
    };
    FuncRegistration::new("replace_in")
        .with_comments([indoc! {"
            /// **shorthand**: `rin`.
            ///
            /// Replaces the text between two delimiters with the `replacement`.
            ///
            /// #### Example
            ///
            /// ```toml
            /// ui_font = \"Arial\" # {< replace_in(`\"`, data.font.ui) >}
            /// ```
            ///
            /// Note: we don't need to include the quotes in the replacement here.
        "}])
        .with_params_info(["between: &str", "replacement: &str", "Result<String>"])
        .in_global_namespace()
        .set_into_module(&mut module, f);
    FuncRegistration::new("rin").set_into_module(&mut module, f);

    let f = |ctx: Ncc, left: IStr, right: IStr, replacement: IStr| -> RhaiFnResult<_> {
        let text: IStr = ctx.call_fn("get_yolk_text", ())?;
        let regex = format!("{left}[^{right}]*{right}");
        tag_text_replace(&text, &regex, &format!("{left}{replacement}{right}"))
    };
    FuncRegistration::new("replace_between")
        .with_comments([indoc! {"
            /// **shorthand**: `rbet`.
            ///
            /// Replaces the text between two delimiters with the `replacement`.
            ///
            /// #### Example
            ///
            /// ```handlebars
            /// ui_font = (Arial) # {< replace_between(`(`, `)`, data.font.ui) >}
            /// ```
            ///
            /// Note: we don't need to include the quotes in the replacement here.
        "}])
        .with_params_info([
            "left: &str",
            "right: &str",
            "replacement: &str",
            "Result<String>",
        ])
        .in_global_namespace()
        .set_into_module(&mut module, f);
    FuncRegistration::new("rbet").set_into_module(&mut module, f);

    let f = |ctx: Ncc, replacement: IStr| -> RhaiFnResult<_> {
        let text: IStr = ctx.call_fn("get_yolk_text", ())?;
        tag_text_replace(
            &text,
            r"#[\da-fA-F]{6}([\da-fA-F]{2})?",
            replacement.as_ref(),
        )
    };
    FuncRegistration::new("replace_color")
        .with_comments([indoc! {"
            /// **shorthand**: `rcol`.
            ///
            /// Replaces a hex color value with a new hex color.
            ///
            /// #### Example
            ///
            /// ```handlebars
            /// background_color = \"#282828\" # {< replace_color(data.colors.bg) >}
            /// ```
        "}])
        .with_params_info(["replacement: &str", "Result<String>"])
        .in_global_namespace()
        .set_into_module(&mut module, f);
    FuncRegistration::new("rcol").set_into_module(&mut module, f);

    let f = |ctx: Ncc, replacement: Dynamic| -> RhaiFnResult<_> {
        let text: IStr = ctx.call_fn("get_yolk_text", ())?;
        tag_text_replace(&text, r"-?\d+(?:\.\d+)?", &replacement.to_string())
    };
    FuncRegistration::new("replace_number")
        .with_comments([indoc! {"
            /// **shorthand**: `rnum`.
            ///
            /// Replaces a number with another number.
            ///
            /// #### Example
            ///
            /// ```handlebars
            /// cursor_size = 32 # {< replace_number(data.cursor_size) >}
            /// ```
        "}])
        .with_params_info(["replacement: Dynamic", "Result<String>"])
        .in_global_namespace()
        .set_into_module(&mut module, f);
    FuncRegistration::new("rnum").set_into_module(&mut module, f);

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
    FuncRegistration::new("replace_quoted")
        .with_comments([indoc! {"
            /// **shorthand**: `rq`.
            ///
            /// Replaces a value between quotes with another value
            ///
            /// #### Example
            ///
            /// ```handlebars
            /// ui_font = \"Arial\" # {< replace_quoted(data.font.ui) >}
            /// ```
        "}])
        .with_params_info(["replacement: &str", "Result<String>"])
        .in_global_namespace()
        .set_into_module(&mut module, f);
    FuncRegistration::new("rq").set_into_module(&mut module, f);

    let f = |ctx: Ncc, replacement: IStr| -> RhaiFnResult<_> {
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
                Err(
                    format!("Refusing to run non-reversible replacement: {text} -> {new_value}",)
                        .into(),
                )
            }
        } else {
            Ok(text.into())
        }
    };
    FuncRegistration::new("replace_value")
        .with_comments([indoc! {"
            /// **shorthand**: `rv`.
            ///
            /// Replaces a value (without spaces) after a `:` or a `=` with another value
            ///
            /// #### Example
            ///
            /// ```handlebars
            /// ui_font = Arial # {< replace_value(data.font.ui) >}
            /// ```
        "}])
        .with_params_info(["replacement: &str", "Result<String>"])
        .in_global_namespace()
        .set_into_module(&mut module, f);
    FuncRegistration::new("rv").set_into_module(&mut module, f);

    module
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

type RhaiFnResult<T> = Result<T, Box<EvalAltResult>>;

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
