use std::collections::VecDeque;

use super::{
    super::element::Element,
    linewise::{MultiLineTagKind, ParsedLine, TagKind},
};

use anyhow::{bail, Result};

use crate::templating::{element::ConditionalBlock, TaggedLine};

pub struct DocumentParser<'a> {
    lines: VecDeque<ParsedLine<'a>>,
    parsed: Vec<Element<'a>>,
}

impl<'a> DocumentParser<'a> {
    pub fn new(lines: Vec<ParsedLine<'a>>) -> Self {
        Self {
            lines: lines.into(),
            parsed: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Result<Vec<Element<'a>>> {
        loop {
            let elem = self.parse_element()?;
            if matches!(elem, Element::Eof) {
                break;
            } else {
                self.parsed.push(elem);
            }
        }
        Ok(self.parsed)
    }

    fn parse_raw_line(&mut self) -> Result<Option<&'a str>> {
        let Some(line) = self.lines.pop_front() else {
            return Ok(None);
        };

        match line {
            ParsedLine::Raw(raw) => Ok(Some(raw)),
            _ => Err(anyhow::anyhow!("Expected raw line, got {:?}", line)),
        }
    }

    fn parse_conditional_body(&mut self) -> Result<Vec<Element<'a>>> {
        let mut children = Vec::new();
        loop {
            let Some(next) = self.lines.front() else {
                bail!("Expected another line in if body");
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
            bail!("Expected end line, got EOF");
        };
        match line {
            ParsedLine::MultiLineTag {
                line,
                kind: MultiLineTagKind::End,
            } => Ok(line),
            line => {
                let s = format!("{:?}", &line);
                self.lines.push_front(line);
                bail!("Expected end line, got {:?}", s);
            }
        }
    }

    fn parse_else_line(&mut self) -> Result<TaggedLine<'a>> {
        let Some(line) = self.lines.pop_front() else {
            bail!("Expected else line, got EOF");
        };
        match line {
            ParsedLine::MultiLineTag {
                line,
                kind: MultiLineTagKind::Else,
            } => Ok(line),
            line => {
                let s = format!("{:?}", &line);
                self.lines.push_front(line);
                bail!("Expected else line, got {:?}", s);
            }
        }
    }
    fn parse_elif_line(&mut self) -> Result<(TaggedLine<'a>, &'a str)> {
        let Some(line) = self.lines.pop_front() else {
            bail!("Expected elif line, got EOF");
        };
        match line {
            ParsedLine::MultiLineTag {
                line,
                kind: MultiLineTagKind::Elif(expr),
            } => Ok((line, expr)),
            line => {
                let s = format!("{:?}", &line);
                self.lines.push_front(line);
                bail!("Expected elif line, got {:?}", s);
            }
        }
    }

    fn parse_element(&mut self) -> Result<Element<'a>> {
        let Some(line) = self.lines.pop_front() else {
            return Ok(Element::Eof);
        };
        match line {
            ParsedLine::MultiLineTag {
                line: if_line,
                kind: MultiLineTagKind::If(if_expr),
            } => {
                let mut blocks = Vec::new();
                let yes_body = self.parse_conditional_body()?;
                blocks.push(ConditionalBlock {
                    line: if_line,
                    expr: Some(if_expr),
                    body: yes_body,
                });
                loop {
                    if let Ok((line, expr)) = self.parse_elif_line() {
                        let body = self.parse_conditional_body()?;
                        blocks.push(ConditionalBlock {
                            line,
                            expr: Some(expr),
                            body,
                        });
                    } else if let Ok(line) = self.parse_else_line() {
                        let body = self.parse_conditional_body()?;
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
                let body = self.parse_conditional_body()?;
                if let Ok(end_line) = self.parse_end_line() {
                    return Ok(Element::MultiLine {
                        line,
                        expr,
                        body,
                        end: end_line,
                    });
                } else {
                    unreachable!(
                        "We know that parse_conditional_body \
                        always ends right before some conditional line"
                    );
                }
            }
            ParsedLine::MultiLineTag { line: _, kind } => {
                // TODO: Ensure that kind has some sort of ".type()" function to use here, rather than printing all of this
                anyhow::bail!("Unexpected {:?}", kind)
            }
            ParsedLine::NextLineTag { line, kind } => Ok(Element::NextLine {
                line,
                expr: kind.expr(),
                is_if: matches!(kind, TagKind::If(_)),
                next_line: match self.parse_raw_line()? {
                    Some(line) => line,
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
            ParsedLine::Raw(text) => Ok(Element::Raw(text)),
        }
    }
}
