use crate::eval_ctx::EvalCtx;
use crate::templating::parser::linewise::ParsedLine;

use super::{
    element,
    parser::{comment_style::CommentStyle, document_parser::DocumentParser, Rule, YolkParser},
};

use miette::{LabeledSpan, Result};
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

    pub fn parse_string(s: &'a str) -> Result<Self> {
        let mut result_doc = YolkParser::parse(Rule::Document, s).map_err(pest_error_to_miette)?;
        let result_lines = result_doc
            .next()
            .ok_or_else(|| miette::miette!("no document in document"))?;
        let lines = result_lines
            .into_inner()
            .into_iter()
            .map(ParsedLine::from_pair)
            .collect();
        let parser = DocumentParser::new(s, lines);
        let elements = parser.parse()?;
        // .map_err(|e| {
        // let labels = match e.labels().and_then(|mut x| x.next()) {
        //     Some(label) => vec![label],
        //     None => vec![],
        // };
        // miette::miette!(labels = labels, "{e}")
        // })
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
        s.split('\n')
            .map(|x| self.comment_style.enable_line(x))
            .collect::<Vec<_>>()
            .join("\n")
    }
    pub fn disabled_str(&self, s: &str) -> String {
        s.split('\n')
            .map(|x| self.comment_style.disable_line(x))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn pest_error_to_miette(e: pest::error::Error<Rule>) -> miette::Report {
    let span = LabeledSpan::at(
        match e.location {
            pest::error::InputLocation::Pos(x) => x..x,
            pest::error::InputLocation::Span((x, y)) => x..y,
        },
        "here",
    );
    miette::miette!(labels = vec![span], "{e}")
}
