use anyhow::{bail, Result};
use std::str::Chars;

#[derive(Debug)]
pub enum Element<'a> {
    Raw(&'a str),
    IfBlock {
        pred: &'a str,
        if_tag: &'a str,
        body: Box<Element<'a>>,
        else_tag_and_body: Option<(&'a str, Box<Element<'a>>)>,
        end_tag: &'a str,
    },
    ReplaceInline {
        before_tag: &'a str,
        tag: &'a str,
        regex_pattern: &'a str,
        expr: &'a str,
    },
    ReplaceBlock {
        tag: &'a str,
        regex_pattern: &'a str,
        expr: &'a str,
        affected_line: &'a str,
    },
    ReplaceInInline {
        before_tag: &'a str,
        tag: &'a str,
        left: &'a str,
        right: &'a str,
        expr: &'a str,
    },
    ReplaceInBlock {
        tag: &'a str,
        left: &'a str,
        right: &'a str,
        expr: &'a str,
        affected_line: &'a str,
    },
    Directive {
        tag: &'a str,
        name: &'a str,
        content: &'a str,
    },
}

struct Parser<'a> {
    input: Chars<'a>,
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Result<Element<'a>> {
        todo!()
    }
    pub fn parse_element(&mut self) -> Result<Element<'a>> {
        todo!()
    }
    pub fn parse_raw(&mut self) -> Result<Element<'a>> {
        let mut current = self.input.clone();
    }
    pub fn parse_tag_start(&mut self) -> Result<()> {
        let mut current = self.input.clone();
        match (current.next(), current.next()) {
            (Some('{'), Some('%')) => {
                self.input = current;
                Ok(())
            }
            _ => bail!("Expected tag start"),
        }
    }
}
/*

# {% replace_in " " `${cringe}` %}
foo="lol"

foo = "lol" # {< replace_in " " `${cringe}` >}

*/
