use std::ops::Range;

use miette::Diagnostic;

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum RhaiError {
    #[error("{origin}")]
    #[diagnostic(forward(origin))]
    SourceError {
        #[label("here")]
        span: Range<usize>,
        origin: Box<RhaiError>,
    },
    #[error(transparent)]
    RhaiError(#[from] rhai::EvalAltResult),
    #[error("{}", .0)]
    #[diagnostic(transparent)]
    Other(miette::Report),
}

impl RhaiError {
    pub fn new_other<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Other(miette::miette!(err))
    }
    pub fn other<E>(err: E) -> Self
    where
        E: Diagnostic + Send + Sync + 'static,
    {
        Self::Other(miette::Report::from(err))
    }

    pub fn from_rhai_compile(source_code: &str, err: rhai::ParseError) -> Self {
        Self::from_rhai(source_code, err.into())
    }

    pub fn from_rhai(source_code: &str, err: rhai::EvalAltResult) -> Self {
        let position = err.position();
        let mut span = 0..0;
        if let Some(line_nr) = position.line() {
            // TODO: this won't work with \r\n, _or will it_? *vsauce music starts playing*
            let offset_start = source_code
                .split_inclusive('\n')
                .take(line_nr - 1)
                .map(|x| x.len())
                .sum::<usize>();
            span = if let Some(within_line) = position.position() {
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
            };
        }
        if span.start >= source_code.len() {
            span = source_code.len() - 1..source_code.len();
        }
        Self::SourceError {
            span,
            origin: Box::new(RhaiError::RhaiError(err)),
        }
    }

    /// Convert this error into a [`miette::Report`] with the given name and source code attached as a rust source.
    pub fn into_report(self, name: impl ToString, source: impl ToString) -> miette::Report {
        miette::Report::from(self).with_source_code(
            miette::NamedSource::new(name.to_string(), source.to_string()).with_language("Rust"),
        )
    }
}
