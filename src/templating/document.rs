use crate::script::eval_ctx::EvalCtx;

use super::{
    comment_style::CommentStyle,
    element::{self, render_elements},
    parser,
};

use miette::{NamedSource, Result};

#[derive(Debug, arbitrary::Arbitrary)]
pub struct Document<'a> {
    comment_style: CommentStyle,
    elements: Vec<element::Element<'a>>,
    source: &'a str,
    source_name: String,
}

impl<'a> Document<'a> {
    pub fn render(&self, eval_ctx: &mut EvalCtx) -> Result<String> {
        let output = render_elements(&self.comment_style, eval_ctx, &self.elements)
            .map_err(|e| e.into_report(&self.source_name, self.source))?;
        Ok(output)
    }

    #[cfg(test)]
    pub fn parse_string(s: &'a str) -> Result<Self> {
        Self::parse_string_named("unnamed", s)
    }

    pub fn parse_string_named(name: &str, s: &'a str) -> Result<Self> {
        let elements = parser::parse_document(s).map_err(|e| {
            miette::Report::from(e).with_source_code(NamedSource::new(name, s.to_string()))
        })?;
        let comment_style = CommentStyle::try_infer_from_elements(&elements).unwrap_or_default();
        Ok(Self {
            elements,
            comment_style,
            source: s,
            source_name: name.to_string(),
        })
    }
}
