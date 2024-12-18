use crate::script::eval_ctx::EvalCtx;
use miette::Result;

use super::{comment_style::CommentStyle, error::TemplateError, parser::Sp};

/// A single, full line with a tag in it. Contains the span of the entire line.
#[derive(Debug, Eq, PartialEq)]
pub struct TaggedLine<'a> {
    pub left: &'a str,
    pub tag: &'a str,
    pub right: &'a str,
    pub full_line: Sp<&'a str>,
}

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
        use crate::templating::parser;
        use miette::IntoDiagnostic as _;
        parser::parse_element(s).into_diagnostic()
    }

    pub fn render(
        &self,
        comment_style: &CommentStyle,
        eval_ctx: &mut EvalCtx,
    ) -> Result<String, TemplateError> {
        match self {
            Element::Plain(s) => Ok(s.as_str().to_string()),
            Element::Inline { line, expr, is_if } => match is_if {
                true => {
                    let eval_result = eval_ctx
                        .eval_rhai::<bool>(expr.as_str())
                        .map_err(|e| TemplateError::from_rhai(e, expr.range()))?;
                    Ok(comment_style.toggle_string(line.full_line.as_str(), eval_result))
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
                    &comment_style.toggle_string(
                        next_line.as_str(),
                        eval_ctx
                            .eval_rhai::<bool>(expr.as_str())
                            .map_err(|e| TemplateError::from_rhai(e, expr.range()))?
                    )
                )),
                false => Ok(format!(
                    "{}{}",
                    line.full_line.as_str(),
                    run_transformation_expr(eval_ctx, next_line.as_str(), expr)?
                )),
            },
            Element::MultiLine { block, end } => {
                let rendered_body = render_elements(comment_style, eval_ctx, &block.body)?;
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
                            .eval_rhai::<bool>(block.expr.as_str())
                            .map_err(|e| TemplateError::from_rhai(e, block.expr.range()))?;
                    had_true = had_true || expr_true;

                    let rendered_body = render_elements(comment_style, eval_ctx, &block.body)?;
                    output.push_str(block.tagged_line.full_line.as_str());
                    output.push_str(&comment_style.toggle_string(&rendered_body, expr_true));
                }
                if let Some(block) = else_block {
                    let expr_true = !had_true;
                    let rendered_body = render_elements(comment_style, eval_ctx, &block.body)?;
                    output.push_str(block.tagged_line.full_line.as_str());
                    output.push_str(&comment_style.toggle_string(&rendered_body, expr_true));
                }
                output.push_str(end.full_line.as_str());
                Ok(output)
            }
        }
    }
}

pub fn render_elements(
    comment_style: &CommentStyle,
    eval_ctx: &mut EvalCtx,
    elements: &[Element<'_>],
) -> Result<String, TemplateError> {
    let mut errs = Vec::new();
    let mut output = String::new();
    for element in elements {
        match element.render(comment_style, eval_ctx) {
            Ok(rendered) => output.push_str(&rendered),
            Err(e) => errs.push(e),
        }
    }
    if errs.is_empty() {
        Ok(output)
    } else {
        Err(TemplateError::Multiple(errs))
    }
}

fn run_transformation_expr(
    eval_ctx: &mut EvalCtx,
    text: &str,
    expr: &Sp<&str>,
) -> Result<String, TemplateError> {
    let result = eval_ctx
        .eval_text_transformation(text, expr.as_str())
        .map_err(|e| TemplateError::from_rhai(e, expr.range()))?;
    let second_pass = eval_ctx
        .eval_text_transformation(&result, expr.as_str())
        .map_err(|e| TemplateError::from_rhai(e, expr.range()))?;
    if result != second_pass {
        cov_mark::hit!(refuse_nonidempodent_transformation);
        println!(
            "Warning: Refusing to apply transformation that is not idempodent: `{}`",
            expr.as_str()
        );
        Ok(text.to_string())
    } else {
        Ok(result)
    }
}
