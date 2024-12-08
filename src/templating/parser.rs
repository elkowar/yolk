use std::ops::Range;

#[cfg(test)]
use testresult::TestResult;
use winnow::{
    ascii::{line_ending, till_line_ending},
    combinator::{
        alt, cut_err, delimited, eof, fail, not, opt, peek, preceded, repeat, repeat_till,
        terminated, trace,
    },
    error::StrContext,
    stream::{Location, Recoverable, Stream},
    token::{any, literal},
    Located, Parser, RecoverableParser,
};

use super::{
    element::{Block, Element},
    parse_error::{YolkParseError, YolkParseFailure},
};

// type Input<'a> = winnow::Located<&'a str>;
type Input<'a> = Recoverable<Located<&'a str>, YolkParseError>;
type PResult<T> = winnow::PResult<T, YolkParseError>;

#[derive(Eq, PartialEq)]
pub struct Sp<T> {
    range: Range<usize>,
    content: T,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Sp<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}]{:?}", self.range, self.content)
    }
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

    #[allow(unused)]
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

pub fn parse_document<'a>(s: &'a str) -> Result<Vec<Element<'a>>, YolkParseFailure> {
    let p = repeat(0.., p_element);
    try_parse(p, s)
}

#[allow(unused)]
pub fn parse_element<'a>(s: &'a str) -> Result<Element<'a>, YolkParseFailure> {
    let p = terminated(p_element, repeat(0.., line_ending).map(|_: ()| ()));
    try_parse(p, s)
}

pub fn try_parse<'a, P: Parser<Input<'a>, T, YolkParseError>, T>(
    mut parser: P,
    input: &'a str,
) -> Result<T, YolkParseFailure> {
    let (_, maybe_val, errs) = parser.recoverable_parse(Located::new(input));
    if let (Some(v), true) = (maybe_val, errs.is_empty()) {
        Ok(v)
    } else {
        Err(YolkParseFailure::from_errs(errs, input))
    }
}

fn p_element<'a>(input: &mut Input<'a>) -> PResult<Element<'a>> {
    trace("peek any", peek(any)).parse_next(input)?;
    Ok(alt((
        p_inline_element.context(lbl("inline element")),
        p_nextline_element.context(lbl("nextline element")),
        p_conditional_element.context(lbl("conditional element")),
        p_multiline_element.context(lbl("multiline element")),
        p_plain_line_element.context(lbl("plain line")),
        fail.context(lbl("valid element")),
    ))
    // .resume_after(
    //     repeat_till(1.., (not(line_ending), any), line_ending)
    //         .map(|((), _)| ())
    //         .void(),
    // )
    .parse_next(input)?)
    // .unwrap_or_else(|| Element::Plain(Sp::new(0..0, ""))))
}

fn p_plain_line_element<'a>(input: &mut Input<'a>) -> PResult<Element<'a>> {
    peek(any).parse_next(input)?;
    alt((
        line_ending.take_span(),
        (p_text_segment, alt((line_ending, eof))).take_span(),
    ))
    .map(Element::Plain)
    .parse_next(input)
}

/// Returns (terminator, entire text segment before terminator)
fn p_text_segment<'a>(input: &mut Input<'a>) -> PResult<(&'a str, &'a str)> {
    repeat_till(
        1..,
        (not(p_any_tag_start), not(line_ending), any),
        peek(alt((p_any_tag_start, line_ending, eof))),
    )
    .map(|((), terminator)| terminator)
    .with_taken()
    .parse_next(input)
}

#[cfg(test)]
#[test]
fn test_p_text_segment() -> TestResult {
    insta::assert_debug_snapshot!(p_text_segment.parse_peek(new_input("foo {% bar %} baz"))?);
    insta::assert_debug_snapshot!(p_text_segment.parse_peek(new_input("foo"))?);
    insta::assert_debug_snapshot!(p_text_segment.parse_peek(new_input("{< bar >}")));
    Ok(())
}

