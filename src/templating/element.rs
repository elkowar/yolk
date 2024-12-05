use crate::eval_ctx::EvalCtx;
use miette::Result;
use pest::Span;

use super::{document::RenderContext, parser::TaggedLine};

#[derive(Debug, Eq, PartialEq)]
pub struct ConditionalBlock<'a> {
    pub line: TaggedLine<'a>,
    pub expr: Option<Span<'a>>,
    pub body: Vec<Element<'a>>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Element<'a> {
    Plain(Span<'a>),
    Inline {
        line: TaggedLine<'a>,
        expr: Span<'a>,
        is_if: bool,
    },
    NextLine {
        line: TaggedLine<'a>,
        expr: Span<'a>,
        next_line: &'a str,
        is_if: bool,
    },
    MultiLine {
        line: TaggedLine<'a>,
        expr: Span<'a>,
        body: Vec<Element<'a>>,
        end: TaggedLine<'a>,
    },
    Conditional {
        blocks: Vec<ConditionalBlock<'a>>,
        end: TaggedLine<'a>,
    },
    Eof,
}

impl<'a> Element<'a> {
    pub fn render(&self, render_ctx: &RenderContext, eval_ctx: &mut EvalCtx) -> Result<String> {
        match self {
            Element::Plain(s) => Ok(s.as_str().to_string()),
            Element::Inline { line, expr, is_if } => match is_if {
                true => {
                    let eval_result = eval_ctx.eval_lua::<bool>("template", expr.as_str())?;
                    Ok(render_ctx.string_toggled(line.full_line.as_str(), eval_result))
                }
                false => Ok(format!(
                    "{}{}{}",
                    run_transformation_expr(eval_ctx, line.left, expr.as_str())?,
                    line.tag,
                    line.right
                )),
            },
            Element::NextLine {
                line,
                expr,
                next_line,
                is_if,
            } => match is_if {
                true => Ok(format!(
                    "{}{}",
                    line.full_line.as_str(),
                    &render_ctx.string_toggled(
                        next_line,
                        eval_ctx.eval_lua::<bool>("template", expr.as_str())?
                    )
                )),
                false => Ok(format!(
                    "{}{}",
                    line.full_line.as_str(),
                    run_transformation_expr(eval_ctx, next_line, expr.as_str())?
                )),
            },
            Element::MultiLine {
                line,
                expr,
                body,
                end,
            } => {
                let rendered_body = render_elements(render_ctx, eval_ctx, body)?;
                Ok(format!(
                    "{}{}{}",
                    line.full_line.as_str(),
                    &run_transformation_expr(eval_ctx, &rendered_body, expr.as_str())?,
                    end.full_line.as_str(),
                ))
            }
            Element::Conditional { blocks, end } => {
                let mut output = String::new();
                let mut had_true = false;
                for block in blocks {
                    // If we've already had a true block, we want to return false for every other one.
                    // If we haven't, and there's an expression, evaluate it.
                    // If there isn't, we're on the else block, which should be true iff we haven't had a true block yet.
                    let expr_true = match block.expr {
                        Some(expr) => {
                            !had_true && eval_ctx.eval_lua::<bool>("template", expr.as_str())?
                        }
                        None => !had_true,
                    };
                    had_true = had_true || expr_true;

                    let rendered_body = render_elements(render_ctx, eval_ctx, &block.body)?;
                    output.push_str(block.line.full_line.as_str());
                    output.push_str(&render_ctx.string_toggled(&rendered_body, expr_true));
                }
                output.push_str(end.full_line.as_str());
                Ok(output)
            }
            Element::Eof => Ok("".to_string()),
        }
    }
}

fn render_elements(
    render_ctx: &RenderContext,
    eval_ctx: &mut EvalCtx,
    elements: &[Element<'_>],
) -> Result<String> {
    elements
        .iter()
        .map(|x| x.render(render_ctx, eval_ctx))
        .collect::<Result<String>>()
}

fn run_transformation_expr(eval_ctx: &mut EvalCtx, text: &str, expr: &str) -> Result<String> {
    let result = eval_ctx.eval_text_transformation(text, expr)?;
    let second_pass = eval_ctx.eval_text_transformation(&result, expr)?;
    if result != second_pass {
        println!("Warning: Refusing to apply transformation that is not idempodent: `{expr}`",);
        Ok(text.to_string())
    } else {
        Ok(result)
    }
}
