use std::ops::Range;

use chumsky::prelude::*;
use chumsky::{error::Simple, Parser};
use text::ident;

pub type Span = Range<usize>;
pub type Spanned<T> = (T, Span);

#[derive(Debug)]
pub enum Element {
    Raw(String),
    IfBlock {
        pred: String,
        if_tag: String,
        body: Box<Element>,
        else_tag_and_body: Option<(String, Box<Element>)>,
        end_tag: String,
    },
    ReplaceInline {
        before_tag: String,
        tag_span: Span,
        regex_pattern: String,
        expr: String,
    },
    ReplaceBlock {
        tag_span: Span,
        regex_pattern: String,
        expr: String,
        affected_line: (String, Span),
    },
}

fn spanned<T>(
    p: impl Parser<char, T, Error = Simple<char>>,
) -> impl Parser<char, Spanned<T>, Error = Simple<char>> {
    p.map_with_span(|t, span| (t, span))
}

fn p_element() -> impl Parser<char, Element, Error = Simple<char>> {
    choice::<_, Simple<char>>((p_replace_block(), p_raw())).labelled("element")
}

fn p_raw() -> impl Parser<char, Element, Error = Simple<char>> {
    any()
        .repeated()
        .collect()
        .map(|e| Element::Raw(e))
        .labelled("raw")
}

fn p_replace_block() -> impl Parser<char, Element, Error = Simple<char>> {
    just("{%")
        .ignore_then(just("replace").padded())
        .ignore_then(p_regex().padded())
        .then(p_tag_inner().padded())
        .then_ignore(just("%}").then(none_of("\n").repeated().then(just("\n"))))
        .then(spanned(none_of("\n").repeated().collect::<String>()))
        .map_with_span(|((regex, expr), affected), span| Element::ReplaceBlock {
            tag_span: span,
            regex_pattern: regex,
            expr,
            affected_line: affected,
        })
        .labelled("replace block")
}

fn p_replace_inline() -> impl Parser<char, Element, Error = Simple<char>> {
    none_of("\n")
        .repeated()
        .collect()
        .debug("preceding line")
        .then_ignore(just("{<").debug("{<"))
        .then_ignore(just("replace").debug("replace"))
        .then(p_regex().padded().debug("regex"))
        .then(p_tag_inner().padded().debug("inner"))
        .then_ignore(just(">}").debug("end").then(
            none_of("\n").repeated(), // .then(just("\n").ignored().or(end().ignored())),
        ))
        .map_with_span(|((preceding, regex), expr), span| Element::ReplaceInline {
            tag_span: span,
            regex_pattern: regex,
            expr,
            before_tag: preceding,
        })
        .labelled("inline replace")
}

/// Parse anything until "%}"
fn p_tag_inner() -> impl Parser<char, String, Error = Simple<char>> {
    // TODO: support inline expressions properly
    filter(|&c| c != '%' && c != '>')
        .repeated()
        .collect()
        .then_ignore(one_of("%>").then(just("}")).rewind())
        .labelled("tag inner")
}

/// Parse a regex between / and /
fn p_regex() -> impl Parser<char, String, Error = Simple<char>> {
    just('/')
        .ignore_then(filter(|&c| c != '/').repeated().collect::<String>())
        .then_ignore(just('/'))
        .labelled("regex")
}

#[cfg(test)]
mod test {
    use ariadne::{Color, Label, Report, ReportKind, Source};
    use chumsky::{error::Simple, Parser};

    use crate::templating::parser::{p_replace_block, p_replace_inline};

    use super::p_element;

    fn test_parse<T, P: Parser<char, T, Error = Simple<char>>>(parser: P, src: &str) -> T {
        let (value, errs) = parser.parse_recovery_verbose(src);

        if !errs.is_empty() {
            errs.into_iter().for_each(|e| {
                Report::build(ReportKind::Error, e.span())
                    .with_message(e.to_string())
                    .with_label(
                        Label::new(e.span())
                            .with_message(e)
                            // .with_message(e.reason())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .print(Source::from(&src))
                    .unwrap()
            });
        }
        value.unwrap()
    }

    #[test]
    pub fn test_parser() {
        let result = test_parse(p_element(), "{% replace /foo/ foo %}\nfoo");
        println!("{:#?}", result);
        println!(
            "{:?}",
            test_parse(p_replace_block(), "{% replace /foo/ foo %}\nfoo")
        );
        println!(
            "{:?}",
            test_parse(p_replace_inline(), "foo # {< replace /foo/ foo >}")
        );
        panic!()
    }
}