fn p_regular_tag_inner<'a>(
    end: &'a str,
) -> impl winnow::Parser<Input<'a>, &'a str, YolkParseError> {
    trace("p_regular_tag_inner", move |i: &mut _| {
        repeat_till(1.., (not(line_ending), not(end), any), peek(end))
            .map(|(_, _): ((), _)| ())
            .context(lbl("expression"))
            .take()
            .parse_next(i)
    })
}

/// p_tag := <start> <p_inner> <end>
fn p_tag<'a, T>(
    start: impl winnow::Parser<Input<'a>, &'a str, YolkParseError>,
    p_inner: impl winnow::Parser<Input<'a>, T, YolkParseError>,
    end: impl winnow::Parser<Input<'a>, &'a str, YolkParseError>,
) -> impl winnow::Parser<Input<'a>, Sp<T>, YolkParseError> {
    (delimited(
        start,
        delimited(wss, p_inner.with_span(), wss),
        cut_err(end),
    ))
    .context(lbl("tag"))
    .map(|(s, span)| Sp::new(span, s))
}

fn p_any_tag_start<'a>(input: &mut Input<'a>) -> PResult<&'a str> {
    alt((literal("{%"), literal("{#"), literal("{<"))).parse_next(input)
}

/// p_tag_line := <start> <p_inner> <right> (\n)?
/// returns left, result-of-p_inner, tag, right
fn p_tag_line<'a, T>(
    start: impl winnow::Parser<Input<'a>, &'a str, YolkParseError> + Copy,
    p_inner: impl winnow::Parser<Input<'a>, T, YolkParseError>,
    end: impl winnow::Parser<Input<'a>, &'a str, YolkParseError>,
    require_newline: bool,
) -> impl winnow::Parser<Input<'a>, (TaggedLine<'a>, Sp<T>), YolkParseError> {
    let left_p = repeat_till(
        0..,
        (not(line_ending), not(p_any_tag_start), any),
        peek(start),
    )
    .map(|((), _)| ())
    .take();
    let tag_p = p_tag(start, p_inner, end).with_taken();
    let line_end: Box<dyn Parser<_, _, _>> = match require_newline {
        true => Box::new(line_ending.map(Some)),
        false => Box::new(opt(line_ending)),
    };
    let right_p = cut_err((till_line_ending, line_end.context(lbl("newline"))).take());
    (left_p, tag_p, right_p)
        .with_spanned()
        .map(|((left, (inner_res, tag), right), full_line)| {
            (
                TaggedLine {
                    left,
                    tag,
                    right,
                    full_line,
                },
                inner_res,
            )
        })
}

fn p_nextline_element<'a>(input: &mut Input<'a>) -> PResult<Element<'a>> {
    peek(any).parse_next(input)?;
    let p_inner = (
        opt((literal("if"), wsp)).map(|x| x.is_some()),
        p_regular_tag_inner("#}"),
    );
    let (tagged_line, expr) = p_tag_line("{#", p_inner, "#}", true)
        .context("line with a next-line tag")
        .parse_next(input)?;
    let next_line = till_line_ending.context("Another line").parse_next(input)?;
    Ok(Element::NextLine {
        tagged_line,
        is_if: expr.content().0,
        expr: expr.map(|x| x.1),
        next_line,
    })
}

fn p_inline_element<'a>(input: &mut Input<'a>) -> PResult<Element<'a>> {
    peek(any).parse_next(input)?;
    let p_inner = (
        opt((literal("if"), wsp)).map(|x| x.is_some()),
        p_regular_tag_inner(">}"),
    )
        .context(lbl("tag-inner"));
    let (tagged_line, expr) = p_tag_line("{<", cut_err(p_inner), ">}", false).parse_next(input)?;
    Ok(Element::Inline {
        line: tagged_line,
        is_if: expr.content().0,
        expr: expr.map(|x| x.1),
    })
}

fn p_multiline_body<'a>(
    p_end_tag_inner: impl winnow::Parser<Input<'a>, &'a str, YolkParseError>,
) -> impl Parser<Input<'a>, Vec<Element<'a>>, YolkParseError> {
    let end_tag_line = peek(p_tag_line("{%", p_end_tag_inner, "%}", false)).context("end of block");
    repeat_till(0.., p_element, end_tag_line)
        .context(lbl("end of block"))
        .map(|(elements, _)| elements)
        .resume_after(
            (repeat_till(0.., any, peek(p_tag_line("{%", "end", "%}", false))))
                .map(|((), _)| ())
                .void(),
        )
        .map(|x| x.unwrap_or_default())
}

