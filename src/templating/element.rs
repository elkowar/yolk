use std::collections::VecDeque;

use anyhow::{bail, Context as _, Result};
use pest::{iterators::Pair, Parser};
use regex::Regex;

use crate::eval_ctx::EvalCtx;

use super::{
    document::Context,
    linewise::{self, MultiLineTagKind, ParsedLine, TagKind},
    Rule, TaggedLine, YolkParser,
};
// TODO: Instead of parsing separate specific syntax per element, what if
// we just parsed {% rhai %}, and made a set of rhai functions that return objects that tell us _what_ that tag does?
// that might make a lot of things a lot easier...
// However, how would that work with conditionals and their `else` blocks? are those just magic syntax again?

#[derive(Debug)]
pub struct ConditionalBlock<'a> {
    line: TaggedLine<'a>,
    expr: Option<&'a str>,
    body: Vec<Element<'a>>,
}

#[derive(Debug)]
pub enum Element<'a> {
    Raw(&'a str),
    Inline {
        line: TaggedLine<'a>,
        expr: &'a str,
        is_if: bool,
    },
    NextLine {
        line: TaggedLine<'a>,
        expr: &'a str,
        next_line: &'a str,
        is_if: bool,
    },
    MultiLine {
        line: TaggedLine<'a>,
        expr: &'a str,
        body: Vec<Element<'a>>,
        end: TaggedLine<'a>,
    },
    Conditional {
        if_block: ConditionalBlock<'a>,
        elifs: Vec<ConditionalBlock<'a>>,
        else_block: Option<ConditionalBlock<'a>>,
        end: TaggedLine<'a>,
    },
    Eof,
}

pub struct ElementParser<'a> {
    lines: VecDeque<ParsedLine<'a>>,
    parsed: Vec<Element<'a>>,
}

impl<'a> ElementParser<'a> {
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
    pub fn parse_raw_line(&mut self) -> Result<Option<&'a str>> {
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

    pub fn parse_end_line(&mut self) -> Result<TaggedLine<'a>> {
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

