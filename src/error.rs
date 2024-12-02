use ariadne::ReportKind;

use crate::templating::parser::{document_parser, Rule};

pub type YolkResult<T> = std::result::Result<T, YolkError>;

#[derive(thiserror::Error, Debug)]
pub enum YolkError {
    #[error(transparent)]
    Pest(pest::error::Error<Rule>),
    #[error(transparent)]
    DocumentParser(document_parser::Error),
}

impl YolkError {
    pub fn into_report(self) -> ariadne::Report<'static> {
        match self {
            Self::Pest(e) => {
                let span = match e.location {
                    pest::error::InputLocation::Pos(x) => x..x,
                    pest::error::InputLocation::Span((x, y)) => x..y,
                };

                let mut builder = ariadne::Report::build(ReportKind::Error, span).with_message(&e);
                if let Some(attempts) = e.parse_attempts() {
                    for attempt in attempts.expected_tokens() {
                        builder.add_note(attempt);
                    }
                }
                builder.finish()
            }
            Self::DocumentParser(e) => e.into_report(),
        }
    }
}
