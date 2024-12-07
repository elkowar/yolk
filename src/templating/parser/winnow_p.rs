use std::ops::Range;

use winnow::{
    ascii::{line_ending, till_line_ending},
    combinator::{
        alt, cut_err, delimited, eof, not, opt, peek, preceded, repeat, repeat_till, terminated,
        trace,
    },
    error::{ContextError, StrContext},
    token::{any, literal},
    Parser,
};

type Input<'a> = winnow::stream::Recoverable<winnow::Located<&'a str>, ContextError>;
type PResult<T> = winnow::PResult<T, ContextError>;

#[derive(Debug, Eq, PartialEq)]
pub struct Sp<T> {
    range: Range<usize>,
    content: T,
}
impl<'a, T> Sp<T> {
    fn new(range: Range<usize>, content: T) -> Self {
        Self { range, content }
    }

    fn content(&self) -> &T {
        &self.content
    }

    fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Sp<U> {
        Sp::new(self.range, f(self.content))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct TaggedLine<'a> {
    pub left: &'a str,
    // pub expr: &'a str,
    pub right: &'a str,
    pub full_line: Sp<&'a str>,
}

/// The starting line and body of a block, such as a multiline tag or part of a conditional.
///
/// `Expr` should either be `Sp<&'a str>` or `()`.
#[derive(Debug, Eq, PartialEq)]
pub struct Block<'a, Expr = Sp<&'a str>> {
    pub line: TaggedLine<'a>,
    pub expr: Expr,
    pub body: Vec<Element<'a>>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Element<'a> {
    Plain(Sp<&'a str>),
    Inline {
        line: TaggedLine<'a>,
        expr: Sp<&'a str>,
        is_if: bool,
    },
    NextLine {
        line: TaggedLine<'a>,
        expr: Sp<&'a str>,
        next_line: &'a str,
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
    Eof,
}

pub fn document<'a>(input: &mut Input<'a>) -> PResult<Vec<Element<'a>>> {
    Ok(repeat(0.., p_element).parse_next(input)?)
}

fn p_element<'a>(input: &mut Input<'a>) -> PResult<Element<'a>> {
    alt((
        p_conditional_element.context(lbl("conditional element")),
        p_multiline_element.context(lbl("multiline element")),
        p_nextline_element.context(lbl("nextline element")),
        p_inline_element.context(lbl("inline element")),
        p_plain_line_element.context(lbl("plain line")),
    ))
    .parse_next(input)
}

fn p_plain_line_element<'a>(input: &mut Input<'a>) -> PResult<Element<'a>> {
    let ((((), _), line), span) =
        repeat_till(1.., (not(line_ending), any), alt((line_ending, eof)))
            .with_taken()
            .with_span()
            .parse_next(input)?;
    Ok(Element::Plain(Sp::new(span, line)))
}

// TODO: try replacing this with using and_then for parsing the inner part, which would allow us to avoid having to provide the #} to the p_regular_tag_inner
fn p_regular_tag_inner<'a>(end: &'a str) -> impl winnow::Parser<Input<'a>, &'a str, ContextError> {
    trace("p_regular_tag_inner", move |i: &mut _| {
        repeat_till(0.., (not(line_ending), not(end), any), peek(end))
            .map(|(_, _): ((), _)| ())
            .context(lbl("tag-content"))
            .take()
            .parse_next(i)
    })
}

/// p_tag := <start> <p_inner> <end>
fn p_tag<'a, T>(
    start: &'a str,
    p_inner: impl winnow::Parser<Input<'a>, T, ContextError>,
    end: &'a str,
) -> impl winnow::Parser<Input<'a>, Sp<T>, ContextError> {
    (delimited(
        start,
        delimited(wss, p_inner.with_span(), wss),
        cut_err(literal(end)),
    ))
    .context(lbl("tag"))
    .map(|(s, span)| Sp::new(span, s))
}

fn p_tag_line<'a, T>(
    start: &'a str,
    p_inner: impl winnow::Parser<Input<'a>, T, ContextError>,
    end: &'a str,
) -> impl winnow::Parser<Input<'a>, (&'a str, Sp<T>, &'a str), ContextError> {
    let left_p = repeat_till(0.., (not(line_ending), not(start), any), peek(start))
        .map(|(_, _): ((), _)| ())
        .context(lbl("left-of-tag"))
        .take();
    let tag_p = p_tag(start, p_inner, end);
    (left_p, tag_p, till_line_ending)
}

fn p_tagged_line<'a>(
    start: &'a str,
    p_inner: impl winnow::Parser<Input<'a>, &'a str, ContextError>,
    end: &'a str,
) -> impl winnow::Parser<Input<'a>, (TaggedLine<'a>, Sp<&'a str>), ContextError> {
    p_tag_line(start, p_inner, end)
        .with_taken()
        .with_span()
        .map(|(((left, expr, right), line), span)| {
            (
                TaggedLine {
                    left,
                    right,
                    full_line: Sp::new(span, line),
                },
                expr,
            )
        })
}

