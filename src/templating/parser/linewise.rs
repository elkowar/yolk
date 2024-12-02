use anyhow::Result;
use pest::{
    iterators::{Pair, Pairs},
    Parser as _, Span,
};

use super::{Rule, TaggedLine, YolkParser};

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
    Plain(Span<'a>),
}

#[derive(Debug)]
pub enum MultiLineTagKind<'a> {
    Regular(&'a str),
    If(&'a str),
    Elif(&'a str),
    Else,
    End,
}
impl<'a> MultiLineTagKind<'a> {
    pub fn kind(&self) -> &'static str {
        match self {
            MultiLineTagKind::Regular(_) => "regular",
            MultiLineTagKind::If(_) => "if",
            MultiLineTagKind::Elif(_) => "elif",
            MultiLineTagKind::Else => "else",
            MultiLineTagKind::End => "end",
        }
    }
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
    pub fn kind(&self) -> &'static str {
        match self {
            TagKind::Regular(_) => "tag",
            TagKind::If(_) => "conditional",
        }
    }
}

impl<'a> ParsedLine<'a> {
    pub fn kind(&self) -> &'static str {
        match self {
            ParsedLine::MultiLineTag { kind, .. } => kind.kind(),
            ParsedLine::NextLineTag { kind, .. } => kind.kind(),
            ParsedLine::InlineTag { kind, .. } => kind.kind(),
            ParsedLine::Plain(_) => "plain",
        }
    }
    pub fn span(&self) -> Span<'a> {
        match self {
            ParsedLine::MultiLineTag { line, .. } => line.full_line,
            ParsedLine::NextLineTag { line, .. } => line.full_line,
            ParsedLine::InlineTag { line, .. } => line.full_line,
            ParsedLine::Plain(span) => *span,
        }
    }
    #[allow(unused)]
    pub fn try_from_str(s: &'a str) -> Result<Self> {
        let mut result = YolkParser::parse(Rule::Line, s)?;
        Ok(Self::from_pair(result.next().unwrap()))
    }

    pub fn from_pair(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::Plain => Self::Plain(pair.as_span()),
            Rule::nl => Self::Plain(pair.as_span()),
            Rule::LineNextLineTag => {
                let span = pair.as_span();
                let inner = pair.into_inner();
                let kind = inner.find_first_tagged("kind").unwrap();
                Self::NextLineTag {
                    kind: match kind.as_rule() {
                        Rule::NextLineTagIfInner => {
                            TagKind::If(inner.find_first_tagged("expr").unwrap().as_str())
                        }
                        Rule::NextLineTagRegularInner => TagKind::Regular(kind.as_str()),
                        _ => unreachable!(),
                    },
                    line: parse_tagged_line(span, inner),
                }
            }
            Rule::LineInlineTag => {
                let span = pair.as_span();
                let inner = pair.into_inner();
                let kind = inner.find_first_tagged("kind").unwrap();
                Self::InlineTag {
                    kind: match kind.as_rule() {
                        Rule::InlineTagIfInner => {
                            TagKind::If(inner.find_first_tagged("expr").unwrap().as_str())
                        }
                        Rule::InlineTagRegularInner => TagKind::Regular(kind.as_str()),
                        _ => unreachable!(),
                    },
                    line: parse_tagged_line(span, inner),
                }
            }

            Rule::LineMultiLineTag => {
                let span = pair.as_span();
                let inner = pair.into_inner();
                let kind = inner.find_first_tagged("kind").unwrap();
                let expr = inner.find_first_tagged("expr");
                Self::MultiLineTag {
                    line: parse_tagged_line(span, inner),
                    kind: match kind.as_rule() {
                        Rule::MultiLineTagRegularInner => MultiLineTagKind::Regular(kind.as_str()),
                        Rule::MultiLineTagIfInner => MultiLineTagKind::If(expr.unwrap().as_str()),
                        Rule::MultiLineTagElseIfInner => {
                            MultiLineTagKind::Elif(expr.unwrap().as_str())
                        }
                        Rule::MultiLineTagElseInner => MultiLineTagKind::Else,
                        Rule::MultiLineTagEndInner => MultiLineTagKind::End,
                        _ => unreachable!(),
                    },
                }
            }
            _ => {
                unreachable!("No other rules should be possible here")
            }
        }
    }
}

fn parse_tagged_line<'a>(span: Span<'a>, inner: Pairs<'a, Rule>) -> TaggedLine<'a> {
    let left = inner.find_first_tagged("left").unwrap();
    let tag = inner.find_first_tagged("tag").unwrap();
    let right = inner.find_first_tagged("right").unwrap();
    TaggedLine {
        left: left.as_str(),
        tag: tag.as_str(),
        right: right.as_str(),
        full_line: span,
    }
}
