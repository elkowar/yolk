use crate::eval_ctx::EvalCtx;
use crate::templating::parser::linewise::ParsedLine;

use super::{
    element,
    parser::{comment_style::CommentStyle, document_parser::DocumentParser},
    Rule, YolkParser,
};

use anyhow::Result;
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
        let result_lines = YolkParser::parse(Rule::Document, s)?;
        let lines = result_lines
            .into_iter()
            .map(ParsedLine::try_from_pair)
            .collect::<Result<_>>()?;
        let parser = DocumentParser::new(lines);
        let elements = parser.parse()?;
        // TODO: properly detect comment prefix automatically,
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
