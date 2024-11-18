use anyhow::Result;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use regex::Regex;

const COMMENT_START: &str = " ==x== ";

#[derive(Parser)]
#[grammar = "file.pest"]
pub struct YolkParser;

pub struct Document<'a> {
    comment_prefix: String,
    elements: Vec<Pair<'a, Rule>>,
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
    pub fn render_element(&self, pair: &Pair<'a, Rule>) -> Result<String> {
        let mut output = String::new();
        match pair.as_rule() {
            Rule::IfBlock => {
                let inner = pair.clone().into_inner();
                let pred = inner.find_first_tagged("pred").unwrap();
                let pred_true = pred.as_str().trim() == "true";
                let if_tag = inner.find_first_tagged("if").unwrap();
                let body = inner.find_first_tagged("body").unwrap();
                let end_tag = inner.find_first_tagged("end").unwrap();
                let else_tag_and_body = inner
                    .find_first_tagged("else")
                    .zip(inner.find_first_tagged("elsebody"));
                output.push_str(if_tag.as_str());
                let rendered_body = self.render_element(&body)?;
                if pred_true {
                    output.push_str(&self.enabled_str(&rendered_body));
                } else {
                    output.push_str(&self.disabled_str(&rendered_body));
                }
                if let Some((else_tag, else_body)) = else_tag_and_body {
                    output.push_str(else_tag.as_str());
                    let rendered_else_body = self.render_element(&else_body)?;
                    if pred_true {
                        output.push_str(&self.disabled_str(&rendered_else_body));
                    } else {
                        output.push_str(&self.enabled_str(&rendered_else_body));
                    }
                }
                output.push_str(end_tag.as_str());
            }
            _ => {
                output.push_str(pair.as_str());
            }
        }
        Ok(output)
    }
    pub fn render(&self) -> Result<String> {
        let mut output = String::new();
        for part in &self.elements {
            output.push_str(&self.render_element(part)?);
        }
        Ok(output)
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

    pub fn parse_string(s: &'a str) -> Result<Self> {
        let mut document = Document::default();
        let mut result = YolkParser::parse(Rule::YolkFile, s)?;
        let yolk_file = result.next().unwrap();

        for rule in yolk_file.into_inner() {
            match rule.as_rule() {
                Rule::DirectiveTag => {
                    let inner = rule.clone().into_inner();
                    let name = inner.find_first_tagged("name").unwrap();
                    let value = inner.find_first_tagged("value").unwrap();
                    match name.as_str() {
                        "CommentPrefix" => {
                            document.comment_prefix = value.as_str().trim().to_string();
                        }
                        other => {
                            println!("Unknown directive: {other}");
                        }
                    }
                }
                _ => {}
            }
            document.elements.push(rule);
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
    let mut document = Document::parse_string(example)?;
    let result = document.render()?;
    println!("{}", result);
    println!("{}", Document::parse_string(&result)?.render()?);
    Ok(())
}
