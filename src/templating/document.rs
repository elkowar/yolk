use crate::eval_ctx::EvalCtx;
use crate::templating::parser::linewise::ParsedLine;
use crate::templating::COMMENT_START;

use super::{element, parser::document_parser::DocumentParser, Rule, YolkParser};

use anyhow::Result;
use pest::Parser as _;
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
        let result_lines = YolkParser::parse(Rule::Document, s)?;
        let lines = result_lines
            .into_iter()
            .map(|pair| ParsedLine::try_from_pair(pair))
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

    pub fn string_toggled(&self, s: &str, enable: bool) -> String {
        if enable {
            self.enabled_str(s)
        } else {
            self.disabled_str(s)
        }
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
