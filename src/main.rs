use anyhow::{Context, Result};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use regex::Regex;

const COMMENT_START: &str = " ==x== ";

#[derive(Parser)]
#[grammar = "yolk.pest"]
pub struct YolkParser;

pub struct Document<'a> {
    comment_prefix: String,
    elements: Vec<element::Element<'a>>,
}

mod element;

pub struct RenderingContext {
    comment_prefix: String,
}

impl RenderingContext {
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
        let render_ctx = RenderingContext {
            comment_prefix: self.comment_prefix.clone(),
        };
        for element in &self.elements {
            output.push_str(&element.render(&render_ctx)?);
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

pub(crate) fn main() -> Result<()> {
    let example = r#"
        // {% CommentPrefix // %}
        hallo
        // {% if true %}
        test
        // {% else %}
        bruh
        // {% end %}
    "#;
    let document = Document::parse_string(example)?;

    let result = document.render()?;
    println!("{}", result);
    println!("{}", Document::parse_string(&result)?.render()?);
    Ok(())
}
