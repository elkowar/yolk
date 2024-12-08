use crate::eval_ctx::EvalCtx;

use super::{comment_style::CommentStyle, element, parser};

use miette::Result;

#[derive(Debug)]
pub struct Document<'a> {
    comment_style: CommentStyle,
    elements: Vec<element::Element<'a>>,
    source: &'a str,
    source_name: Option<String>,
}

impl<'a> Default for Document<'a> {
    fn default() -> Self {
        Self {
            comment_style: CommentStyle::Prefix("#".to_string()),
            elements: Vec::new(),
            source_name: None,
            source: "",
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
        let comment_style = CommentStyle::try_infer_from_elements(&elements).unwrap_or_default();
        Ok(Self {
            elements,
            comment_style,
            source: s,
            source_name: None,
        })
    }
    pub fn parse_string_named(name: &str, s: &'a str) -> Result<Self> {
        let elements = parser::parse_document(s)?;
        let comment_style = CommentStyle::try_infer_from_elements(&elements).unwrap_or_default();
        Ok(Self {
            elements,
            comment_style,
            source: s,
            source_name: Some(name.to_string()),
        })
    }

    #[allow(unused)]
    pub fn elements(&self) -> &[element::Element<'a>] {
        &self.elements
    }
    #[allow(unused)]
    pub fn comment_style(&self) -> &CommentStyle {
        &self.comment_style
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
