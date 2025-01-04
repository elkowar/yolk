//! The parser for yolk templates
//!
//! A lot of the structure, especially with regards to error handling, is heavily inspired by the v2_parser in <https://github.com/kdl-org/kdl-rs>.

use std::ops::Range;

use winnow::{
    ascii::{line_ending, till_line_ending},
    combinator::{
        alt, cut_err, delimited, eof, fail, not, opt, peek, preceded, repeat, repeat_till,
        terminated, trace,
    },
    stream::{Location, Recoverable, Stream},
    token::{any, literal},
    Located, Parser, RecoverableParser,
};

use super::{
    element::{Block, Element, TaggedLine},
    error::{cx, YolkParseError, YolkParseFailure},
};

// type Input<'a> = winnow::Located<&'a str>;
type Input<'a> = Recoverable<Located<&'a str>, YolkParseError>;
type PResult<T> = winnow::PResult<T, YolkParseError>;

#[derive(Eq, PartialEq, arbitrary::Arbitrary)]
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

impl<T> Sp<T> {
    pub fn new(range: Range<usize>, content: T) -> Self {
        Self { range, content }
    }

    pub fn content(&self) -> &T {
        &self.content
    }

    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Sp<U> {
        Sp::new(self.range, f(self.content))
    }
}

pub fn parse_document(s: &str) -> Result<Vec<Element<'_>>, YolkParseFailure> {
    let p = repeat(0.., p_element);
    try_parse(p, s)
}

#[allow(unused)]
pub fn parse_element(s: &str) -> Result<Element<'_>, YolkParseFailure> {
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
    alt((
        p_inline_element,
        p_nextline_element,
        p_conditional_element,
        p_multiline_element,
        p_plain_line_element,
        fail.context(cx().msg("Encountered invalid element").lbl("element")),
    ))
    .parse_next(input)
}

fn p_plain_line_element<'a>(input: &mut Input<'a>) -> PResult<Element<'a>> {
    let line_content_p = repeat_till(
        1..,
        (not(alt((p_any_tag_start, line_ending))), any),
        peek(alt((line_ending, eof))),
    )
    .map(|((), terminator)| terminator)
    .with_taken();

    peek(any).parse_next(input)?;
    repeat(
        1..,
        alt((
            line_ending,
            (line_content_p, alt((line_ending, eof))).take(),
        )),
    )
    .map(|()| ())
    .take_span()
    .map(Element::Plain)
    .parse_next(input)
}

fn p_regular_tag_inner(end: &str) -> impl winnow::Parser<Input<'_>, &'_ str, YolkParseError> {
    let p = repeat_till(1.., (not(end), any), peek(end)).map(|(_, _): ((), _)| ());
    cut_err(
        p.context(cx().msg("Failed to parse expression").lbl("expression"))
            .take(),
    )
}

/// p_tag := <start> <p_inner> <end>
fn p_tag<'a, T>(
    start: impl winnow::Parser<Input<'a>, &'a str, YolkParseError>,
    p_inner: impl winnow::Parser<Input<'a>, T, YolkParseError>,
    end: impl winnow::Parser<Input<'a>, &'a str, YolkParseError>,
) -> impl winnow::Parser<Input<'a>, Sp<T>, YolkParseError> {
    (delimited(
        start,
        delimited(wsp0_or_newline, p_inner.with_span(), wsp0_or_newline),
        cut_err(end),
    ))
    .context(cx().msg("Failed to parse tag").lbl("tag"))
    .map(|(s, span)| Sp::new(span, s))
}

fn p_any_tag_start<'a>(input: &mut Input<'a>) -> PResult<&'a str> {
    alt((literal("{%"), literal("{#"), literal("{<"))).parse_next(input)
}

