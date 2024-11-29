use pest_derive::Parser;

pub mod document;
pub mod element;
mod parser;

pub(crate) const COMMENT_START: &str = "<yolk> ";

#[derive(Debug)]
pub struct TaggedLine<'a> {
    left: &'a str,
    tag: &'a str,
    right: &'a str,
    full_line: &'a str,
}

#[derive(Parser)]
#[grammar = "templating/yolk.pest"]
pub struct YolkParser;
