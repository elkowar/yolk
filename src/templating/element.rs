use crate::eval_ctx::EvalCtx;
use anyhow::Result;

use super::{document::Context, TaggedLine};

#[derive(Debug)]
pub struct ConditionalBlock<'a> {
    pub line: TaggedLine<'a>,
    pub expr: Option<&'a str>,
    pub body: Vec<Element<'a>>,
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
        blocks: Vec<ConditionalBlock<'a>>,
        end: TaggedLine<'a>,
    },
    Eof,
}

impl<'a> Element<'a> {
    pub fn render(&self, render_ctx: &Context, eval_ctx: &mut EvalCtx) -> Result<String> {
        match self {
            Element::Raw(s) => Ok(s.to_string()),
            Element::Inline { line, expr, is_if } => match is_if {
                true => Ok(render_ctx.string_toggled(line.full_line, eval_ctx.eval_expr(expr)?)),
                false => Ok(format!(
                    "{}{}{}",
                    run_transformation_expr(eval_ctx, line.left, expr)?,
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
                    line.full_line,
                    &render_ctx.string_toggled(next_line, eval_ctx.eval_expr(expr)?),
                )),
                false => Ok(format!(
                    "{}{}",
                    line.full_line,
                    run_transformation_expr(eval_ctx, next_line, expr)?
                )),
            },
            Element::MultiLine {
                line,
                expr,
                body,
                end,
            } => {
                let rendered_body = render_elements(render_ctx, eval_ctx, &body)?;
                Ok(format!(
                    "{}{}{}",
                    line.full_line,
                    &run_transformation_expr(eval_ctx, &rendered_body, expr)?,
                    end.full_line
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
                        Some(expr) => !had_true && eval_ctx.eval_expr(expr)?,
                        None => !had_true,
                    };
                    had_true = had_true || expr_true;

                    let rendered_body = render_elements(render_ctx, eval_ctx, &block.body)?;
                    output.push_str(block.line.full_line);
                    output.push_str(&render_ctx.string_toggled(&rendered_body, expr_true));
                }
                output.push_str(end.full_line);
                Ok(output)
            }
            Element::Eof => todo!(),
        }
    }
}

fn render_elements<'a>(
    render_ctx: &Context,
    eval_ctx: &mut EvalCtx,
    elements: &[Element<'a>],
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

#[cfg(test)]
mod test {
    use testresult::TestResult;

    use crate::eval_ctx;
    use crate::templating::document::Document;

    #[test]
    pub fn test_render_inline() -> TestResult {
        let mut eval_ctx = eval_ctx::EvalCtx::new_for_tag()?;
        let doc = Document::parse_string("color=red /* {< `_{YOLK_TEXT}_` >} */\n")?;
        assert_eq!(
            "_color=red /* _{< `_{YOLK_TEXT}_` >} */\n",
            doc.render(&mut eval_ctx)?
        );
        Ok(())
    }

    #[test]
    pub fn test_render_next_line() -> TestResult {
        let mut eval_ctx = eval_ctx::EvalCtx::new_for_tag()?;
        let doc = Document::parse_string("/* {# `_{YOLK_TEXT}_` #} */\nfoo\n")?;
        // TODO: Fix the fact that appending at the end here goes after the \n???
        assert_eq!(
            "/* {# `_{YOLK_TEXT}_` #} */\n_foo\n_",
            doc.render(&mut eval_ctx)?
        );
        Ok(())
    }

    #[test]
    pub fn test_multiline_conditional() -> TestResult {
        let mut eval_ctx = eval_ctx::EvalCtx::new_for_tag()?;
        let input_str = indoc::indoc! {r#"
            /* {% if false %} */
            foo
            /* {% elif true %} */
            bar
            /* {% else %} */
            bar
            /* {% end %} */
        "#};
        let doc = Document::parse_string(input_str)?;
        assert_eq!(
            indoc::indoc! {r#"
                /* {% if false %} */
                #<yolk> foo
                /* {% elif true %} */
                bar
                /* {% else %} */
                #<yolk> bar
                /* {% end %} */
            "#},
            doc.render(&mut eval_ctx)?
        );
        Ok(())
    }

    #[test]
    pub fn test_render_replace() -> TestResult {
        let doc = Document::parse_string(indoc::indoc! {"
            {# replace(`'.*'`, `'new'`) #}
            foo: 'original'
        "})?;
        let mut eval_ctx = eval_ctx::EvalCtx::new_for_tag()?;
        assert_eq!(
            indoc::indoc! {"
                {# replace(`'.*'`, `'new'`) #}
                foo: 'new'
            "},
            doc.render(&mut eval_ctx)?
        );
        Ok(())
    }

    #[test]
    pub fn test_render_replace_refuse_non_idempodent() -> TestResult {
        let element = Document::parse_string("{# replace(`'.*'`, `a'a'`) #}\nfoo: 'original'\n")?;
        let mut eval_ctx = eval_ctx::EvalCtx::new_for_tag()?;
        assert_eq!(
            "{# replace(`'.*'`, `a'a'`) #}\nfoo: 'original'\n",
            element.render(&mut eval_ctx)?
        );
        Ok(())
    }
}
