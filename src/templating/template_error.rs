use miette::SourceSpan;

use crate::script::lua_error::RhaiError;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum TemplateError {
    #[error("Error evaluating lua")]
    Rhai {
        #[source]
        error: RhaiError,
        #[label(primary, "here")]
        error_span: Option<SourceSpan>,
    },
    #[error("Failed to evaluate template")]
    Multiple(#[related] Vec<TemplateError>),
}

impl TemplateError {
    pub fn from_rhai(error: RhaiError, expr_span: impl Into<SourceSpan>) -> Self {
        match error {
            RhaiError::SourceError { ref span, .. } => {
                let expr_span = expr_span.into();
                let start = expr_span.offset() + span.start;
                let end = expr_span.offset() + span.end;
                Self::Rhai {
                    error,
                    error_span: Some((start..end).into()),
                }
            }
            error => Self::Rhai {
                error,
                error_span: None,
            },
        }
    }
}
