use miette::{Diagnostic, SourceSpan};
use rhai::Engine;

use super::rhai_function_hints::{hint_for_function_not_found, FunctionCallHint};

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum RhaiScriptError {
    #[error("{kind}")]
    #[diagnostic(forward(kind))]
    Located {
        #[label("here")]
        span: SourceSpan,
        kind: RhaiScriptErrorKind,
    },
    #[error(transparent)]
    #[diagnostic(transparent)]
    Unlocated(#[from] RhaiScriptErrorKind),
}

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum RhaiScriptErrorKind {
    #[error(transparent)]
    Rhai(#[from] Box<rhai::EvalAltResult>),
    /// A function was not found, and we managed to find some additional information about available alternatives.
    #[error("{message}")]
    EnrichedFunctionNotFound {
        message: String,
        #[help]
        help: Option<String>,
        #[source]
        origin: Box<rhai::EvalAltResult>,
        hint: Box<FunctionCallHint>,
    },
    #[error("{}", .0)]
    #[diagnostic(transparent)]
    Other(miette::Report),
}

impl From<rhai::EvalAltResult> for RhaiScriptErrorKind {
    fn from(err: rhai::EvalAltResult) -> Self {
        Self::Rhai(Box::new(err))
    }
}

impl From<rhai::EvalAltResult> for RhaiScriptError {
    fn from(err: rhai::EvalAltResult) -> Self {
        Self::Unlocated(err.into())
    }
}

impl RhaiScriptError {
    pub fn new_other<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::from_report(miette::Report::from_err(err))
    }

    pub fn other<E>(err: E) -> Self
    where
        E: Diagnostic + Send + Sync + 'static,
    {
        Self::from_report(miette::Report::from(err))
    }

    pub fn from_report(report: miette::Report) -> Self {
        Self::Unlocated(RhaiScriptErrorKind::Other(report))
    }

    pub fn msg(message: impl std::fmt::Display) -> Self {
        Self::from_report(miette::Report::msg(message.to_string()))
    }

    pub fn from_rhai_compile(source_code: &str, err: rhai::ParseError) -> Self {
        Self::from_rhai(source_code, err.into())
    }

    pub fn from_rhai(source_code: &str, err: rhai::EvalAltResult) -> Self {
        Self::from_rhai_inner(source_code, err, None)
    }

    pub fn from_rhai_with_engine(
        source_code: &str,
        err: rhai::EvalAltResult,
        engine: &Engine,
    ) -> Self {
        Self::from_rhai_inner(source_code, err, Some(engine))
    }

    fn from_rhai_inner(
        source_code: &str,
        err: rhai::EvalAltResult,
        engine: Option<&Engine>,
    ) -> Self {
        let position = err.position();
        let span = if source_code.is_empty() {
            (0..0).into()
        } else if let Some(line_nr) = position.line() {
            // TODO: this won't work with \r\n, _or will it_? *vsauce music starts playing*
            let offset_start = source_code
                .split_inclusive('\n')
                .take(line_nr - 1)
                .map(|x| x.len())
                .sum::<usize>();
            let span = if let Some(within_line) = position.position() {
                offset_start + within_line..offset_start + within_line + 1
            } else {
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
                offset_start + indent..offset_end
            }
            .into();
            clamp_span(span, source_code.len())
        } else {
            (0..0).into()
        };
        let kind = match (engine, err) {
            (Some(engine), rhai::EvalAltResult::ErrorFunctionNotFound(signature, position)) => {
                let hint = hint_for_function_not_found(engine, &signature);
                RhaiScriptErrorKind::EnrichedFunctionNotFound {
                    message: hint.message(),
                    help: hint.help(),
                    origin: Box::new(rhai::EvalAltResult::ErrorFunctionNotFound(
                        signature, position,
                    )),
                    hint: Box::new(hint),
                }
            }
            (_, err) => err.into(),
        };
        Self::Located { span, kind }
    }

    /// Convert this error into a [`miette::Report`] with the given name and source code attached.
    pub fn into_report(self, name: impl ToString, source: impl ToString) -> miette::Report {
        miette::Report::from(self).with_source_code(
            miette::NamedSource::new(name.to_string(), source.to_string()).with_language("rhai"),
        )
    }

    pub fn span(&self) -> Option<SourceSpan> {
        match self {
            Self::Located { span, .. } => Some(*span),
            Self::Unlocated(_) => None,
        }
    }
}

fn clamp_span(span: SourceSpan, source_len: usize) -> SourceSpan {
    let start = span.offset().min(source_len.saturating_sub(1));
    let requested_len = span.len();
    let len = requested_len.min(source_len.saturating_sub(start));
    (start, len).into()
}

#[cfg(test)]
mod test {
    use crate::script::eval_ctx::EvalCtx;
    use crate::util::test_util::render_report;
    use crate::yolk::EvalMode;

    /// Evaluate a rhai snippet that is expected to fail, and render the
    /// resulting error report exactly as a user would see it on the CLI.
    fn render_rhai_error(source: &str) -> String {
        let mut ctx = EvalCtx::new_in_mode(EvalMode::Local).unwrap();
        let err = ctx
            .eval_rhai::<()>(source)
            .expect_err("expected rhai evaluation to fail");
        render_report(err.into_report("<inline>", source))
    }

    #[test]
    fn test_enriched_unknown_function_renders_once() {
        insta::assert_snapshot!(render_rhai_error(r#"ptint("hi")"#));
    }

    #[test]
    fn test_enriched_wrong_arguments_renders_once() {
        insta::assert_snapshot!(render_rhai_error("io::path_exists(1)"));
    }

    #[test]
    fn test_generic_runtime_error() {
        insta::assert_snapshot!(render_rhai_error("1 / 0"));
    }

    #[test]
    fn test_nested_function_call_error() {
        insta::assert_snapshot!(render_rhai_error("fn boom() { 1 / 0 } boom()"));
    }

    #[test]
    fn test_variable_not_found() {
        insta::assert_snapshot!(render_rhai_error("foo + 1"));
    }

    #[test]
    fn test_syntax_error() {
        insta::assert_snapshot!(render_rhai_error("let x = "));
    }
}
