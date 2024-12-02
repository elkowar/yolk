use std::collections::VecDeque;

use super::{
    super::element::Element,
    comment_style::CommentStyle,
    linewise::{MultiLineTagKind, ParsedLine, TagKind},
    TaggedLine,
};

use ariadne::ReportKind;
use pest::Span;

use crate::templating::element::ConditionalBlock;

#[derive(Debug, thiserror::Error)]
pub enum Error<'a> {
    #[error("Unexpected {}", .1)]
    UnexpectedElement(Span<'a>, String),
    #[error("Expected {} but got {}", .1, .2)]
    UnexpectedElementWithExpected(Span<'a>, &'static str, &'static str),
    #[error("Expected {} but got EOF", .1)]
    UnexpectedEof(Span<'a>, &'static str),
}

impl<'a> Error<'a> {
    pub fn into_report(self) -> ariadne::Report<'a> {
        let span = match self {
            Error::UnexpectedElement(span, _) => span,
            Error::UnexpectedElementWithExpected(span, _, _) => span,
            Error::UnexpectedEof(span, _) => span,
        };
        ariadne::Report::build(ReportKind::Error, span.start()..span.end())
            .with_message(self)
            .finish()
    }
}

type Result<'a, T> = std::result::Result<T, Error<'a>>;

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

    pub fn parse(mut self) -> Result<'a, Vec<Element<'a>>> {
        let mut parsed = Vec::new();
        loop {
            match self.parse_element()? {
                Element::Eof => break,
                elem => parsed.push(elem),
            }
        }
        Ok(parsed)
    }

    fn mk_eof_error(&self, expected: &'static str) -> Error<'a> {
        Error::UnexpectedEof(
            Span::new(self.input, self.input.len(), self.input.len()).unwrap(),
            expected,
        )
    }

    fn parse_plain_line(&mut self) -> Result<'a, Option<Span<'a>>> {
        match self.lines.pop_front() {
            Some(ParsedLine::Plain(raw)) => Ok(Some(raw)),
            Some(line) => Err(Error::UnexpectedElementWithExpected(
                line.span(),
                "plain",
                line.kind(),
            )),
            None => Ok(None),
        }
    }

    fn parse_multiline_body(&mut self) -> Result<'a, Vec<Element<'a>>> {
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

    fn parse_end_line(&mut self) -> Result<'a, TaggedLine<'a>> {
        let Some(line) = self.lines.pop_front() else {
            Err(self.mk_eof_error("end"))?
        };
        match line {
            ParsedLine::MultiLineTag {
                line,
                kind: MultiLineTagKind::End,
            } => Ok(line),
            line => {
                let err = Error::UnexpectedElementWithExpected(line.span(), "end", line.kind());
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
                let err = Error::UnexpectedElementWithExpected(line.span(), "else", line.kind());
                self.lines.push_front(line);
                Err(err)?
            }
        }
    }
    fn parse_elif_line(&mut self) -> Result<'a, (TaggedLine<'a>, &'a str)> {
        let Some(line) = self.lines.pop_front() else {
            Err(self.mk_eof_error("elif"))?
        };
        match line {
            ParsedLine::MultiLineTag {
                line,
                kind: MultiLineTagKind::Elif(expr),
            } => Ok((line, expr)),
            line => {
                let err = Error::UnexpectedElementWithExpected(line.span(), "elif", line.kind());
                self.lines.push_front(line);
                Err(err)?
            }
        }
    }

    fn parse_element(&mut self) -> Result<'a, Element<'a>> {
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
            ParsedLine::MultiLineTag { line, kind } => Err(Error::UnexpectedElement(
                line.full_line,
                kind.kind().to_string(),
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
