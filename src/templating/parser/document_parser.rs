use std::{collections::VecDeque, ops::Range};

use super::{
    super::element::Element,
    comment_style::CommentStyle,
    linewise::{MultiLineTagKind, ParsedLine, TagKind},
    TaggedLine,
};

use miette::Diagnostic;
use pest::Span;

use crate::templating::element::ConditionalBlock;

#[derive(Debug, thiserror::Error, Diagnostic)]
#[diagnostic()]
pub enum Error {
    #[error("Expected {} but got {}", .1, .2)]
    UnexpectedElement(#[label("Here")] Range<usize>, &'static str, &'static str),
}

impl Error {
    fn unexpected(span: Span<'_>, expected: &'static str, got: &'static str) -> Self {
        let range = span.start()..span.end();
        Self::UnexpectedElement(range, expected, got)
    }

    pub fn span(&self) -> Range<usize> {
        match self {
            Self::UnexpectedElement(range, ..) => range.clone(),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

// TODO: Make this file not use anyhow::Error as the parser error type. Even as a temporary solution that's hideous.

pub struct DocumentParser<'a> {
    input: &'a str,
    lines: VecDeque<ParsedLine<'a>>,
    comment_style: Option<CommentStyle>,
}

impl<'a> DocumentParser<'a> {
    pub fn new(input: &'a str, lines: Vec<ParsedLine<'a>>) -> Self {
        Self {
            input,
            lines: lines.into(),
            comment_style: None,
        }
    }

    pub fn parse(mut self) -> Result<Vec<Element<'a>>> {
        let mut parsed = Vec::new();
        loop {
            match self.parse_element()? {
                Element::Eof => break,
                elem => parsed.push(elem),
            }
        }
        Ok(parsed)
    }

    fn mk_eof_error(&self, expected: &'static str) -> Error {
        Error::UnexpectedElement(self.input.len()..self.input.len(), expected, "eof")
    }

    fn parse_plain_line(&mut self) -> Result<Option<Span<'a>>> {
        match self.lines.pop_front() {
            Some(ParsedLine::Plain(raw)) => Ok(Some(raw)),
            Some(line) => Err(Error::unexpected(line.span(), "plain", line.kind())),
            None => Ok(None),
        }
    }

    fn parse_multiline_body(&mut self) -> Result<Vec<Element<'a>>> {
        let mut children = Vec::new();
        loop {
            let Some(next) = self.lines.front() else {
                Err(self.mk_eof_error("another line"))?
            };
            match next {
                ParsedLine::MultiLineTag { line: _, kind }
                    if !matches!(kind, MultiLineTagKind::If(_) | MultiLineTagKind::Regular(_)) =>
                {
                    return Ok(children);
                }
                _ => children.push(self.parse_element()?),
            }
        }
    }

    fn parse_end_line(&mut self) -> Result<TaggedLine<'a>> {
        let Some(line) = self.lines.pop_front() else {
            Err(self.mk_eof_error("end"))?
        };
        match line {
            ParsedLine::MultiLineTag {
                line,
                kind: MultiLineTagKind::End,
            } => Ok(line),
            line => {
                let err = Error::unexpected(line.span(), "end", line.kind());
                self.lines.push_front(line);
                Err(err)?
            }
        }
    }

    fn parse_else_line(&mut self) -> Result<TaggedLine<'a>> {
        let Some(line) = self.lines.pop_front() else {
            Err(self.mk_eof_error("else"))?
        };
        match line {
            ParsedLine::MultiLineTag {
                line,
                kind: MultiLineTagKind::Else,
            } => Ok(line),
            line => {
                let err = Error::unexpected(line.span(), "else", line.kind());
                self.lines.push_front(line);
                Err(err)?
            }
        }
    }
    fn parse_elif_line(&mut self) -> Result<(TaggedLine<'a>, &'a str)> {
        let Some(line) = self.lines.pop_front() else {
            Err(self.mk_eof_error("elif"))?
        };
        match line {
            ParsedLine::MultiLineTag {
                line,
                kind: MultiLineTagKind::Elif(expr),
            } => Ok((line, expr)),
            line => {
                let err = Error::unexpected(line.span(), "elif", line.kind());
                self.lines.push_front(line);
                Err(err)?
            }
        }
    }

    fn parse_element(&mut self) -> Result<Element<'a>> {
        let Some(line) = self.lines.pop_front() else {
            return Ok(Element::Eof);
        };
        if self.comment_style.is_none() {
            self.comment_style = CommentStyle::try_infer(&line);
        }
        match line {
            ParsedLine::MultiLineTag {
                line: if_line,
                kind: MultiLineTagKind::If(if_expr),
            } => {
                let mut blocks = Vec::new();
                let yes_body = self.parse_multiline_body()?;
                blocks.push(ConditionalBlock {
                    line: if_line,
                    expr: Some(if_expr),
                    body: yes_body,
                });
                loop {
                    if let Ok((line, expr)) = self.parse_elif_line() {
                        let body = self.parse_multiline_body()?;
                        blocks.push(ConditionalBlock {
                            line,
                            expr: Some(expr),
                            body,
                        });
                    } else if let Ok(line) = self.parse_else_line() {
                        let body = self.parse_multiline_body()?;
                        blocks.push(ConditionalBlock {
                            line,
                            expr: None,
                            body,
                        });
                    } else if let Ok(end) = self.parse_end_line() {
                        return Ok(Element::Conditional { blocks, end });
                    } else {
                        unreachable!(
                            "We know that parse_conditional_body always \
                            ends right before some conditional line"
                        );
                    }
                }
            }
            ParsedLine::MultiLineTag {
                line,
                kind: MultiLineTagKind::Regular(expr),
            } => {
                let body = self.parse_multiline_body()?;
                let end = self.parse_end_line()?;
                Ok(Element::MultiLine {
                    line,
                    expr,
                    body,
                    end,
                })
            }
            ParsedLine::MultiLineTag { line, kind } => Err(Error::unexpected(
                line.full_line,
                "anything else",
                kind.kind(),
            )),
            ParsedLine::NextLineTag { line, kind } => Ok(Element::NextLine {
                line,
                expr: kind.expr(),
                is_if: matches!(kind, TagKind::If(_)),
                next_line: match self.parse_plain_line()? {
                    Some(line) => line.as_str(),
                    None => todo!(
                        "Potentially keep incomplete stuff, in case \
                        we want to support evaluating partially invalid files"
                    ),
                },
            }),
            ParsedLine::InlineTag { line, kind } => Ok(Element::Inline {
                line,
                expr: kind.expr(),
                is_if: matches!(kind, TagKind::If(_)),
            }),
            ParsedLine::Plain(text) => Ok(Element::Plain(text)),
        }
    }
}
