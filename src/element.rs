use crate::ctx;

use anyhow::Result;
use pest::iterators::Pair;

use super::Rule;

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
    pub fn render(&self, render_ctx: &ctx::Context) -> Result<String> {
        match self {
            Element::Raw(s) => Ok(s.to_string()),
            Element::IfBlock {
                pred,
                if_tag,
                body,
                else_tag_and_body,
                end_tag,
            } => {
                let pred_true = pred.trim() == "true";
                let rendered_body = body.render(render_ctx)?;
                let rendered_else_body = else_tag_and_body
                    .as_ref()
                    .map(|(else_tag, else_body)| {
                        anyhow::Ok((else_tag, else_body.render(render_ctx)?))
                    })
                    .transpose()?;
                let mut output = String::new();
                output.push_str(if_tag);
                if pred_true {
                    output.push_str(&render_ctx.enabled_str(&rendered_body));
                } else {
                    output.push_str(&render_ctx.disabled_str(&rendered_body));
                }
                if let Some((else_tag, rendered_else_body)) = rendered_else_body {
                    output.push_str(else_tag);
                    if pred_true {
                        output.push_str(&render_ctx.disabled_str(&rendered_else_body));
                    } else {
                        output.push_str(&render_ctx.enabled_str(&rendered_else_body));
                    }
                }
                output.push_str(end_tag);
                Ok(output)
            }
            Element::Directive { tag, .. } => Ok(tag.to_string()),
        }
    }
}