fn p_multiline_element<'a>(input: &mut Input<'a>) -> PResult<Element<'a>> {
    peek(any).parse_next(input)?;
    let (tagged_line, expr) = p_tag_line(
        "{%",
        // p_regular_tag_inner("%}"),
        // TODO: does this actually help?
        preceded(
            not(alt(("if", "elif", "else", "end"))),
            p_regular_tag_inner("%}"),
        ),
        "%}",
        true,
    )
    .parse_next(input)?;
    let body = cut_err(p_multiline_body("end")).parse_next(input)?;
    let (end, _) =
        cut_err(p_tag_line("{%", "end", "%}", false).context("end tag")).parse_next(input)?;

    Ok(Element::MultiLine {
        block: Block {
            tagged_line,
            expr,
            body,
        },
        end,
    })
}

fn p_multiline_block_starting_with<'a, Expr>(
    start_p: impl winnow::Parser<Input<'a>, (TaggedLine<'a>, Expr), YolkParseError>,
    block_p: impl winnow::Parser<Input<'a>, Vec<Element<'a>>, YolkParseError>,
) -> impl winnow::Parser<Input<'a>, Block<'a, Expr>, YolkParseError> {
    let p = (start_p, cut_err(block_p));
    let p = p.map(|((tagged_line, expr), body)| Block {
        tagged_line,
        expr,
        body,
    });
    trace("p_multiline_block_starting_with", p)
}

fn p_conditional_element<'a>(input: &mut Input<'a>) -> PResult<Element<'a>> {
    peek(any).parse_next(input)?;
    let p_if = p_multiline_block_starting_with::<Sp<&'a str>>(
        p_tag_line(
            "{%",
            preceded(("if", wsp), p_regular_tag_inner("%}")),
            "%}",
            true,
        ),
        p_multiline_body(alt((
            "end",
            "else",
            preceded(("elif", wsp), p_regular_tag_inner("%}")),
        ))),
    );
    let p_elif = p_multiline_block_starting_with::<Sp<&'a str>>(
        p_tag_line(
            "{%",
            preceded(("elif", wsp), p_regular_tag_inner("%}")),
            "%}",
            true,
        ),
        p_multiline_body(alt((
            "end",
            "else",
            preceded(("elif", wsp), p_regular_tag_inner("%}")),
        ))),
    );
    let p_else = p_multiline_block_starting_with(
        p_tag_line("{%", "else".void(), "%}", true).context("else tag"),
        p_multiline_body("end"),
    );
    let p_end = terminated(p_tag_line("{%", "end", "%}", false), opt(line_ending));
    let (if_body, elif_bodies, else_block, end_line): (_, Vec<_>, Option<Block<'a, _>>, _) = (
        p_if.context(lbl("if block")),
        cut_err(repeat(0.., p_elif.context(lbl("elif block")))),
        cut_err(opt(p_else.context(lbl("else block")))),
        cut_err(p_end.context(lbl("end tag"))),
    )
        .parse_next(input)?;

    let mut blocks = Vec::new();
    blocks.push(if_body);
    blocks.extend(elif_bodies.into_iter());
    Ok(Element::Conditional {
        blocks,
        else_block: else_block.map(|x| x.map_expr(|_| ())),
        end: end_line.0,
    })
}

fn wss(input: &mut Input<'_>) -> PResult<()> {
    winnow::ascii::space0.void().parse_next(input)
}

fn wsp(input: &mut Input<'_>) -> PResult<()> {
    winnow::ascii::space0.void().parse_next(input)
}

/// Create a context string
fn lbl(s: &'static str) -> &'static str {
    s
    // StrContext::Label(s)
}
/// Create a [`StrContext::Expected`] with a [`winnow::error::StrContextValue::Description`].
#[allow(unused)]
fn exp(s: &'static str) -> StrContext {
    StrContext::Expected(winnow::error::StrContextValue::Description(s))
}

#[cfg(test)]
fn new_input(s: &str) -> Input<'_> {
    use winnow::{stream::Recoverable, Located};
    // Located::new(s)
    Recoverable::new(Located::new(s))
}

