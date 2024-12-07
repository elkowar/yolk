use crate::eval_ctx::EvalCtx;

use super::{
    element,
    parser::{self, comment_style::CommentStyle},
};

use miette::{LabeledSpan, Result};

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
        let elements = parser::parse_document(s)?;
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

// fn pest_error_to_miette(e: pest::error::Error<Rule>) -> miette::Report {
//     let span = LabeledSpan::at(
//         match e.location {
//             pest::error::InputLocation::Pos(x) => x..x,
//             pest::error::InputLocation::Span((x, y)) => x..y,
//         },
//         "here",
//     );
//     miette::miette!(labels = vec![span], "{e}")
// }