/// p_tag_line := <start> <p_inner> <right> (\n)?
/// returns the [`TaggedLine`] and the result of the `p_inner` parser.
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
    let right_p = cut_err(
        (
            till_line_ending,
            line_end.context(cx().msg("Expected newline")),
        )
            .take(),
    );
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
        opt((literal("if"), wsp1)).map(|x| x.is_some()),
        p_regular_tag_inner("#}"),
    );
    let (tagged_line, expr) = p_tag_line("{#", p_inner, "#}", true).parse_next(input)?;
    let next_line = till_line_ending.spanned().parse_next(input)?;
    Ok(Element::NextLine {
        tagged_line,
        is_if: expr.content().0,
        expr: expr.map(|x| x.1),
        next_line,
    })
}

/// Parses an inline element, including the surrounding line
fn p_inline_element<'a>(input: &mut Input<'a>) -> PResult<Element<'a>> {
    peek(any).parse_next(input)?;
    let p_inner = (
        opt((literal("if"), wsp1)).map(|x| x.is_some()),
        p_regular_tag_inner(">}"),
    )
        .context(cx().msg("Failed to parse tag inner").lbl("here"));
    let (tagged_line, expr) = p_tag_line("{<", cut_err(p_inner), ">}", false).parse_next(input)?;
    Ok(Element::Inline {
        line: tagged_line,
        is_if: expr.content().0,
        expr: expr.map(|x| x.1),
    })
}

/// Parses 0..n elements until the end tag line, but does not consume the end tag line.
///
/// `p_end_tag_inner` is used as the inner parser for the [`p_tag_line`] parser of the end tag,
/// which allows you to specify the specific set of end tags that should be allowed.
fn p_multiline_body<'a>(
    p_end_tag_inner: impl winnow::Parser<Input<'a>, &'a str, YolkParseError>,
) -> impl Parser<Input<'a>, Vec<Element<'a>>, YolkParseError> {
    let end_tag_line = peek(p_tag_line("{%", p_end_tag_inner, "%}", false));
    repeat_till(0.., p_element, end_tag_line)
        .context(
            cx().msg("Expected block to end here")
                .lbl("block end")
                .hlp("Did you forget an `{% end %}` tag?"),
        )
        .map(|(elements, _)| elements)
}

/// Parses a regular multiline block start tag (no if, elif, else, end), then parses a [`p_multiline_body`], then a regular end tag.
fn p_multiline_element<'a>(input: &mut Input<'a>) -> PResult<Element<'a>> {
    peek(any).parse_next(input)?;
    let (tagged_line, expr) = p_tag_line(
        "{%",
        preceded(
            not(alt(("if", "elif", "else", "end"))),
            p_regular_tag_inner("%}"),
        ),
        "%}",
        true,
    )
    .parse_next(input)?;
    let body = cut_err(p_multiline_body("end")).parse_next(input)?;
    let (end, _) = cut_err(
        p_tag_line("{%", "end", "%}", false).context(
            cx().msg("Expected block to end here")
                .lbl("block end")
                .hlp("Did you forget an `{% end %}` tag?"),
        ),
    )
    .parse_next(input)?;

    Ok(Element::MultiLine {
        block: Block {
            tagged_line,
            expr,
            body,
        },
        end,
    })
}

/// Parse a multiline block starting with a tag line using the given parser into a [`Block`].
fn p_block<'a, Expr>(
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
    let p_if = p_block::<Sp<&'a str>>(
        p_tag_line(
            "{%",
            preceded(("if", wsp1), p_regular_tag_inner("%}")),
            "%}",
            true,
        ),
        p_multiline_body(alt((
            "end",
            "else",
            preceded(("elif", wsp1), p_regular_tag_inner("%}")),
        ))),
    );
    let p_elif = p_block::<Sp<&'a str>>(
        p_tag_line(
            "{%",
            preceded(("elif", wsp1), p_regular_tag_inner("%}")),
            "%}",
            true,
        ),
        p_multiline_body(alt((
            "end",
            "else",
            preceded(("elif", wsp1), p_regular_tag_inner("%}")),
        ))),
    );
    let p_else = p_block(
        p_tag_line("{%", "else".void(), "%}", true)
            .context(cx().msg("Failed to parse else tag").lbl("else tag")),
        p_multiline_body("end"),
    );
    let p_end = p_tag_line("{%", "end", "%}", false);
    let (if_body, elif_bodies, else_block, end_line): (_, Vec<_>, Option<Block<'a, _>>, _) = (
        p_if.context(cx().msg("Failed to parse if block").lbl("if block")),
        cut_err(repeat(
            0..,
            p_elif.context(cx().msg("Failed to parse elif block").lbl("elif block")),
        )),
        cut_err(opt(p_else.context(
            cx().msg("Failed to parse else block").lbl("else block"),
        ))),
        cut_err(p_end.context(cx().msg("Failed to parse end tag").lbl("end tag"))),
    )
        .parse_next(input)?;

    let mut blocks = Vec::new();
    blocks.push(if_body);
    blocks.extend(elif_bodies);
    Ok(Element::Conditional {
        blocks,
        else_block: else_block.map(|x| x.map_expr(|_| ())),
        end: end_line.0,
    })
}

