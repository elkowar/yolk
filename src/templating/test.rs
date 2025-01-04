use rstest::{fixture, rstest};

use crate::util::test_util::TestResult;

use crate::script::eval_ctx::EvalCtx;
use crate::templating::document::Document;
use crate::yolk::EvalMode;
use indoc::indoc;

#[fixture]
pub fn eval_ctx() -> EvalCtx {
    EvalCtx::new_in_mode(EvalMode::Local).unwrap()
}

#[rstest]
#[case::inline_tag(
    "foo /* {< get_yolk_text().to_upper() >} */",
    "FOO /* {< get_yolk_text().to_upper() >} */"
)]
#[case::nextline_tag(
    "/* {# get_yolk_text().to_upper() #} */\nfoo\n",
    "/* {# get_yolk_text().to_upper() #} */\nFOO\n"
)]
#[case::multiline(
    indoc!{r#"
        /* {% get_yolk_text().to_upper() %} */
        foo
        /* {% end %} */
    "#},
    indoc! {r#"
        /* {% get_yolk_text().to_upper() %} */
        FOO
        /* {% end %} */
    "#},
)]
#[case::inline_conditional("foo/* {< if false >} */", "/*<yolk> foo/* {< if false >} */*/")]
#[case::nextline_conditional(
    "/* {# if false #} */\nfoo\n",
    "/* {# if false #} */\n/*<yolk> foo*/\n"
)]
#[case::multiline_conditional(
    indoc!{r#"
        /* {% if false %} */
        foo
        /* {% elif false %} */
        foo
        /* {% elif true %} */
        bar
        /* {% else %} */
        bar
        /* {% end %} */
    "#},
    indoc!{r#"
        /* {% if false %} */
        /*<yolk> foo*/
        /* {% elif false %} */
        /*<yolk> foo*/
        /* {% elif true %} */
        bar
        /* {% else %} */
        /*<yolk> bar*/
        /* {% end %} */
    "#}
)]
#[case::nextline_conditional_with_newlines(
    indoc!{"
        /* {#
            if false
        #} */
        foo
    "},
    indoc!{"
        /* {#
            if false
        #} */
        /*<yolk> foo*/
    "},
)]
#[case::replace(
    indoc!{r#"
        {# replace_re(`'.*'`, `'new'`) #}
        foo: 'original'
    "#},
    indoc!{r#"
        {# replace_re(`'.*'`, `'new'`) #}
        foo: 'new'
    "#}
)]
pub fn test_render(
    mut eval_ctx: EvalCtx,
    #[case] input: &str,
    #[case] expected: &str,
) -> TestResult {
    let doc = Document::parse_string(input)?;
    let actual = doc.render(&mut eval_ctx)?;
    pretty_assertions::assert_eq!(expected, actual);
    Ok(())
}

#[rstest]
#[case::regression_keep_indents(indoc!{r#"
    # foo
        indented
            indented more
            foo // {< if true >}
        indented
    not
"#} )]
#[case::regression_blank_lines_around_conditional(indoc!{"
    foo

    {% if true %}
    foo
    {% end %}

    foo
"})]
pub fn test_render_noop(mut eval_ctx: EvalCtx, #[case] input: &str) -> TestResult {
    let doc = Document::parse_string(input)?;
    let actual = doc.render(&mut eval_ctx)?;
    pretty_assertions::assert_eq!(input, actual);
    Ok(())
}

#[test]
pub fn test_render_replace_refuse_non_idempodent() -> TestResult {
    let element = Document::parse_string("{# replace(`'.*'`, `a'a'`) #}\nfoo: 'original'")?;
    let mut eval_ctx = EvalCtx::new_in_mode(EvalMode::Local)?;
    assert!(element.render(&mut eval_ctx).is_err());
    Ok(())
}
