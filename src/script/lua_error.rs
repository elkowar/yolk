use std::{ops::Range, sync::LazyLock};

use miette::Diagnostic;
use regex::Regex;

static LUA_ERROR_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^.*: \[.*?\]:(\d+): (.*)$").unwrap());

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("Error in lua code: {}", .message)]
pub struct LuaError {
    message: String,
    #[label()]
    span: Range<usize>,
    origin: mlua::Error,
}

impl LuaError {
    pub fn from_mlua_with_source(source_code: &str, err: mlua::Error) -> Self {
        let mut msg = err.to_string();
        let mut span = 0..0;
        if let Some(caps) = LUA_ERROR_REGEX.captures(&msg) {
            let line_nr = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let err_msg = caps.get(2).unwrap().as_str();
            let offset_start = source_code
                .lines()
                .take(line_nr - 1)
                .map(|x| x.len())
                .sum::<usize>();
            let offset_end = offset_start
                + source_code
                    .lines()
                    .nth(line_nr - 1)
                    .map(|x| x.len())
                    .unwrap_or_default();
            span = offset_start..offset_end;
            msg = err_msg.to_string();
        }
        Self {
            message: msg,
            span,
            origin: err,
        }
    }
}
