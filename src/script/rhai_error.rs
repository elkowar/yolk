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
    #[error("No function found matching signature: {}", .signature)]
    #[diagnostic(help("{}:\n{}", if *perfect_match {"Candidates"} else {"Did you mean"}, candidates.join("\n")))]
    RhaiFunctionNotFound {
        signature: String,
        perfect_match: bool,
        candidates: Vec<String>,
    },
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

    pub fn from_rhai_compile(
        engine: &rhai::Engine,
        source_code: &str,
        err: rhai::ParseError,
    ) -> Self {
        Self::from_rhai(engine, source_code, err.into())
    }

    pub fn from_rhai(engine: &rhai::Engine, source_code: &str, err: rhai::EvalAltResult) -> Self {
        let span = span_from_rhai_eval_result(source_code, &err);
        let rhai_error = match err {
            rhai::EvalAltResult::ErrorFunctionNotFound(signature, _position) => {
                Self::from_rhai_function_not_found(engine, signature)
            }
            other => Self::RhaiError(other),
        };

        Self::SourceError {
            span,
            origin: Box::new(rhai_error),
        }
    }

    /// Convert this error into a [`miette::Report`] with the given name and source code attached as a rust source.
    pub fn into_report(self, name: impl ToString, source: impl ToString) -> miette::Report {
        miette::Report::from(self).with_source_code(
            miette::NamedSource::new(name.to_string(), source.to_string()).with_language("Rust"),
        )
    }

    /// Generate a [`RhaiError::RhaiFunctionNotFound`] for a [`rhai::EvalAltResult::ErrorFunctionNotFound`],
    /// adding hints on which similar functions do exist.
    ///
    /// You most definitely want to use [`RhaiError::from_rhai_with_source`] instead, as it also adds span information.
    /// This function is only used internally.
    fn from_rhai_function_not_found(engine: &rhai::Engine, signature: String) -> RhaiError {
        let used_fn_name = signature.split(['(', ' ']).next().unwrap_or("");
        let used_fn_name = used_fn_name.split("::").last().unwrap_or(used_fn_name);
        let mut candidates = engine.collect_fn_metadata(
            None,
            |info| {
                let name = info.metadata.name.as_str();
                let distance = strsim::levenshtein(used_fn_name, name);
                (distance < 3).then(|| {
                    let candidate = info
                        .metadata
                        .gen_signature(|x| engine.map_type_name(x).into());
                    let module_prefix = if info.namespace.is_empty() {
                        "".to_string()
                    } else {
                        format!("{}::", info.namespace)
                    };
                    let candidate = format!("{}{}", module_prefix, candidate);
                    (candidate, distance)
                })
            },
            true,
        );
        candidates.sort_by_key(|(_, distance)| *distance);
        let had_perfect_match = candidates.iter().any(|(_, d)| *d == 0);
        if had_perfect_match {
            candidates = candidates
                .into_iter()
                .take_while(|(_, distance)| *distance == 0)
                .collect();
        }

        RhaiError::RhaiFunctionNotFound {
            signature,
            perfect_match: had_perfect_match,
            candidates: candidates.into_iter().map(|(x, _)| x).collect(),
        }
    }
}

fn span_from_rhai_eval_result(source_code: &str, err: &rhai::EvalAltResult) -> Range<usize> {
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
    span
}
