use std::ops::Range;

use winnow::{
    ascii::{line_ending, till_line_ending},
    combinator::{
        alt, cut_err, delimited, eof, not, opt, peek, preceded, repeat, repeat_till, terminated,
        trace,
    },
    error::{ContextError, StrContext},
    token::{any, literal},
    Located, Parser,
};

use super::element::{Block, Element};

pub mod comment_style;

type Input<'a> = winnow::Located<&'a str>;
// type Input<'a> = winnow::stream::Recoverable<winnow::Located<&'a str>, ContextError>;
type PResult<T> = winnow::PResult<T, ContextError>;

#[derive(Debug, Eq, PartialEq)]
pub struct Sp<T> {
    range: Range<usize>,
    content: T,
}

impl<'a> Sp<&'a str> {
    pub fn as_str(&self) -> &'a str {
        self.content
    }
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
    pub tag: &'a str,
    pub right: &'a str,
    pub full_line: Sp<&'a str>,
}

/// This just generates a single report, looking at the first error.
/// This is imperfect...
/// TODO: make this better
fn report_from_err(
    err: winnow::error::ParseError<Input<'_>, ContextError>,
    input: &str,
) -> miette::Report {
    let message = err.inner().to_string();
    let input = input.to_owned();
    let start = err.offset();
    // Assume the error span is only for the first `char`.
    // Semantic errors are free to choose the entire span returned by `Parser::with_span`.
    let end = (start + 1..)
        .find(|e| input.is_char_boundary(*e))
        .unwrap_or(start);

    // lmao this is garbage, especially the help thingy
    miette::MietteDiagnostic::new(message)
        .with_labels(vec![miette::LabeledSpan::at(start..end, "here")])
        .with_help(
            err.inner()
                .context()
                .next()
                .map(|x| x.to_string())
                .unwrap_or_default(),
        )
        .into()
}

pub fn parse_document<'a>(s: &'a str) -> miette::Result<Vec<Element<'a>>> {
    terminated(
        repeat(0.., p_element),
        repeat(0.., line_ending).map(|_: ()| ()),
    )
    .parse(Located::new(s))
    .map_err(|e| report_from_err(e, s))
}

#[allow(unused)]
pub fn parse_element<'a>(s: &'a str) -> miette::Result<Element<'a>> {
    terminated(p_element, repeat(0.., line_ending).map(|_: ()| ()))
        .parse(Located::new(s))
        .map_err(|e| report_from_err(e, s))
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
    let ((((), _), line), span) = repeat_till(1.., any, alt((line_ending, eof)))
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

/// p_tag_line := <start> <p_inner> <right>
/// returns left, result-of-p_inner, tag, right
fn p_tag_line<'a, T>(
    start: &'a str,
    p_inner: impl winnow::Parser<Input<'a>, T, ContextError>,
    end: &'a str,
) -> impl winnow::Parser<Input<'a>, (&'a str, Sp<T>, &'a str, &'a str), ContextError> {
    let left_p = repeat_till(0.., (not(line_ending), not(start), any), peek(start))
        .map(|(_, _): ((), _)| ())
        .context(lbl("left of tag"))
        .take();
    let tag_p = p_tag(start, p_inner, end).with_taken();
    (left_p, tag_p, (till_line_ending, opt(line_ending)).take()).map(|(l, t, r)| (l, t.0, t.1, r))
}

