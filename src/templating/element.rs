use anyhow::{Context as _, Result};
use pest::{iterators::Pair, Parser};
use regex::Regex;

use crate::eval_ctx::EvalCtx;

use super::{document::Context, Rule, YolkParser};

#[derive(Debug)]
pub enum Element<'a> {
    Raw(&'a str),
    IfBlock {
        pred: &'a str,
        if_tag: &'a str,
        body: Box<Element<'a>>,
        else_tag_and_body: Option<(&'a str, Box<Element<'a>>)>,
        end_tag: &'a str,
    },
    ReplaceBlock {
        tag: &'a str,
        regex_pattern: &'a str,
        expr: &'a str,
        affected_line: &'a str,
    },
    Directive {
        tag: &'a str,
        name: &'a str,
        content: &'a str,
    },
}

impl<'a> Element<'a> {
    pub fn try_from_pair(pair: Pair<'a, Rule>) -> Result<Self> {
        match pair.as_rule() {
            Rule::IfBlock => {
                let inner = pair.into_inner();
                let pred = inner.find_first_tagged("pred").unwrap();
                let if_tag = inner.find_first_tagged("if").unwrap();
                let body = Box::new(Element::try_from_pair(
                    inner.find_first_tagged("body").unwrap(),
                )?);
                let end_tag = inner.find_first_tagged("end").unwrap();

                let else_tag_and_body = inner
                    .find_first_tagged("else")
                    .zip(inner.find_first_tagged("elsebody"))
                    .map(|(else_tag, else_body)| {
                        let else_body = Box::new(Element::try_from_pair(else_body)?);
                        anyhow::Ok((else_tag.as_str(), else_body))
                    })
                    .transpose()?;
                Ok(Element::IfBlock {
                    pred: pred.as_str(),
                    if_tag: if_tag.as_str(),
                    body,
                    else_tag_and_body,
                    end_tag: end_tag.as_str(),
                })
            }
            Rule::ReplaceBlock => {
                let block_inner = pair.into_inner();
                let tag = block_inner
                    .find_first_tagged("replace_tag")
                    .expect("no tag");
                let tag_str = tag.as_str();
                let tag_inner = tag.into_inner();
                Ok(Element::ReplaceBlock {
                    tag: tag_str,
                    regex_pattern: tag_inner
                        .find_first_tagged("regexp")
                        .expect("No regex")
                        .into_inner()
                        .as_str(),
                    expr: tag_inner
                        .find_first_tagged("expr")
                        .expect("No expr")
                        .as_str(),
                    affected_line: block_inner
                        .find_first_tagged("affected")
                        .expect("No affected line")
                        .as_str(),
                })
            }
            Rule::DirectiveTag => {
                let tag = pair.as_str();
                let inner = pair.into_inner();
                let name = inner
                    .find_first_tagged("name")
                    .expect("Missing 'name' tag")
                    .as_str();
                let content = inner
                    .find_first_tagged("value")
                    .expect("Missing 'value' tag")
                    .as_str();
                Ok(Element::Directive { tag, name, content })
            }
            Rule::Raw => Ok(Element::Raw(pair.as_str())),
            _ => Ok(Element::Raw(pair.as_str())),
        }
    }

    pub fn render(&self, render_ctx: &Context, eval_ctx: &mut EvalCtx<'_>) -> Result<String> {
        match self {
            Element::Raw(s) => Ok(s.to_string()),
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
                        // TODO fail here instead
                        output.push_str(&affected_line);
                        return Ok(output);
                    }
                }

                output.push_str(&after_replacement);
                Ok(output)
            }
            Element::Directive { tag, .. } => Ok(tag.to_string()),
        }
    }

    #[allow(unused)]
    pub fn parse(s: &'a str) -> Result<Self> {
        let mut pairs = YolkParser::parse(Rule::Element, s)?;
        Ok(Self::try_from_pair(pairs.next().context("No content")?)?)
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
