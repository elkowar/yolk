use std::sync::Arc;

use miette::{Diagnostic, NamedSource, Severity, SourceSpan};
use winnow::{
    error::{AddContext, ErrorKind, FromRecoverableError, ParserError},
    stream::{Location, Stream},
};

use crate::script::rhai_error::RhaiError;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum TemplateError {
    #[error("Error evaluating rhai")]
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

    /// Convert this error into a [`miette::Report`] with the given name and source code attached.
    pub fn into_report(self, name: impl ToString, source: impl ToString) -> miette::Report {
        miette::Report::from(self)
            .with_source_code(NamedSource::new(name.to_string(), source.to_string()))
    }
}

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
                    message: e.message,
                    input: src.clone(),
                    span: e.span.unwrap_or_else(|| (0usize..0usize).into()),
                    label: e.label,
                    help: e.help,
                    severity: Severity::Error,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Diagnostic, Clone, Eq, PartialEq, thiserror::Error)]
#[error("{}", message.unwrap_or_else(|| "An unspecified parse error occurred."))]
pub struct YolkParseDiagnostic {
    #[source_code]
    pub input: Arc<NamedSource<String>>,

    /// Offset in chars of the error.
    #[label("{}", label.unwrap_or_else(|| "here"))]
    pub span: SourceSpan,

    /// Message
    pub message: Option<&'static str>,

    /// Label text for this span. Defaults to `"here"`.
    pub label: Option<&'static str>,

    /// Suggestion for fixing the parser error.
    #[help]
    pub help: Option<&'static str>,

    /// Severity level for the Diagnostic.
    #[diagnostic(severity)]
    pub severity: miette::Severity,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct YolkParseError {
    pub message: Option<&'static str>,
    pub span: Option<SourceSpan>,
    pub label: Option<&'static str>,
    pub help: Option<&'static str>,
}

impl<I: Stream> ParserError<I> for YolkParseError {
    fn from_error_kind(_input: &I, _kind: ErrorKind) -> Self {
        Self {
            span: None,
            label: None,
            help: None,
            message: None,
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

impl<I: Stream> AddContext<I, YolkParseContext> for YolkParseError {
    fn add_context(
        mut self,
        _input: &I,
        _token_start: &<I as Stream>::Checkpoint,
        ctx: YolkParseContext,
    ) -> Self {
        self.message = ctx.message.or(self.message);
        self.label = ctx.label.or(self.label);
        self.help = ctx.help.or(self.help);
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
        e.span = e
            .span
            .or_else(|| Some(span_from_checkpoint(input, token_start)));
        e
    }
}

impl<I: Stream + Location> FromRecoverableError<I, YolkParseContext> for YolkParseError {
    #[inline]
    fn from_recoverable_error(
        token_start: &<I as Stream>::Checkpoint,
        _err_start: &<I as Stream>::Checkpoint,
        input: &I,
        e: YolkParseContext,
    ) -> Self {
        YolkParseError {
            span: Some((input.offset_from(token_start).saturating_sub(1)..input.location()).into()),
            label: e.label,
            help: e.help,
            message: e.message,
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub(super) struct YolkParseContext {
    message: Option<&'static str>,
    label: Option<&'static str>,
    help: Option<&'static str>,
}

impl YolkParseContext {
    pub(super) fn msg(mut self, txt: &'static str) -> Self {
        self.message = Some(txt);
        self
    }

    pub(super) fn lbl(mut self, txt: &'static str) -> Self {
        self.label = Some(txt);
        self
    }

    pub(super) fn hlp(mut self, txt: &'static str) -> Self {
        self.help = Some(txt);
        self
    }
}

pub(super) fn cx() -> YolkParseContext {
    Default::default()
}

fn span_from_checkpoint<I: Stream + Location>(
    input: &I,
    start: &<I as Stream>::Checkpoint,
) -> SourceSpan {
    let offset = input.offset_from(start);
    ((input.location() - offset)..input.location()).into()
}