fn p_nextline_element<'a>(input: &mut Input<'a>) -> PResult<Element<'a>> {
    let p_inner = (
        opt((literal("if"), wsp)).map(|x| x.is_some()),
        p_regular_tag_inner("#}"),
    );
    let ((left, expr, right), full_line) = p_tag_line("{#", p_inner, "#}")
        .with_taken()
        .with_span()
        .map(|((tag, line), span)| (tag, Sp::new(span, line)))
        .parse_next(input)?;
    line_ending.parse_next(input)?;
    let next_line = till_line_ending.parse_next(input)?;
    Ok(Element::NextLine {
        line: TaggedLine {
            left,
            right,
            full_line,
        },
        is_if: expr.content().0,
        expr: expr.map(|x| x.1),
        next_line,
    })
}

fn p_inline_element<'a>(input: &mut Input<'a>) -> PResult<Element<'a>> {
    let p_inner = (
        opt((literal("if"), wsp)).map(|x| x.is_some()),
        p_regular_tag_inner(">}"),
    )
        .context(lbl("tag-inner"));
    let ((left, expr, right), full_line) = p_tag_line("{<", p_inner, ">}")
        .with_taken()
        .with_span()
        .map(|((tag, line), span)| (tag, Sp::new(span, line)))
        .parse_next(input)?;

    Ok(Element::Inline {
        line: TaggedLine {
            left,
            right,
            full_line,
        },
        is_if: expr.content().0,
        expr: expr.map(|x| x.1),
    })
}

fn p_multiline_body<'a>(input: &mut Input<'a>) -> PResult<Vec<Element<'a>>> {
    trace(
        "p_multiline_body",
        repeat_till(
            0..,
            p_element,
            peek(p_tagged_line(
                "{%",
                alt((
                    "end",
                    "else",
                    preceded(("elif", wsp), p_regular_tag_inner("%}")),
                )),
                "%}",
            )),
        ),
    )
    .parse_next(input)
    .map(|(elements, _)| elements)
}

fn p_multiline_element<'a>(input: &mut Input<'a>) -> PResult<Element<'a>> {
    let ((left, expr, right), full_line) = p_tag_line("{%", p_regular_tag_inner("%}"), "%}")
        .with_taken()
        .with_span()
        .map(|((tag, line), span)| (tag, Sp::new(span, line)))
        .parse_next(input)?;
    line_ending.parse_next(input)?;
    let elements = p_multiline_body.parse_next(input)?;
    let (end, _end_expr) = p_tagged_line("{%", "end", "%}").parse_next(input)?;

    Ok(Element::MultiLine {
        block: Block {
            line: TaggedLine {
                left,
                right,
                full_line,
            },
            expr,
            body: elements,
        },
        end,
    })
}

fn p_multiline_block_starting_with<'a, Expr>(
    start_p: impl winnow::Parser<Input<'a>, (&'a str, Expr, &'a str), ContextError>,
) -> impl winnow::Parser<Input<'a>, Block<'a, Expr>, ContextError> {
    trace(
        "p_multiline_block_starting_with",
        (
            start_p
                .with_taken()
                .with_span()
                .map(|((tag, line), span)| (tag, Sp::new(span, line))),
            line_ending,
            p_multiline_body,
        )
            .map(|(((left, expr, right), full_line), _, body)| Block {
                line: TaggedLine {
                    left,
                    right,
                    full_line,
                },
                expr,
                body,
            }),
    )
}

fn p_conditional_element<'a>(input: &mut Input<'a>) -> PResult<Element<'a>> {
    let (if_body, elif_bodies, else_block, end_line): (_, Vec<_>, Option<Block<'a, _>>, _) = (
        p_multiline_block_starting_with::<Sp<&'a str>>(p_tag_line(
            "{%",
            preceded(("if", wsp), p_regular_tag_inner("%}")),
            "%}",
        ))
        .context(lbl("if block")),
        repeat(
            0..,
            p_multiline_block_starting_with::<Sp<&'a str>>(p_tag_line(
                "{%",
                preceded(("elif", wsp), p_regular_tag_inner("%}")),
                "%}",
            ))
            .context(lbl("elif block")),
        ),
        opt(p_multiline_block_starting_with::<()>(
            p_tag_line("{%", "else", "%}").map(|(l, _, r)| (l, (), r)),
        )
        .context(lbl("else block"))),
        terminated(p_tagged_line("{%", "end", "%}"), opt(line_ending)),
    )
        .context(lbl("conditional element"))
        .parse_next(input)?;

    let mut blocks = Vec::new();
    blocks.push(if_body);
    blocks.extend(elif_bodies.into_iter());
    Ok(Element::Conditional {
        blocks,
        else_block,
        end: end_line.0,
    })
}

fn wss(input: &mut Input<'_>) -> PResult<()> {
    winnow::ascii::space0.void().parse_next(input)
}

