use std::sync::Arc;

use miette::{Diagnostic, NamedSource, Severity, SourceSpan};
use winnow::{
    error::{
        AddContext, ContextError, ErrorKind, FromRecoverableError, StrContext, StrContextValue,
    },
    stream::{Location, Stream},
};

#[derive(Debug, Diagnostic, Clone, Eq, PartialEq, thiserror::Error)]
#[error("Failed to parse yolk template file")]
pub struct YolkParseFailure {
    #[source_code]
    pub input: Arc<miette::NamedSource<String>>,
    #[related]
    pub diagnostics: Vec<YolkParseDiagnostic>,
}

impl YolkParseFailure {
    pub fn from_errs(errs: Vec<YolkParseError>, input: &str) -> YolkParseFailure {
        let src = Arc::new(NamedSource::new("file", input.to_string()));
        YolkParseFailure {
            input: src.clone(),
            diagnostics: errs
                .into_iter()
                .map(|e| YolkParseDiagnostic {
                    input: src.clone(),
                    span: e.span.unwrap_or_else(|| (0usize..0usize).into()),
                    label: e.label,
                    help: e.help,
                    severity: Severity::Error,
                    kind: if let Some(ctx) = e.context {
                        YolkParseErrorKind::Context(ctx)
                    } else {
                        YolkParseErrorKind::Other
                    },
                })
                .collect(),
        }
    }
}

#[derive(Debug, Diagnostic, Clone, Eq, PartialEq, thiserror::Error)]
#[error("{kind}")]
pub struct YolkParseDiagnostic {
    #[source_code]
    pub input: Arc<NamedSource<String>>,
    ///
    /// Offset in chars of the error.
    #[label("{}", label.unwrap_or("here"))]
    pub span: SourceSpan,

    /// Label text for this span. Defaults to `"here"`.
    pub label: Option<&'static str>,

    /// Suggestion for fixing the parser error.
    #[help]
    pub help: Option<&'static str>,

    /// Severity level for the Diagnostic.
    #[diagnostic(severity)]
    pub severity: miette::Severity,

    /// Specific error kind for this parser error.
    pub kind: YolkParseErrorKind,
}

#[derive(Debug, Diagnostic, Clone, Eq, PartialEq, thiserror::Error)]
pub enum YolkParseErrorKind {
    /// Generic parsing error.
    #[error("Expected {0}.")]
    #[diagnostic(code(yolk::parser::parse_component))]
    Context(&'static str),

    /// Generic unspecified error. If this is returned, the call site should
    /// be annotated with context, if possible.
    #[error("An unspecified parse error occurred.")]
    #[diagnostic(code(yolk::parser::other))]
    Other,
}

#[derive(Debug, Clone, Eq, PartialEq, Diagnostic, thiserror::Error)]
pub struct YolkParseError {
    pub context: Option<&'static str>,
    pub span: Option<SourceSpan>,
    pub label: Option<&'static str>,
    pub help: Option<&'static str>,
    pub kind: Option<YolkParseErrorKind>,
}

impl std::fmt::Display for YolkParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(kind) = &self.kind {
            write!(f, "{}:", kind)?;
        }
        if let Some(ctx) = &self.context {
            write!(f, "{}", ctx)?;
        }
        Ok(())
    }
}

impl<I: Stream> winnow::error::ParserError<I> for YolkParseError {
    fn from_error_kind(_input: &I, _kind: ErrorKind) -> Self {
        Self {
            span: None,
            label: None,
            help: None,
            context: None,
            kind: None,
        }
    }

    fn append(
        self,
        _input: &I,
        _token_start: &<I as Stream>::Checkpoint,
        _kind: ErrorKind,
    ) -> Self {
        self
    }
}

impl<I: Stream> AddContext<I> for YolkParseError {
    fn add_context(
        mut self,
        _input: &I,
        _token_start: &<I as Stream>::Checkpoint,
        ctx: &'static str,
    ) -> Self {
        self.context = self.context.or(Some(ctx));
        self
    }
}

impl<I: Stream + Location> FromRecoverableError<I, Self> for YolkParseError {
    #[inline]
    fn from_recoverable_error(
        token_start: &<I as Stream>::Checkpoint,
        _err_start: &<I as Stream>::Checkpoint,
        input: &I,
        mut e: Self,
    ) -> Self {
        e.span = e.span.or_else(|| {
            Some((input.offset_from(token_start).saturating_sub(1)..input.location()).into())
        });
        e
    }
}

impl<I: Stream + Location> FromRecoverableError<I, ContextError> for YolkParseError {
    #[inline]
    fn from_recoverable_error(
        token_start: &<I as Stream>::Checkpoint,
        _err_start: &<I as Stream>::Checkpoint,
        input: &I,
        e: ContextError,
    ) -> Self {
        YolkParseError {
            span: Some((input.offset_from(token_start).saturating_sub(1)..input.location()).into()),
            label: None,
            help: None,
            context: e.context().next().and_then(|e| match e {
                StrContext::Label(l) => Some(*l),
                StrContext::Expected(StrContextValue::StringLiteral(s)) => Some(*s),
                StrContext::Expected(StrContextValue::Description(s)) => Some(*s),
                _ => None,
            }),
            kind: None,
        }
    }
}
