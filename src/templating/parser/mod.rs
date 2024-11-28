use anyhow::{bail, Result};
use std::str::Chars;

#[derive(Debug)]
pub enum Element<'a> {
    /// A block of raw text without any tags
    Raw(&'a str),
    InlineBlock {
        left_of_tag: &'a str,
        tag: &'a str,
        right_of_tag: &'a str,
        expr: &'a str,
    },
    NextLineBlock {
        left_of_tag: &'a str,
        tag: &'a str,
        right_of_tag: &'a str,
        expr: &'a str,
        next_line: &'a str,
    },
    MultiLineBlock {
        left_of_tag: &'a str,
        tag: &'a str,
        right_of_tag: &'a str,
        expr: &'a str,
        body: Vec<Element<'a>>,
    },
}

#[derive(Clone, Debug)]
struct Input<'a> {
    text: &'a str,
    chars: Chars<'a>,
    offset: usize,
}

impl<'a> Input<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            chars: text.chars(),
            offset: 0,
        }
    }
    pub fn next(&mut self) -> Option<char> {
        self.chars.next().map(|c| {
            self.offset += c.len_utf8();
            c
        })
    }

    pub fn start_str(&self) -> Self {
        Self {
            text: &self.text[self.offset..],
            chars: self.text[self.offset..].chars(),
            offset: 0,
        }
    }

    pub fn current_to_str(&self) -> &'a str {
        &self.text[..self.offset]
    }

    pub fn read_n(&mut self, n: usize) -> Option<&'a str> {
        let start = self.offset;
        for _ in 0..n {
            self.next()?;
        }
        Some(&self.text[start..self.offset])
    }
}

struct Parser<'a> {
    input: Input<'a>,
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Result<Element<'a>> {
        todo!()
    }
    pub fn parse_element(&mut self) -> Result<Element<'a>> {
        todo!()
    }

    pub fn parse_inline_tag(&mut self) -> Result<Element<'a>> {
        let _ = self.parse_literal("{<")?;
        self.parse_until(">}")
    }

    pub fn parse_literal(&mut self, lit: &str) -> Result<()> {
        let mut current = self.input.clone();
        let did_match = current.read_n(2) == Some(lit);
        if did_match {
            self.input = current;
            Ok(())
        } else {
            bail!("Expected literal `{}`", lit);
        }
    }
}

/*

# {% replace_in " " `${cringe}` %}
foo="lol"

foo = "lol" # {< replace_in " " `${cringe}` >}

*/
