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

pub enum ParsedLine {
    Raw(&'a str),
    Tag(&'a str),
}

pub struct State<'a, Lines: Iterator<Item = &'a str>> {
    parsed: Vec<ParsedLine>,
    input: Lines,
    comment_prefix: String,
}

pub enum Section<'a> {
    Raw(&'a str),
    If(IfBlock<'a>),
}

pub struct IfBlock<'a> {
    tag_line: &'a str,
    body: Box<Vec<Section<'a>>>,
    end_line: &'a str,
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

    let mut state = State {
        output: Vec::new(),
        input: test_input.lines(),
        comment_prefix: "#".to_string(),
    };

    state.process()?;
    println!("{}", state.output.join("\n"));
    Ok(())
}
impl<'a, Lines: Iterator<Item = &'a str>> State<'a, Lines> {
    pub fn process(&mut self) -> Result<()> {
        while let Some(line) = self.input.next() {
            self.process_line(line)?;
        }
        Ok(())
    }

    fn process_line(&mut self, line: &'a str) -> Result<()> {
        if let Ok(raw_line) = YolkParser::parse(Rule::RawLine, line) {
            self.output.push(Cow::from(raw_line.as_str()));
        } else {
            let tag_line = YolkParser::parse(Rule::TagLine, line)?;
            println!("{tag_line:#?}");
            self.tag(tag_line)?;
        }
        Ok(())
    }
    fn tag(&mut self, mut tag_line: Pairs<'a, Rule>) -> Result<()> {
        self.output.push(Cow::from(tag_line.as_str()));
        let tag = tag_line.next().unwrap();
        match tag.as_rule() {
            Rule::IfTag => self.if_tag(tag)?,
            Rule::DirectiveTag => self.directive(tag)?,
            _ => unreachable!(),
        }
        Ok(())
    }

    fn if_tag(&mut self, tag: Pair<'a, Rule>) -> Result<()> {
        let mut inner = tag.into_inner();
        let condition = inner
            .find_first_tagged("pred")
            .context("Missing predicate")?;
        if condition.as_str() == "true" {
            while let Some(line) = self.input.next() {
                let line: String = remove_comment(&self.comment_prefix, &line).into_owned();
                if let Ok(_) = YolkParser::parse(Rule::RawLine, line.as_ref()) {
                    self.output.push(Cow::Owned(line));
                } else {
                    // let mut tag_line = YolkParser::parse(Rule::TagLine, &line)?;
                    // println!("{tag_line:#?}");
                    // self.output.push(Cow::from(tag_line.as_str()));
                    // let tag = tag_line.next().unwrap();
                    // match tag.as_rule() {
                    //     Rule::EndTag => {
                    //         break;
                    //     }
                    //     _ => self.tag(tag_line)?,
                    // }
                }
            }
        }

        Ok(())
    }

    fn directive(&mut self, pair: Pair<'a, Rule>) -> Result<()> {
        let mut inner = pair.into_inner();
        let next = inner.next().unwrap();
        match next.as_rule() {
            Rule::DirectiveName => {
                if next.as_str() == "CommentPrefix" {
                    self.comment_prefix = inner.next().unwrap().as_str().to_string();
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

/// Remove a comment prefix from a line, if one exists
fn remove_comment<'a>(comment_prefix: &str, line: &'a str) -> Cow<'a, str> {
    let regex = Regex::new(&format!(r"^\s*{}\s*", comment_prefix)).unwrap();
    regex.replace(line, "")
}
