use pest::Span;
use pest_derive::Parser;

pub mod comment_style;
pub mod document_parser;
pub mod linewise;

#[derive(Debug, Eq, PartialEq)]
pub struct TaggedLine<'a> {
    pub left: &'a str,
    pub tag: &'a str,
    pub right: &'a str,
    pub full_line: Span<'a>,
}

#[derive(Parser)]
#[grammar = "templating/parser/yolk.pest"]
pub struct YolkParser;
