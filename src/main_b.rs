use std::borrow::Cow;

pub mod main_a;

use anyhow::{Context, Result};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use regex::Regex;
#[derive(Parser)]
#[grammar = "yolk.pest"]
pub struct YolkParser;

#[derive(Debug)]
pub enum ParsedLine<'a> {
    Raw(&'a str),
    If(&'a str, PTag<'a>),
}

#[derive(Debug)]
pub enum PTag<'a> {
    If { pred: &'a str },
    Else,
    End,
    Directive { name: &'a str, content: &'a str },
}

pub struct State<'a, Lines: Iterator<Item = &'a str>> {
    parsed: Vec<ParsedLine<'a>>,
    input: Lines,
    comment_prefix: String,
}

pub fn main() -> Result<()> {
    let test_input = r#"
        // {% CommentPrefix // %}
        hallo
        // {% if true %}
        test
        // {% else %}
        // bruh
        // {% end %}
    "#;

    let parsed = parse_lines(test_input.lines())?;

    println!(
        "{}",
        parsed
            .into_iter()
            .map(|x| format!("{:?}", x))
            .collect::<Vec<_>>()
            .join("\n")
    );
    Ok(())
}

pub fn parse_lines<'a>(mut input: impl Iterator<Item = &'a str>) -> Result<Vec<ParsedLine<'a>>> {
    let mut parsed = Vec::new();
    while let Some(line) = input.next() {
        if let Ok(raw_line) = YolkParser::parse(Rule::RawLine, line) {
            parsed.push(ParsedLine::Raw(raw_line.as_str()))
        } else {
            let mut tag_line = YolkParser::parse(Rule::TagLine, line)?;
            println!("{:#?}", tag_line);
            let tag = tag_line.next().unwrap();
            let tag = tag.into_inner().next().unwrap();
            let tag = match tag.as_rule() {
                Rule::IfTag => {
                    let pred = tag_line.find_first_tagged("pred").unwrap();
                    PTag::If {
                        pred: pred.as_str(),
                    }
                }
                Rule::ElseTag => PTag::Else,
                Rule::EndTag => PTag::End,
                Rule::DirectiveTag => {
                    let name = tag_line.find_first_tagged("name").unwrap();
                    let content = tag_line.find_first_tagged("content").unwrap();
                    PTag::Directive {
                        name: name.as_str(),
                        content: content.as_str(),
                    }
                }
                Rule::EOI => continue,
                other => unreachable!("{other:#?} in parse_lines"),
            };
            parsed.push(ParsedLine::If(tag_line.as_str(), tag))
        }
    }
    Ok(parsed)
}

/// Remove a comment prefix from a line, if one exists
fn remove_comment<'a>(comment_prefix: &str, line: &'a str) -> Cow<'a, str> {
    let regex = Regex::new(&format!(r"^\s*{}\s*", comment_prefix)).unwrap();
    regex.replace(line, "")
}
