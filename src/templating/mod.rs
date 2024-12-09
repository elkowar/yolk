pub mod comment_style;
pub mod document;
pub mod element;
pub mod parse_error;
mod parser;
pub mod template_error;

pub(crate) const COMMENT_START: &str = "<yolk> ";

#[cfg(test)]
mod test {
    use testresult::TestResult;

    use crate::script::eval_ctx::EvalCtx;
    use crate::templating::document::Document;
    use crate::yolk::EvalMode;

    #[test]
    pub fn test_render_inline() -> TestResult {
        let mut eval_ctx = EvalCtx::new_in_mode(EvalMode::Local)?;
        let doc = Document::parse_string("foo /* {< string.upper(YOLK_TEXT) >} */")?;
        assert_eq!(
            "FOO /* {< string.upper(YOLK_TEXT) >} */",
            doc.render(&mut eval_ctx)?
        );
        Ok(())
    }

    #[test]
    pub fn test_render_next_line() -> TestResult {
        let mut eval_ctx = EvalCtx::new_in_mode(EvalMode::Local)?;
        let doc = Document::parse_string("/* {# string.upper(YOLK_TEXT) #} */\nfoo\n")?;
        dbg!(&doc);
        assert_eq!(
            "/* {# string.upper(YOLK_TEXT) #} */\nFOO\n",
            doc.render(&mut eval_ctx)?
        );
        Ok(())
    }

    #[test]
    pub fn test_render_inline_conditional() -> TestResult {
        let mut eval_ctx = EvalCtx::new_in_mode(EvalMode::Local)?;
        // TODO: nested comment syntax is not supported everywhere, how do we deal with that?
        let doc = Document::parse_string("foo/* {< if false >} */")?;
        assert_eq!(
            "/*<yolk> foo/* {< if false >} */*/",
            doc.render(&mut eval_ctx)?
        );
        Ok(())
    }

    #[test]
    pub fn test_render_next_line_conditional() -> TestResult {
        let mut eval_ctx = EvalCtx::new_in_mode(EvalMode::Local)?;
        let doc = Document::parse_string("/* {# if false #} */\nfoo\n")?;
        assert_eq!(
            "/* {# if false #} */\n/*<yolk> foo*/\n",
            doc.render(&mut eval_ctx)?
        );
        Ok(())
    }

    #[test]
    pub fn test_multiline_conditional() -> TestResult {
        let mut eval_ctx = EvalCtx::new_in_mode(EvalMode::Local)?;
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
                /*<yolk> foo*/
                /* {% elif true %} */
                bar
                /* {% else %} */
                /*<yolk> bar*/
                /* {% end %} */
            "#},
            doc.render(&mut eval_ctx)?
        );
        Ok(())
    }

    #[test]
    pub fn test_render_replace() -> TestResult {
        let doc = Document::parse_string(indoc::indoc! {"
            {# replace_re(`'.*'`, `'new'`) #}
            foo: 'original'
        "})?;
        let mut eval_ctx = EvalCtx::new_in_mode(EvalMode::Local)?;
        assert_eq!(
            indoc::indoc! {"
                {# replace_re(`'.*'`, `'new'`) #}
                foo: 'new'
            "},
            dbg!(doc).render(&mut eval_ctx)?
        );
        Ok(())
    }

    #[test]
    pub fn test_render_replace_refuse_non_idempodent() -> TestResult {
        let element = Document::parse_string("{# replace(`'.*'`, `a'a'`) #}\nfoo: 'original'")?;
        let mut eval_ctx = EvalCtx::new_in_mode(EvalMode::Local)?;
        assert!(element.render(&mut eval_ctx).is_err());
        // assert_eq!(
        //     "{# replace(`'.*'`, `a'a'`) #}\nfoo: 'original'",
        //     element.render(&mut eval_ctx)?
        // );
        Ok(())
    }

    #[test]
    pub fn test_render_noop() -> TestResult {
        let doc = Document::parse_string(indoc::indoc! {"
            # foo
                indented
                    indented more
                    foo // {< if true >}
                indented
            not
        "})?;
        let mut eval_ctx = EvalCtx::new_in_mode(EvalMode::Local)?;
        assert_eq!(
            indoc::indoc! {"
                # foo
                    indented
                        indented more
                        foo // {< if true >}
                    indented
                not
                "},
            doc.render(&mut eval_ctx)?
        );
        Ok(())
    }
    #[test]
    pub fn test_preserve_blank_lines_around_conditional_tag() -> TestResult {
        let doc = Document::parse_string(indoc::indoc! {"
            foo

            {% if true %}
            foo
            {% end %}

            foo
        "})?;
        let mut eval_ctx = EvalCtx::new_in_mode(EvalMode::Local)?;
        assert_eq!(
            indoc::indoc! {"
                foo

                {% if true %}
                foo
                {% end %}

                foo
            "},
            doc.render(&mut eval_ctx)?
        );
        Ok(())
    }
}
