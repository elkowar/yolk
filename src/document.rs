use crate::ctx;
use crate::element;

use super::Rule;

use super::YolkParser;

use anyhow::Result;
use pest::Parser;

pub struct Document<'a> {
    pub(crate) comment_prefix: String,
    pub(crate) elements: Vec<element::Element<'a>>,
}

impl<'a> Default for Document<'a> {
    fn default() -> Self {
        Self {
            comment_prefix: "#".to_string(),
            elements: Vec::new(),
        }
    }
}

impl<'a> Document<'a> {
    pub fn render(&self) -> Result<String> {
        let mut output = String::new();
        let ctx = ctx::Context {
            comment_prefix: self.comment_prefix.clone(),
        };
        for element in &self.elements {
            output.push_str(&element.render(&ctx)?);
        }
        Ok(output)
    }

    pub fn parse_string(s: &'a str) -> Result<Self> {
        let mut document = Document::default();
        let mut result = YolkParser::parse(Rule::YolkFile, s)?;
        let yolk_file = result.next().unwrap();

        for rule in yolk_file.into_inner() {
            document
                .elements
                .push(element::Element::try_from_pair(rule)?);
        }
        Ok(document)
    }
}
