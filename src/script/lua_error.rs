use std::{ops::Range, sync::LazyLock};

use miette::{Diagnostic, NamedSource};
use regex::Regex;

static LUA_ERROR_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^.*: \[.*?\]:(\d+): (.*)$").unwrap());

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("Error in lua code: {}", .message)]
pub struct LuaError {
    pub message: String,
    #[label("here")]
    pub span: Range<usize>,
    origin: mlua::Error,
    #[source_code]
    pub source_code: Option<NamedSource<String>>,
}

impl LuaError {
    pub fn from_mlua_with_source(name: &str, source_code: &str, err: mlua::Error) -> Self {
        let mut msg = err.to_string();
        let mut span = 0..0;
        if let Some(caps) = LUA_ERROR_REGEX.captures(&msg) {
            let line_nr = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let err_msg = caps.get(2).unwrap().as_str();
            let offset_start = source_code
                .split_inclusive('\n')
                .take(line_nr - 1)
                .map(|x| x.len())
                .sum::<usize>();
            let offset_end = offset_start
                + source_code
                    .lines()
                    .nth(line_nr - 1)
                    .map(|x| x.len())
                    .unwrap_or_default();
            let indent = source_code[offset_start..]
                .chars()
                .take_while(|x| x.is_whitespace())
                .count();
            let offset_start = offset_start + indent;
            span = offset_start..offset_end;
            msg = err_msg.to_string();
        }
        Self {
            message: msg,
            span,
            origin: err,
            source_code: Some(NamedSource::new(name, source_code.to_string())),
        }
    }
}
