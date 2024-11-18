use anyhow::Result;
use document::Document;
use pest_derive::Parser;

mod ctx;
mod document;
mod element;

pub(crate) const COMMENT_START: &str = " ==x== ";

#[derive(Parser)]
#[grammar = "yolk.pest"]
pub struct YolkParser;

pub(crate) fn main() -> Result<()> {
    let example = r#"
        // {% CommentPrefix // %}
        hallo
        // {% if true %}
        test
        // {% else %}
        bruh
        // {% end %}
    "#;
    let document = document::Document::parse_string(example)?;

    let result = document.render()?;
    println!("{}", result);
    println!("{}", Document::parse_string(&result)?.render()?);
    Ok(())
}