#[allow(unused)]
pub trait ParserExt<I: Stream + Location, O, E>: winnow::Parser<I, O, E> + Sized {
    fn with_spanned(self) -> impl Parser<I, (O, Sp<I::Slice>), E> {
        self.with_span()
            .with_taken()
            .map(|((o, span), text)| (o, Sp::new(span, text)))
    }
    fn spanned(self) -> impl Parser<I, Sp<O>, E> {
        self.with_span().map(|(o, span)| (Sp::new(span, o)))
    }
    fn take_span(self) -> impl Parser<I, Sp<I::Slice>, E> {
        self.take().with_span().map(|(o, span)| (Sp::new(span, o)))
    }
}

impl<T: Parser<I, O, E> + Sized, I: Stream + Location, O, E> ParserExt<I, O, E> for T {}

#[cfg(test)]
mod test {
    use assert_matches::assert_matches;
    use insta::assert_debug_snapshot;
    use miette::GraphicalReportHandler;
    use testresult::TestResult;
    use winnow::Parser as _;

    use crate::templating::parser::{
        p_conditional_element, p_multiline_element, p_nextline_element, parse_document, Element,
        Sp, TaggedLine,
    };

    use super::{new_input, p_inline_element, p_tag_line};

    fn render_error(e: impl miette::Diagnostic) -> String {
        let mut out = String::new();
        GraphicalReportHandler::new()
            .with_theme(miette::GraphicalTheme::unicode_nocolor())
            .render_report(&mut out, &e)
            .unwrap();
        out
    }

    #[test]
    fn test_inline_tag() -> TestResult {
        insta::assert_debug_snapshot!(p_inline_element.parse(new_input("foo /* {< test >} */"))?);
        Ok(())
    }

    #[test]
    fn test_nextline_tag() -> TestResult {
        insta::assert_debug_snapshot!(p_nextline_element(&mut new_input("/* {# x #} */\nfoo"))?);
        Ok(())
    }
    #[test]
    fn test_parse_end() -> TestResult {
        assert_debug_snapshot!(p_tag_line("{%", "end", "%}", false).parse(new_input("a{% end %}b")));
        Ok(())
    }

    #[test]
    fn test_multiline_block() -> TestResult {
        assert_debug_snapshot!(p_multiline_element(&mut new_input(
            "/* {% test %} */\nfoo\n/* {% end %} */"
        ))?);
        Ok(())
    }

    #[test]
    fn test_conditional() {
        assert_debug_snapshot!(p_conditional_element(&mut new_input(indoc::indoc! {r#"
            // {% if a %}
            a
            b
            // {% elif b %}
            // {% elif c %}
            // {% else %}
            c
            // {% end %}
        "#})));
    }

    #[test]
    fn test_nested_conditional() {
        assert_debug_snapshot!(p_conditional_element(&mut new_input(indoc::indoc! {r#"
            // {% if foo %}
            // {% if foo %}
            // {% end %}
            // {% end %}
        "#})));
    }

    #[test]
    fn test_nextline_tag_document() {
        insta::assert_debug_snapshot!(parse_document(&mut new_input(indoc::indoc! {r#"
            # {# replace(`'.*'`, `'{data.value}'`) #}
            value = 'foo'
        "#})));
    }

    #[test]
    fn test_error_incomplete_nextline() {
        insta::assert_snapshot!(render_error(parse_document("{#f#}").unwrap_err()));
    }
    #[test]
    fn test_error_incomplete_multiline() {
        insta::assert_snapshot!(render_error(parse_document("{%f%}").unwrap_err()));
    }
    #[test]
    fn test_error_multiline_with_else() {
        insta::assert_snapshot!(render_error(
            parse_document("{%f%}\n{%else%}\n{%end%}").unwrap_err()
        ));
    }
    #[test]
    fn test_error_if_without_expression() {
        insta::assert_snapshot!(render_error(parse_document("{<if>}").unwrap_err()));
    }
}
