use std::ops::Range;

use crate::eval_ctx::EvalCtx;
use miette::{IntoDiagnostic, Result};

use super::{
    document::RenderContext,
    parser::{self, Sp, TaggedLine},
    template_error::TemplateError,
};

/// The starting line and body of a block, such as a multiline tag or part of a conditional.
///
/// `Expr` should either be `Sp<&'a str>` or `()`.
#[derive(Debug, Eq, PartialEq)]
pub struct Block<'a, Expr = Sp<&'a str>> {
    /// The full line including the tag
    pub tagged_line: TaggedLine<'a>,
    pub expr: Expr,
    pub body: Vec<Element<'a>>,
}

impl<'a, Expr> Block<'a, Expr> {
    pub fn map_expr<T>(self, f: impl FnOnce(Expr) -> T) -> Block<'a, T> {
        Block {
            tagged_line: self.tagged_line,
            expr: f(self.expr),
            body: self.body,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Element<'a> {
    Plain(Sp<&'a str>),
    Inline {
        /// The full line including the tag
        line: TaggedLine<'a>,
        expr: Sp<&'a str>,
        is_if: bool,
    },
    NextLine {
        /// The full line including the tag
        tagged_line: TaggedLine<'a>,
        expr: Sp<&'a str>,
        next_line: Sp<&'a str>,
        is_if: bool,
    },
    MultiLine {
        block: Block<'a, Sp<&'a str>>,
        end: TaggedLine<'a>,
    },
    Conditional {
        blocks: Vec<Block<'a, Sp<&'a str>>>,
        else_block: Option<Block<'a, ()>>,
        end: TaggedLine<'a>,
    },
}

impl<'a> Element<'a> {
    #[allow(unused)]
    pub fn try_from_str(s: &'a str) -> Result<Self> {
        parser::parse_element(s).into_diagnostic()
    }

    pub fn span(&self) -> Range<usize> {
        match self {
            Element::Plain(s) => s.range(),
            Element::Inline { line, .. } => line.full_line.range(),
            Element::NextLine {
                tagged_line,
                next_line,
                ..
            } => tagged_line.full_line.range().start..next_line.range().end,
            Element::MultiLine { block, end } => {
                block.tagged_line.full_line.range().start..end.full_line.range().end
            }
            Element::Conditional { blocks, end, .. } => {
                blocks.first().map_or(end.full_line.range(), |block| {
                    block.tagged_line.full_line.range().start..end.full_line.range().end
                })
            }
        }
    }

    pub fn render(
        &self,
        render_ctx: &RenderContext,
        eval_ctx: &mut EvalCtx,
    ) -> Result<String, TemplateError> {
        match self {
            Element::Plain(s) => Ok(s.as_str().to_string()),
            Element::Inline { line, expr, is_if } => match is_if {
                true => {
                    let eval_result = eval_ctx
                        .eval_lua::<bool>("template", expr.as_str())
                        .map_err(|e| TemplateError::from_lua_error(e, expr.range()))?;
                    Ok(render_ctx.string_toggled(line.full_line.as_str(), eval_result))
                }
                false => Ok(format!(
                    "{}{}{}",
                    run_transformation_expr(eval_ctx, line.left, expr)?,
                    line.tag,
                    line.right
                )),
            },
            Element::NextLine {
                tagged_line: line,
                expr,
                next_line,
                is_if,
            } => match is_if {
                true => Ok(format!(
                    "{}{}",
                    line.full_line.as_str(),
                    &render_ctx.string_toggled(
                        next_line.as_str(),
                        eval_ctx
                            .eval_lua::<bool>("template", expr.as_str())
                            .map_err(|e| TemplateError::from_lua_error(e, expr.range()))?
                    )
                )),
                false => Ok(format!(
                    "{}{}",
                    line.full_line.as_str(),
                    run_transformation_expr(eval_ctx, next_line.as_str(), expr)?
                )),
            },
            Element::MultiLine { block, end } => {
                let rendered_body = render_elements(render_ctx, eval_ctx, &block.body)?;
                Ok(format!(
                    "{}{}{}",
                    block.tagged_line.full_line.as_str(),
                    &run_transformation_expr(eval_ctx, &rendered_body, &block.expr)?,
                    end.full_line.as_str(),
                ))
            }
            Element::Conditional {
                blocks,
                else_block,
                end,
            } => {
                let mut output = String::new();
                let mut had_true = false;
                for block in blocks {
                    // If we've already had a true block, we want to return false for every other one.
                    // If we haven't, and there's an expression, evaluate it.
                    // If there isn't, we're on the else block, which should be true iff we haven't had a true block yet.
                    let expr_true = !had_true
                        && eval_ctx
                            .eval_lua::<bool>("template", block.expr.as_str())
                            .map_err(|e| TemplateError::from_lua_error(e, block.expr.range()))?;
                    had_true = had_true || expr_true;

                    let rendered_body = render_elements(render_ctx, eval_ctx, &block.body)?;
                    output.push_str(block.tagged_line.full_line.as_str());
                    output.push_str(&render_ctx.string_toggled(&rendered_body, expr_true));
                }
                if let Some(block) = else_block {
                    let expr_true = !had_true;
                    let rendered_body = render_elements(render_ctx, eval_ctx, &block.body)?;
                    output.push_str(block.tagged_line.full_line.as_str());
                    output.push_str(&render_ctx.string_toggled(&rendered_body, expr_true));
                }
                output.push_str(end.full_line.as_str());
                Ok(output)
            }
        }
    }
}

fn render_elements(
    render_ctx: &RenderContext,
    eval_ctx: &mut EvalCtx,
    elements: &[Element<'_>],
) -> Result<String, TemplateError> {
    elements
        .iter()
        .map(|x| x.render(render_ctx, eval_ctx))
        .collect()
}

fn run_transformation_expr(
    eval_ctx: &mut EvalCtx,
    text: &str,
    expr: &Sp<&str>,
) -> Result<String, TemplateError> {
    let result = eval_ctx
        .eval_text_transformation(text, expr.as_str())
        .map_err(|e| TemplateError::from_lua_error(e, expr.range()))?;
    let second_pass = eval_ctx
        .eval_text_transformation(&result, expr.as_str())
        .map_err(|e| TemplateError::from_lua_error(e, expr.range()))?;
    if result != second_pass {
        println!(
            "Warning: Refusing to apply transformation that is not idempodent: `{}`",
            expr.as_str()
        );
        Ok(text.to_string())
    } else {
        Ok(result)
    }
}
