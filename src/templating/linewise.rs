use anyhow::Result;
use pest::iterators::{Pair, Pairs};

use super::{Rule, TaggedLine};

#[derive(Debug)]
pub enum ParsedLine<'a> {
    MultiLineTag {
        line: TaggedLine<'a>,
        kind: MultiLineTagKind<'a>,
    },
    NextLineTag {
        line: TaggedLine<'a>,
        kind: TagKind<'a>,
    },
    InlineTag {
        line: TaggedLine<'a>,
        kind: TagKind<'a>,
    },
    Raw(&'a str),
}

#[derive(Debug)]
pub enum MultiLineTagKind<'a> {
    Regular(&'a str),
    If(&'a str),
    Elif(&'a str),
    Else,
    End,
}

#[derive(Debug)]
pub enum TagKind<'a> {
    Regular(&'a str),
    If(&'a str),
}

impl<'a> TagKind<'a> {
    pub fn expr(&self) -> &'a str {
        match self {
            TagKind::Regular(expr) | TagKind::If(expr) => expr,
        }
    }
}

impl<'a> ParsedLine<'a> {
    pub fn try_from_pair(pair: Pair<'a, Rule>) -> Result<Self> {
        match pair.as_rule() {
            Rule::Raw => Ok(Self::Raw(pair.as_str())),
            Rule::nl => Ok(Self::Raw(pair.as_str())),
            Rule::LineNextLineTag => {
                let inner = pair.into_inner();
                let kind = inner.find_first_tagged("kind").unwrap();
                Ok(Self::NextLineTag {
                    kind: match kind.as_rule() {
                        Rule::NextLineTagIfInner => {
                            TagKind::If(inner.find_first_tagged("expr").unwrap().as_str())
                        }
                        Rule::NextLineTagRegularInner => TagKind::Regular(kind.as_str()),
                        _ => unreachable!(),
                    },
                    line: parse_tagged_line(inner),
                })
            }
            Rule::LineInlineTag => {
                let inner = pair.into_inner();
                let kind = inner.find_first_tagged("kind").unwrap();
                Ok(Self::InlineTag {
                    kind: match kind.as_rule() {
                        Rule::InlineTagIfInner => {
                            TagKind::If(inner.find_first_tagged("expr").unwrap().as_str())
                        }
                        Rule::InlineTagRegularInner => TagKind::Regular(kind.as_str()),
                        _ => unreachable!(),
                    },
                    line: parse_tagged_line(inner),
                })
            }

            Rule::LineMultiLineTag => {
                let inner = pair.into_inner();
                let kind = inner.find_first_tagged("kind").unwrap();
                let expr = inner.find_first_tagged("expr");
                Ok(Self::MultiLineTag {
                    line: parse_tagged_line(inner),
                    kind: match kind.as_rule() {
                        Rule::MultiLineTagRegularInner => {
                            MultiLineTagKind::Regular(expr.unwrap().as_str())
                        }
                        Rule::MultiLineTagIfInner => MultiLineTagKind::If(expr.unwrap().as_str()),
                        Rule::MultiLineTagElseIfInner => {
                            MultiLineTagKind::Elif(expr.unwrap().as_str())
                        }
                        Rule::MultiLineTagElseInner => MultiLineTagKind::Else,
                        Rule::MultiLineTagEndInner => MultiLineTagKind::End,
                        _ => unreachable!(),
                    },
                })
            }
            _ => {
                todo!()
            }
        }
    }
}

fn parse_tagged_line<'a>(inner: Pairs<'a, Rule>) -> TaggedLine<'a> {
    let left = inner.find_first_tagged("left").unwrap();
    let tag = inner.find_first_tagged("tag").unwrap();
    let right = inner.find_first_tagged("right").unwrap();
    TaggedLine {
        left: left.as_str(),
        tag: tag.as_str(),
        right: right.as_str(),
        full_line: inner.as_str(),
    }
}
