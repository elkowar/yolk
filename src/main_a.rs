use anyhow::Result;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "file.pest"]
pub struct YolkParser;

pub struct Document<'a> {
    comment_prefix: String,
    parts: Vec<Pair<'a, Rule>>,
}

impl<'a> Default for Document<'a> {
    fn default() -> Self {
        Self {
            comment_prefix: "#".to_string(),
            parts: Vec::new(),
        }
    }
}

impl<'a> Document<'a> {
    pub fn render(&self) -> String {
        let mut output = String::new();
        for part in &self.parts {
            match part.as_rule() {
                Rule::TagBlock => {
                    let mut inner = part.clone().into_inner();
                    let block = inner.next().unwrap();
                    match block.as_rule() {
                        Rule::TagIfBlock => {
                            let mut inner = block.clone().into_inner();
                            let condition = inner.next().unwrap();
                            let body = inner.next().unwrap();
                            if condition.as_str() == "true" {
                                output.push_str(&body.as_str());
                            }
                        }
                        other => unreachable!("{other:?} in TagBlock"),
                    }
                }
                _ => {
                    output.push_str(part.as_str());
                }
            }
        }
        output
    }
}

fn main() -> Result<()> {
    let mut result = YolkParser::parse(
        Rule::YolkFile,
        r#"
        // {% CommentPrefix // %}
        hallo
        {% if true %}
        test
        {% else %}
        bruh
        {% end %}
    "#,
    )?;

    let mut document = Document::default();

    let yolk_file = result.next().unwrap();

    for rule in yolk_file.into_inner() {
        match rule.as_rule() {
            Rule::Directive => {
                let mut inner = rule.clone().into_inner();
                let directive = inner.next().unwrap();
                match directive.as_str() {
                    "CommentPrefix" => {
                        let value = inner.next().unwrap();
                        document.comment_prefix = value.as_str().to_string();
                    }
                    other => {
                        println!("Unknown directive: {other}");
                    }
                }
            }
            _ => {
                println!("Unhandled rule: {rule:?}");
            }
        }

        document.parts.push(rule);
    }
    println!("{}", document.render());
    Ok(())
}
