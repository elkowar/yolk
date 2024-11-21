use pest_derive::Parser;

pub mod document;
pub mod element;

pub(crate) const COMMENT_START: &str = "<yolk> ";

#[derive(Parser)]
#[grammar = "templating/yolk.pest"]
pub struct YolkParser;

#[cfg(test)]
mod test {
    use crate::{
        eval_ctx::{EvalCtx, SystemInfo},
        templating::document::{self},
    };

    #[test]
    pub fn test_template_if() {
        let mut eval_ctx = EvalCtx::new(SystemInfo::mock());

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
        let mut eval_ctx = EvalCtx::new(SystemInfo::mock());

        let example = indoc::indoc! {r#"
            # {% replace(/".*"/, '"${system.hostname}"')%}
            name = "foo"
        "#};
        let document = document::Document::parse_string(example).unwrap();
        let result = document.render(&mut eval_ctx).unwrap();
        assert_eq!(
            indoc::indoc! { r#"
                # {% replace(/".*"/, `"${system.hostname}"`)%}
                name = "host"
            "#},
            result
        )
    }
}
