use std::{collections::VecDeque, ops::Range};

use super::{
    super::element::Element,
    comment_style::CommentStyle,
    linewise::{MultiLineTagKind, ParsedLine, TagKind},
    TaggedLine,
};

use miette::{Diagnostic, LabeledSpan};
use pest::Span;

use crate::templating::element::ConditionalBlock;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Expected {} but got {}", .1, .2)]
    UnexpectedElement(Range<usize>, &'static str, &'static str),

    #[error("{}", .inner)]
    WithinBlock {
        started_at: Range<usize>,
        #[source]
        inner: Box<Error>,
    },
}

impl Diagnostic for Error {
    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        match self {
            Error::UnexpectedElement(span, expected, _) => Some(Box::new(std::iter::once(
                LabeledSpan::at(span.clone(), format!("expected {expected}")),
            ))),
            Error::WithinBlock { started_at, inner } => {
                let mut labels = vec![LabeledSpan::at(started_at.clone(), "block started here")];
                if let Some(l) = inner.labels() {
                    labels.extend(l);
                }
                Some(Box::new(labels.into_iter()))
            }
        }
    }
    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        match self {
            Self::WithinBlock { inner, .. } => Some(inner.as_ref() as &dyn Diagnostic),
            _ => None,
        }
    }
}

impl Error {
    fn unexpected(span: Span<'_>, expected: &'static str, got: &'static str) -> Self {
        let range = span.start()..span.end();
        Self::UnexpectedElement(range, expected, got)
    }
    fn within_block(started_at: Span<'_>, inner: Error) -> Self {
        let range = started_at.start()..started_at.end();
        Self::WithinBlock {
            started_at: range,
            inner: Box::new(inner),
        }
    }

    #[allow(dead_code)]
    pub fn span(&self) -> Range<usize> {
        match self {
            Self::UnexpectedElement(range, ..) => range.clone(),
            Self::WithinBlock { inner, .. } => inner.span(),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

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
        let span_start = if !self.input.is_empty() {
            self.input.len() - 1
        } else {
            self.input.len()
        };
        Error::UnexpectedElement(span_start..self.input.len(), expected, "eof")
    }

    fn parse_plain_line(&mut self) -> Result<Span<'a>> {
        match self.lines.pop_front() {
            Some(ParsedLine::Plain(raw)) => Ok(raw),
            Some(line) => Err(Error::unexpected(line.span(), "plain line", line.kind())),
            None => Err(self.mk_eof_error("plain line")),
        }
    }

    fn parse_multiline_body(&mut self, starting_line: &Span<'a>) -> Result<Vec<Element<'a>>> {
        let mut children = Vec::new();
        loop {
            let Some(next) = self.lines.front() else {
                Err(Error::within_block(
                    *starting_line,
                    self.mk_eof_error("end of block"),
                ))?
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
    fn parse_elif_line(&mut self) -> Result<(TaggedLine<'a>, Span<'a>)> {
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

    fn parse_if_element(
        &mut self,
        if_line: TaggedLine<'a>,
        if_expr: Span<'a>,
    ) -> Result<Element<'a>> {
        let mut blocks = Vec::new();
        let yes_body = self.parse_multiline_body(&if_line.full_line)?;
        blocks.push(ConditionalBlock {
            line: if_line,
            expr: Some(if_expr),
            body: yes_body,
        });
        loop {
            if let Ok((line, expr)) = self.parse_elif_line() {
                let body = self.parse_multiline_body(&line.full_line)?;
                blocks.push(ConditionalBlock {
                    line,
                    expr: Some(expr),
                    body,
                });
            } else if let Ok(line) = self.parse_else_line() {
                let body = self.parse_multiline_body(&line.full_line)?;
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
            } => self.parse_if_element(if_line, if_expr),
            ParsedLine::MultiLineTag {
                line,
                kind: MultiLineTagKind::Regular(expr),
            } => {
                let body = self.parse_multiline_body(&line.full_line)?;
                let end = self
                    .parse_end_line()
                    .map_err(|e| Error::within_block(line.full_line, e))?;
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
                expr: kind.expr(),
                is_if: matches!(kind, TagKind::If(_)),
                next_line: self
                    .parse_plain_line()
                    .map_err(|e| Error::within_block(line.full_line, e))?
                    .as_str(),
                line,
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
