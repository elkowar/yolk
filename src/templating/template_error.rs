use miette::{Diagnostic, LabeledSpan, SourceSpan};

use crate::script::lua_error::LuaError;

use super::parse_error::YolkParseFailure;

#[derive(Debug, thiserror::Error)]
pub enum TemplateError {
    #[error(transparent)]
    ParseError(#[from] YolkParseFailure),
    #[error("{lua_error}")]
    LuaEvalError {
        lua_error: LuaError,
        error_span: Option<SourceSpan>,
    },
    #[error("Failed to evaluate template")]
    Multiple(Vec<TemplateError>),
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

impl TemplateError {
    pub fn labels_vec(&self) -> Vec<LabeledSpan> {
        match self {
            TemplateError::ParseError(err) => {
                if let Some(labels) = err.labels() {
                    labels.collect()
                } else {
                    vec![]
                }
            }
            TemplateError::LuaEvalError {
                error_span: Some(span),
                ..
            } => {
                vec![LabeledSpan::new_primary_with_span(
                    Some("here".to_string()),
                    *span,
                )]
            }
            TemplateError::LuaEvalError {
                error_span: None, ..
            } => {
                vec![]
            }
            TemplateError::Multiple(_) => {
                vec![]
            }
        }
    }
}

impl Diagnostic for TemplateError {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        None
    }

    fn severity(&self) -> Option<miette::Severity> {
        None
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        None
    }

    fn url<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        None
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        None
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        Some(Box::new(self.labels_vec().into_iter()))
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        match self {
            TemplateError::Multiple(errors) => {
                Some(Box::new(errors.iter().map(|x| x as &dyn Diagnostic)))
            }
            _ => None,
        }
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        None
    }
}