fn wsp(input: &mut Input<'_>) -> PResult<()> {
    winnow::ascii::space0.void().parse_next(input)
}

fn lbl(s: &'static str) -> StrContext {
    StrContext::Label(s)
}

#[cfg(test)]
fn new_input(s: &str) -> Input<'_> {
    use winnow::{stream::Recoverable, Located};

    Recoverable::new(Located::new(s))
}

#[cfg(test)]
mod test {
    use assert_matches::assert_matches;
    use testresult::TestResult;
    use winnow::Parser as _;

    use crate::templating::parser::winnow_p::{
        p_conditional_element, p_multiline_element, p_nextline_element, p_tagged_line, Element, Sp,
        TaggedLine,
    };

    use super::{new_input, p_inline_element, p_tag_line};

    #[test]
    fn test_inline_tag() -> TestResult {
        let input = new_input("foo /* {< test >} */");
        assert_matches!(
            p_inline_element.parse(input)?,
            Element::Inline {
                line,
                expr,
                is_if: false
            } =>{
                 assert_eq!(*line.full_line.content(), "foo /* {< test >} */");
                 assert_eq!(line.left, "foo /* ");
                 assert_eq!(line.right, " */");
                 assert_eq!(*expr.content(), " test ");
            }
        );
        Ok(())
    }
    #[test]
    fn test_nextline_tag() -> TestResult {
        let mut input = new_input("/* {# test #} */\nfoo");
        assert_matches!(
            p_nextline_element(&mut input)?,
            Element::NextLine {
                line,
                expr,
                is_if: false,
                next_line: "foo"
            } =>{
                 assert_eq!(*line.full_line.content(), "/* {# test #} */");
                 assert_eq!(line.left, "/* ");
                 assert_eq!(line.right, " */");
                 assert_eq!(*expr.content(), " test ");
            }
        );
        Ok(())
    }
    #[test]
    fn test_parse_end() -> TestResult {
        let input = new_input("a{% end %}b");
        assert_matches!(p_tag_line("{%", "end", "%}").parse(input.clone())?,
            ("a", expr, "b") =>{
                 assert_eq!(*expr.content(), "end");
            }
        );
        assert_matches!(p_tagged_line("{%", "end", "%}").parse(input.clone())?,
            (TaggedLine { left: "a", right: "b", full_line }, expr) =>{
                assert_eq!(*full_line.content(), "a{% end %}b");
                 assert_eq!(*expr.content(), "end");
            }
        );
        Ok(())
    }

    #[test]
    fn test_multiline_block() -> TestResult {
        let mut input = new_input("/* {% test %} */\nfoo\n/*{% end %}*/");
        assert_matches!(
            p_multiline_element(&mut input)?,
            Element::MultiLine { block, end } =>{
                 assert_eq!(*block.line.full_line.content(), "/* {% test %} */");
                 assert_eq!(block.line.left, "/* ");
                 assert_eq!(block.line.right, " */");
                 assert_eq!(*block.expr.content(), "test ");
                 assert_eq!(*end.full_line.content(), "/* {% test %} */");
            }
        );
        Ok(())
    }

    #[test]
    fn test_conditional() -> TestResult {
        let mut input = new_input(indoc::indoc! {r#"
            // {% if foo %}
            thing
            thang
            // {% elif bar %}
            // {% elif baz %}
            // {% else %}
            stuff
            // {% end %}
        "#});
        assert_matches!(
            p_conditional_element(&mut input)?,
            Element::Conditional { blocks, else_block: Some(els), end } =>{
                 assert_eq!(*blocks[0].line.full_line.content(), "// {% if foo %}");
                 assert_eq!(*blocks[0].expr.content(), "foo ");
                 assert_matches!(blocks[0].body[0], Element::Plain(Sp{ content: "thing\n", .. }));
                 assert_matches!(blocks[0].body[1], Element::Plain(Sp{ content: "thang\n", .. }));
                 assert_eq!(*blocks[1].line.full_line.content(), "// {% elif bar %}");
                 assert_eq!(*blocks[2].line.full_line.content(), "// {% elif baz %}");
                 assert_eq!(*els.line.full_line.content(), "// {% else %}");
                 assert_eq!(*end.full_line.content(), "// {% end %}");
            }
        );
        Ok(())
    }

    #[test]
    fn test_nested_conditional() -> TestResult {
        let mut input = new_input(indoc::indoc! {r#"
            // {% if foo %}
            // {% if foo %}
            // {% end %}
            // {% end %}
        "#});
        assert_matches!(
            p_conditional_element(&mut input)?,
            Element::Conditional { blocks, else_block: None, .. } =>{
                 assert_eq!(*blocks[0].line.full_line.content(), "// {% if foo %}");
                 assert_eq!(*blocks[0].expr.content(), "foo ");
                 assert_matches!(blocks[0].body[0], Element::Conditional {..});
            }
        );
        Ok(())
    }
}