fn p_tagged_line<'a>(
    start: &'a str,
    p_inner: impl winnow::Parser<Input<'a>, &'a str, ContextError>,
    end: &'a str,
) -> impl winnow::Parser<Input<'a>, (TaggedLine<'a>, Sp<&'a str>), ContextError> {
    p_tag_line(start, p_inner, end)
        .with_taken()
        .with_span()
        .map(|(((left, expr, tag, right), line), span)| {
            (
                TaggedLine {
                    left,
                    tag,
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
    let ((left, expr, tag, right), full_line) = p_tag_line("{#", p_inner, "#}")
        .with_taken()
        .with_span()
        .map(|((tag, line), span)| (tag, Sp::new(span, line)))
        .context(StrContext::Expected("line with a next-line tag".into()))
        .parse_next(input)?;
    let next_line = till_line_ending
        .context(StrContext::Expected("Another line".into()))
        .parse_next(input)?;
    Ok(Element::NextLine {
        line: TaggedLine {
            left,
            tag,
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
    let ((left, expr, tag, right), full_line) = p_tag_line("{<", p_inner, ">}")
        .with_taken()
        .with_span()
        .map(|((tag, line), span)| (tag, Sp::new(span, line)))
        .parse_next(input)?;

    Ok(Element::Inline {
        line: TaggedLine {
            left,
            tag,
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
    let ((left, expr, tag, right), full_line) = p_tag_line("{%", p_regular_tag_inner("%}"), "%}")
        .with_taken()
        .with_span()
        .map(|((tag, line), span)| (tag, Sp::new(span, line)))
        .parse_next(input)?;
    let elements = p_multiline_body.parse_next(input)?;
    let (end, _end_expr) = p_tagged_line("{%", "end", "%}").parse_next(input)?;

    Ok(Element::MultiLine {
        block: Block {
            line: TaggedLine {
                left,
                tag,
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
    start_p: impl winnow::Parser<Input<'a>, (&'a str, Expr, &'a str, &'a str), ContextError>,
) -> impl winnow::Parser<Input<'a>, Block<'a, Expr>, ContextError> {
    let p = (
        start_p
            .with_taken()
            .with_span()
            .map(|((tag, line), span)| (tag, Sp::new(span, line))),
        p_multiline_body,
    );
    let p = p.map(|(((left, expr, tag, right), full_line), body)| Block {
        line: TaggedLine {
            left,
            tag,
            right,
            full_line,
        },
        expr,
        body,
    });
    trace("p_multiline_block_starting_with", p)
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
            p_tag_line("{%", "else", "%}").map(|(l, _, t, r)| (l, (), t, r)),
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
    use winnow::Located;
    Located::new(s)
    // Recoverable::new(Located::new(s))
}

#[cfg(test)]
mod test {
    use assert_matches::assert_matches;
    use testresult::TestResult;
    use winnow::Parser as _;

    use crate::templating::parser::{
        p_conditional_element, p_multiline_element, p_nextline_element, p_tagged_line,
        parse_document, Element, Sp, TaggedLine,
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
                 assert_eq!(*expr.content(), "test ");
            }
        );
        Ok(())
    }

    #[test]
    fn test_nextline_tag() -> TestResult {
        let mut input = new_input("/* {# x #} */\nfoo");
        assert_matches!(
            p_nextline_element(&mut input)?,
            Element::NextLine {
                line,
                expr,
                is_if: false,
                next_line: "foo"
            } =>{
                 assert_eq!(*line.full_line.content(), "/* {# x #} */\n");
                 assert_eq!(line.left, "/* ");
                 assert_eq!(line.right, " */\n");
                 assert_eq!(*expr.content(), "x ");
            }
        );
        Ok(())
    }
    #[test]
    fn test_parse_end() -> TestResult {
        let input = new_input("a{% end %}b");
        assert_matches!(p_tag_line("{%", "end", "%}").parse(input.clone())?,
            ("a", expr, _, "b") =>{
                 assert_eq!(*expr.content(), "end");
            }
        );
        assert_matches!(p_tagged_line("{%", "end", "%}").parse(input.clone())?,
            (TaggedLine { left: "a", right: "b", tag: "{% end %}", full_line }, expr) =>{
                assert_eq!(*full_line.content(), "a{% end %}b");
                 assert_eq!(*expr.content(), "end");
            }
        );
        Ok(())
    }

    #[test]
    fn test_multiline_block() -> TestResult {
        let mut input = new_input("/* {% test %} */\nfoo\n/* {% end %} */");
        assert_matches!(
            p_multiline_element(&mut input)?,
            Element::MultiLine { block, end } =>{
                 assert_eq!(*block.line.full_line.content(), "/* {% test %} */\n");
                 assert_eq!(block.line.left, "/* ");
                 assert_eq!(block.line.right, " */\n");
                 assert_eq!(*block.expr.content(), "test ");
                 assert_eq!(*end.full_line.content(), "/* {% end %} */");
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
                 assert_eq!(*blocks[0].line.full_line.content(), "// {% if foo %}\n");
                 assert_eq!(*blocks[0].expr.content(), "foo ");
                 assert_matches!(blocks[0].body[0], Element::Plain(Sp{ content: "thing\n", .. }));
                 assert_matches!(blocks[0].body[1], Element::Plain(Sp{ content: "thang\n", .. }));
                 assert_eq!(*blocks[1].line.full_line.content(), "// {% elif bar %}\n");
                 assert_eq!(*blocks[2].line.full_line.content(), "// {% elif baz %}\n");
                 assert_eq!(*els.line.full_line.content(), "// {% else %}\n");
                 assert_eq!(*end.full_line.content(), "// {% end %}\n");
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
                 assert_eq!(*blocks[0].line.full_line.content(), "// {% if foo %}\n");
                 assert_eq!(*blocks[0].expr.content(), "foo ");
                 assert_matches!(blocks[0].body[0], Element::Conditional {..});
            }
        );
        Ok(())
    }

    #[test]
    fn test_nextline_tag_document() -> TestResult {
        let input = indoc::indoc! {r#"
            # {# replace(`'.*'`, `'{data.value}'`) #}
            value = 'foo'
        "#};
        let mut input = new_input(input);
        let result = parse_document(&mut input)?;
        assert_matches!(
            &result[0],
            Element::NextLine {
                line,
                expr,
                is_if: false,
                next_line: "value = 'foo'"
            } =>{
                 assert_eq!(*line.full_line.content(), "# {# replace(`'.*'`, `'{data.value}'`) #}\n");
                 assert_eq!(line.left, "# ");
                 assert_eq!(line.right, "\n");
                 assert_eq!(*expr.content(), "replace(`'.*'`, `'{data.value}'`) ");
            }
        );
        Ok(())
    }
}
