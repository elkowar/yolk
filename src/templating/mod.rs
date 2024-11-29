use pest_derive::Parser;

pub mod document;
pub mod element;
mod linewise;

pub(crate) const COMMENT_START: &str = "<yolk> ";

#[derive(Debug)]
pub struct TaggedLine<'a> {
    left: &'a str,
    tag: &'a str,
    right: &'a str,
    full_line: &'a str,
}

#[derive(Parser)]
#[grammar = "templating/yolk.pest"]
pub struct YolkParser;

#[cfg(test)]
mod test {
    use pest::parses_to;

    use crate::{
        eval_ctx::EvalCtx,
        templating::{
            document::{self},
            Rule, YolkParser,
        },
    };
    /*

    #[test]
    pub fn test_parse_directive() {
        use pest::consumes_to;
        parses_to! {
            parser: YolkParser,
            input: "{% CommentPrefix // %}",
            rule: Rule::DirectiveTag,
            tokens: [
                DirectiveTag(0, 22, [DirectiveName(3, 16), TagInner(17, 19), EOI(22, 22)]),
            ]
        };
    }

    #[test]
    pub fn test_template_if() {
        let mut eval_ctx = EvalCtx::new();

        let example = indoc::indoc! {r#"
            // {% CommentPrefix // %}
            // {% if 2+2 == 1 %}
            test
            // {% else %}
            test2
            // {% end %}
        "#};
        let document = document::Document::parse_string(example).unwrap();
        let result = document.render(&mut eval_ctx).unwrap();
        assert_eq!(
            indoc::indoc! { r#"
            // {% CommentPrefix // %}
            // {% if 2+2 == 1 %}
            //<yolk> test
            // {% else %}
            test2
            // {% end %}
            "#},
            result
        )
    }

    #[test]
    pub fn test_template_replace() {
        let mut eval_ctx = EvalCtx::new();
        let example = indoc::indoc! {r#"
            # {% replace /".*"/ `"${2+2}"` %}
            name = "foo"
        "#};
        let document = document::Document::parse_string(example).unwrap();
        let result = document.render(&mut eval_ctx).unwrap();
        assert_eq!(
            indoc::indoc! { r#"
                # {% replace /".*"/ `"${2+2}"` %}
                name = "4"
            "#},
            result
        )
    }
    */
}