    pub fn parse_else_line(&mut self) -> Result<TaggedLine<'a>> {
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
    pub fn parse_elif_line(&mut self) -> Result<(TaggedLine<'a>, &'a str)> {
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

    pub fn parse_element(&mut self) -> Result<Element<'a>> {
        let Some(line) = self.lines.pop_front() else {
            return Ok(Element::Eof);
        };
        match line {
            ParsedLine::MultiLineTag {
                line: if_line,
                kind: MultiLineTagKind::If(if_expr),
            } => {
                let mut elifs = Vec::new();
                let mut else_block = None;
                let yes_body = self.parse_conditional_body()?;
                let if_block = ConditionalBlock {
                    line: if_line,
                    expr: Some(if_expr),
                    body: yes_body,
                };
                loop {
                    if let Ok((line, expr)) = self.parse_elif_line() {
                        let body = self.parse_conditional_body()?;
                        elifs.push(ConditionalBlock {
                            line,
                            expr: Some(expr),
                            body,
                        });
                    } else if let Ok(line) = self.parse_else_line() {
                        let body = self.parse_conditional_body()?;
                        else_block = Some(ConditionalBlock {
                            line,
                            expr: None,
                            body,
                        })
                    } else if let Ok(end) = self.parse_end_line() {
                        return Ok(Element::Conditional {
                            if_block,
                            elifs,
                            else_block,
                            end,
                        });
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

impl<'a> Element<'a> {
    pub fn render(&self, render_ctx: &Context, eval_ctx: &mut EvalCtx<'_>) -> Result<String> {
        match self {
            Element::Raw(s) => Ok(s.to_string()),

            Element::Conditional {
                if_block,
                elifs,
                else_block,
                end,
            } => {
                let pred_value: bool = eval_ctx.eval(if_block.expr.unwrap())?;
                todo!()
            } /*
                  Element::IfBlock {
                      pred,
                      if_tag,
                      body,
                      else_tag_and_body,
                      end_tag,
                  } => {
                      let pred_value: bool = eval_ctx.eval(pred.trim())?;

                      let rendered_body = body.render(render_ctx, eval_ctx)?;
                      let rendered_else_body = else_tag_and_body
                          .as_ref()
                          .map(|(else_tag, else_body)| {
                              anyhow::Ok((else_tag, else_body.render(render_ctx, eval_ctx)?))
                          })
                          .transpose()?;
                      let mut output = String::new();
                      output.push_str(if_tag);
                      if pred_value {
                          output.push_str(&render_ctx.enabled_str(&rendered_body));
                      } else {
                          output.push_str(&render_ctx.disabled_str(&rendered_body));
                      }
                      if let Some((else_tag, rendered_else_body)) = rendered_else_body {
                          output.push_str(else_tag);
                          if pred_value {
                              output.push_str(&render_ctx.disabled_str(&rendered_else_body));
                          } else {
                              output.push_str(&render_ctx.enabled_str(&rendered_else_body));
                          }
                      }
                      output.push_str(end_tag);
                      Ok(output)
                  }
                  Element::ReplaceBlock {
                      tag,
                      regex_pattern,
                      expr,
                      affected_line,
                  } => {
                      let replacement: rhai::Dynamic = eval_ctx.eval(expr.trim())?;
                      let replacement = replacement.to_string();
                      let mut output = tag.to_string();
                      let regex = Regex::new(regex_pattern)?;

                      let after_replacement = regex.replace(affected_line, &replacement);
                      let original_value = regex.find(affected_line);
                      if let Some(original_value) = original_value {
                          let reverted_line = regex.replace(&after_replacement, original_value.as_str());
                          if &reverted_line != affected_line {
                              eprintln!(
                                  "Warning: Refusing to apply non-reversible `replace` action: `{}` in `{}`",
                                  regex_pattern, affected_line
                              );
                              output.push_str(affected_line);
                              return Ok(output);
                          }
                      }

                      output.push_str(&after_replacement);
                      Ok(output)
                  }
                  Element::ReplaceInBlock {
                      tag,
                      left,
                      right,
                      expr,
                      affected_line,
                  } => {
                      let replacement: rhai::Dynamic = eval_ctx.eval(expr.trim())?;
                      let replacement = replacement.to_string();
                      let mut output = tag.to_string();
                      let regex = Regex::new(&format!("{}[^{}]*{}", left, right, right))?;

                      let after_replacement = regex.replace(affected_line, &replacement);
                      let original_value = regex.find(affected_line);
                      if let Some(original_value) = original_value {
                          let reverted_line = regex.replace(&after_replacement, original_value.as_str());
                          if &reverted_line != affected_line {
                              eprintln!(
                                  "Warning: Refusing to apply non-reversible `replace_in` action in `{affected_line}`",
                              );
                              output.push_str(affected_line);
                              return Ok(output);
                          }
                      }

                      output.push_str(&after_replacement);
                      Ok(output)
                  }
                  Element::ReplaceInline {
                      before_tag,
                      tag,
                      regex_pattern,
                      expr,
                  } => {
                      let replacement: rhai::Dynamic = eval_ctx.eval(expr.trim())?;
                      let replacement = replacement.to_string();
                      let mut output = String::new();
                      let regex = Regex::new(regex_pattern)?;

                      let after_replacement = regex.replace(before_tag, &replacement);
                      let original_value = regex.find(before_tag);
                      if let Some(original_value) = original_value {
                          let reverted_line = regex.replace(&after_replacement, original_value.as_str());
                          if &reverted_line != before_tag {
                              eprintln!(
                                  "Warning: Refusing to apply non-reversible `replace` action: `{}` in `{}`",
                                  regex_pattern, before_tag
                              );
                              output.push_str(before_tag);
                          } else {
                              output.push_str(&after_replacement);
                          }
                      } else {
                          output.push_str(&after_replacement);
                      }
                      output.push_str(tag);
                      Ok(output)
                  }
                  Element::ReplaceInInline {
                      before_tag,
                      tag,
                      left,
                      right,
                      expr,
                  } => {
                      let replacement: rhai::Dynamic = eval_ctx.eval(expr.trim())?;
                      let replacement = replacement.to_string();
                      let mut output = String::new();
                      let regex = Regex::new(&format!("{}[^{}]*{}", left, right, right))?;

                      let after_replacement = regex.replace(before_tag, &replacement);
                      let original_value = regex.find(before_tag);
                      if let Some(original_value) = original_value {
                          let reverted_line = regex.replace(&after_replacement, original_value.as_str());
                          if &reverted_line != before_tag {
                              eprintln!(
                                  "Warning: Refusing to apply non-reversible `replace_in` action in `{before_tag}`",
                              );
                              output.push_str(before_tag);
                          } else {
                              output.push_str(&after_replacement);
                          }
                      } else {
                          output.push_str(&after_replacement);
                      }

                      output.push_str(tag);
                      Ok(output)
                  }
                  Element::Directive { tag, .. } => Ok(tag.to_string()),
              */
        }
    }

    #[allow(unused)]
    pub fn parse(s: &'a str) -> Result<Self> {
        let mut pairs = YolkParser::parse(Rule::Element, s)?;
        Self::try_from_pair(pairs.next().context("No content")?)
    }
}

#[cfg(test)]
mod test {
    use testresult::TestResult;

    use crate::eval_ctx;
    use crate::templating::document::Context;
    use crate::templating::element::Element;
    #[test]
    pub fn test_render_replace() -> TestResult {
        let element = Element::parse("{% replace /'.*'/ `'new'` %}\nfoo: 'original'")?;
        let render_ctx = Context::default();
        let mut eval_ctx = eval_ctx::EvalCtx::new();
        assert_eq!(
            "{% replace /'.*'/ `'new'` %}\nfoo: 'new'",
            element.render(&render_ctx, &mut eval_ctx)?
        );
        Ok(())
    }

    #[test]
    pub fn test_render_replace_inline() -> TestResult {
        let element = Element::parse("foo: 'original' # {<< replace /'.*'/ `'new'` >>}")?;
        let render_ctx = Context::default();
        let mut eval_ctx = eval_ctx::EvalCtx::new();
        assert_eq!(
            "foo: 'new' # {<< replace /'.*'/ `'new'` >>}",
            element.render(&render_ctx, &mut eval_ctx)?
        );
        Ok(())
    }

    #[test]
    pub fn test_render_replace_refuse_nonreversible() -> TestResult {
        let element = Element::parse("{% replace /'.*'/ `no quotes` %}\nfoo: 'original'")?;
        let render_ctx = Context::default();
        let mut eval_ctx = eval_ctx::EvalCtx::new();
        assert_eq!(
            "{% replace /'.*'/ `no quotes` %}\nfoo: 'original'",
            element.render(&render_ctx, &mut eval_ctx)?
        );
        Ok(())
    }
}