fn wsp0_or_newline(input: &mut Input<'_>) -> PResult<()> {
    repeat(
        0..,
        alt((winnow::ascii::space1, winnow::ascii::line_ending)).void(),
    )
    .parse_next(input)
}

fn wsp1(input: &mut Input<'_>) -> PResult<()> {
    winnow::ascii::space1.void().parse_next(input)
}

#[cfg(test)]
fn new_input(s: &str) -> Input<'_> {
    use winnow::{stream::Recoverable, Located};
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
    use crate::util::test_util::{render_error, TestResult};
    use insta::assert_debug_snapshot;
    use winnow::Parser as _;

    use crate::templating::parser::{
        p_conditional_element, p_multiline_element, p_nextline_element, parse_document,
    };

    use super::{new_input, p_inline_element, p_tag_line};

    #[test]
    fn test_inline_tag() -> TestResult {
        assert_debug_snapshot!(p_inline_element.parse(new_input("foo /* {< test >} */")));
        Ok(())
    }

    #[test]
    fn test_inline_conditional_tag() -> TestResult {
        assert_debug_snapshot!(p_inline_element.parse(new_input("foo /* {< if test >} */")));
        assert_debug_snapshot!(p_inline_element.parse(new_input("foo /* {< iftest >} */")));
        Ok(())
    }

    #[test]
    fn test_nextline_tag() -> TestResult {
        assert_debug_snapshot!(p_nextline_element(&mut new_input("/* {# x #} */\nfoo"))?);
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
    fn test_blank_lines_get_combined() {
        assert_debug_snapshot!(parse_document("\n\n\n\n"));
    }

    #[test]
    fn test_regular_lines_get_combined() {
        assert_debug_snapshot!(parse_document(indoc::indoc! {r#"
            foo
            bar
            // {% if a %}
            foo
            bar
            baz
            // {% end %}
            foo
            bar
        "#}));
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
        assert_debug_snapshot!(parse_document(indoc::indoc! {r#"
            # {# replace_re(`'.*'`, `'{data.value}'`) #}
            value = 'foo'
        "#}));
    }

    #[test]
    fn test_blanklines_around_tag() {
        assert_debug_snapshot!(parse_document("a\n\n{%a%}\n{%end%}\n\na"));
        assert_debug_snapshot!(parse_document("a\n\n{%if a%}\n{%end%}\n\na"));
    }

    #[test]
    fn test_newline_in_inline() -> TestResult {
        assert_debug_snapshot!(parse_document("foo /* {< test\ntest >} */"));
        Ok(())
    }
    #[test]
    fn test_newlines_in_multiline() -> TestResult {
        assert_debug_snapshot!(parse_document("foo \n/* \n{#\ntest\ntest\n#} */\nbar"));
        Ok(())
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
    fn test_error_incomplete_multiline_long() {
        insta::assert_snapshot!(render_error(
            parse_document("\nfoo\n{%f%}\nbar\n").unwrap_err()
        ));
    }

    #[test]
    fn test_error_multiline_with_else() {
        insta::assert_snapshot!(render_error(
            parse_document("{%f%}\n{%else%}\n{%end%}").unwrap_err()
        ));
    }

    #[test]
    fn test_error_empty_tag() {
        insta::assert_snapshot!(render_error(parse_document("{%%}").unwrap_err()));
    }
}
