use crate::eval_ctx::EvalCtx;
use crate::templating::COMMENT_START;

use super::element;
use super::Rule;

use super::YolkParser;

use anyhow::Result;
use pest::Parser;
use regex::Regex;

#[derive(Debug)]
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
    pub fn render(&self, eval_ctx: &mut EvalCtx) -> Result<String> {
        let mut output = String::new();
        let ctx = Context {
            comment_prefix: self.comment_prefix.clone(),
        };
        for element in &self.elements {
            output.push_str(&element.render(&ctx, eval_ctx)?);
        }
        Ok(output)
    }

    pub fn parse_string(s: &'a str) -> Result<Self> {
        let mut document = Document::default();
        let mut result = YolkParser::parse(Rule::YolkFile, s)?;
        let yolk_file = result.next().unwrap();

        for rule in yolk_file.into_inner() {
            let element = element::Element::try_from_pair(rule)?;
            match element {
                element::Element::Directive {
                    name: "CommentPrefix",
                    content,
                    ..
                } => {
                    document.comment_prefix = content.trim().to_string();
                }
                _ => {}
            }
            document.elements.push(element);
        }
        Ok(document)
    }
}

pub struct Context {
    pub(crate) comment_prefix: String,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            comment_prefix: "#".to_string(),
        }
    }
}

impl Context {
    #[allow(unused)]
    pub fn new(comment_prefix: String) -> Self {
        Self { comment_prefix }
    }
    pub fn enabled_str(&self, s: &str) -> String {
        let re = Regex::new(&format!("{}{}", self.comment_prefix, COMMENT_START)).unwrap();
        let lines: Vec<_> = s.split('\n').map(|line| re.replace_all(line, "")).collect();
        lines.join("\n")
    }
    pub fn disabled_str(&self, s: &str) -> String {
        let re = Regex::new(&format!("^\\s*{}{}", self.comment_prefix, COMMENT_START)).unwrap();
        let lines: Vec<_> = s
            .split('\n')
            .map(|line| {
                if !re.is_match(line) && !line.is_empty() {
                    let indent: String = line
                        .chars()
                        .take_while(|&c| c == ' ' || c == '\t')
                        .collect();
                    format!(
                        "{}{}{}{}",
                        indent,
                        self.comment_prefix,
                        COMMENT_START,
                        line.trim_start_matches(|c| c == ' ' || c == '\t')
                    )
                } else {
                    line.to_string()
                }
            })
            .collect();
        lines.join("\n")
    }
}
