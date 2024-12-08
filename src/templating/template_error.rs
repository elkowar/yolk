use miette::SourceSpan;

use crate::script::lua_error::LuaError;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum TemplateError {
    #[error("Error evaluating lua")]
    LuaEvalError {
        #[source]
        lua_error: LuaError,
        #[label(primary, "here")]
        error_span: Option<SourceSpan>,
    },
    #[error("Failed to evaluate template")]
    Multiple(#[related] Vec<TemplateError>),
}

impl TemplateError {
    pub fn from_lua_error(lua_error: LuaError, lua_expr_span: impl Into<SourceSpan>) -> Self {
        match lua_error {
            LuaError::SourceError { ref span, .. } => {
                let lua_expr_span = lua_expr_span.into();
                let lua_start = lua_expr_span.offset() + span.start;
                let lua_end = lua_expr_span.offset() + span.end;
                Self::LuaEvalError {
                    lua_error,
                    error_span: Some((lua_start..lua_end).into()),
                }
            }
            lua_error => Self::LuaEvalError {
                lua_error,
                error_span: None,
            },
        }
    }
}
