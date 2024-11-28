/* G
use winnow::{
    combinator::repeat_till,
    token::{any, literal, take_till, take_until, take_while},
    IResult, PResult, Parser,
};

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

pub fn p_raw<'s>(input: &mut &'s str) -> IResult<&'s str, &'s str> {
    take_till(0.., ("{%", "{%")).parse_peek(input)
}

pub fn p_replace_block<'s>(input: &mut &'s str) -> IResult<&'s str, Element> {
    let (input, _) = literal("{%").parse_peek(input)?;
    let (input, _) = take_until(0.., "replace")(input)?;
    let (input, _) = take_until(0.., "%}")(input)?;
    let (input, _) = take_until(0.., "{%")(input)?;
}

*/
