pub mod comment_style;
pub mod document;
pub mod element;
pub mod error;
mod parser;

pub(crate) const COMMENT_START: &str = "<yolk> ";

#[cfg(test)]
mod test;
