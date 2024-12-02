use crate::templating::parser::linewise::ParsedLine;
use crate::{eval_ctx::EvalCtx, templating::parser::document_parser};

use super::{
    element,
    parser::{comment_style::CommentStyle, document_parser::DocumentParser, Rule, YolkParser},
};

use miette::{Diagnostic, LabeledSpan, Result, SourceCode};
use pest::Parser as _;

#[derive(Debug)]
pub struct Document<'a> {
    pub(crate) comment_style: CommentStyle,
    pub(crate) elements: Vec<element::Element<'a>>,
}

impl<'a> Default for Document<'a> {
    fn default() -> Self {
        Self {
            comment_style: CommentStyle::Prefix("#".to_string()),
            elements: Vec::new(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("{}", .1)]
    Pest(String, Box<pest::error::Error<Rule>>),
    #[error("{}", .1)]
    DocumentParser(String, document_parser::Error),
}

impl Diagnostic for ParseError {
    fn source_code(&self) -> Option<&dyn SourceCode> {
        match self {
            ParseError::Pest(text, _) => Some(text),
            ParseError::DocumentParser(text, _) => Some(text),
        }
    }
    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        match self {
            ParseError::Pest(_, e) => Some(Box::new(std::iter::once(LabeledSpan::at(
                match e.location {
                    pest::error::InputLocation::Pos(x) => x..x,
                    pest::error::InputLocation::Span((x, y)) => x..y,
                },
                format!("{}", e),
            )))),
            ParseError::DocumentParser(_, e) => Some(Box::new(std::iter::once(LabeledSpan::at(
                e.span(),
                format!("{}", e),
            )))),
        }
    }
}

// impl ParseError {
//     pub fn into_report(self) -> ariadne::Report<'static> {
//         match self {
//             ParseError::Pest(e) => {
//                 let span = match e.location {
//                     pest::error::InputLocation::Pos(x) => x..x,
//                     pest::error::InputLocation::Span((x, y)) => x..y,
//                 };

//                 let mut builder = ariadne::Report::build(ReportKind::Error, span).with_message(&e);
//                 if let Some(attempts) = e.parse_attempts() {
//                     for attempt in attempts.expected_tokens() {
//                         builder.add_note(attempt);
//                     }
//                 }
//                 builder.finish()
//             }
//             ParseError::DocumentParser(e) => ariadne::Report::build(ReportKind::Error, e.span())
//                 .with_message(&e)
//                 .finish(),
//         }
//     }
// }

impl<'a> Document<'a> {
    pub fn render(&self, eval_ctx: &mut EvalCtx) -> Result<String> {
        let mut output = String::new();
        let ctx = RenderContext {
            comment_style: self.comment_style.clone(),
        };
        for element in &self.elements {
            output.push_str(&element.render(&ctx, eval_ctx)?);
        }
        Ok(output)
    }

    pub fn parse_string(s: &'a str) -> Result<Self, ParseError> {
        let result_lines = YolkParser::parse(Rule::Document, s)
            .map_err(|e| ParseError::Pest(s.to_string(), Box::new(e)))?;
        let lines = result_lines
            .into_iter()
            .map(ParsedLine::from_pair)
            .collect();
        let parser = DocumentParser::new(s, lines);
        let elements = parser
            .parse()
            .map_err(|e| ParseError::DocumentParser(s.to_string(), e))?;
        Ok(Self {
            elements,
            ..Default::default()
        })
    }
}

pub struct RenderContext {
    pub(crate) comment_style: CommentStyle,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self {
            comment_style: CommentStyle::Prefix("#".to_string()),
        }
    }
}

impl RenderContext {
    #[allow(unused)]
    pub fn new(comment_style: CommentStyle) -> Self {
        Self { comment_style }
    }

    pub fn string_toggled(&self, s: &str, enable: bool) -> String {
        if enable {
            self.enabled_str(s)
        } else {
            self.disabled_str(s)
        }
    }

    pub fn enabled_str(&self, s: &str) -> String {
        let lines = s
            .split('\n')
            .map(|x| self.comment_style.enable_line(x))
            .collect::<Vec<_>>();
        lines.join("\n")
    }
    pub fn disabled_str(&self, s: &str) -> String {
        let lines = s
            .split('\n')
            .map(|x| self.comment_style.disable_line(x))
            .collect::<Vec<_>>();
        lines.join("\n")
    }
}
