#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error("{message}")]
#[diagnostic()]
pub struct MultiError {
    message: String,
    #[diagnostic_source]
    errors: ErrList,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error("Something went wrong")]
#[diagnostic()]
pub struct ErrList(#[related] Vec<miette::Report>);

impl MultiError {
    pub fn new(message: impl Into<String>, errors: Vec<miette::Report>) -> Self {
        Self {
            message: message.into(),
            errors: ErrList(errors),
        }
    }
}
impl From<miette::Report> for MultiError {
    fn from(report: miette::Report) -> Self {
        Self {
            message: "Something went wrong".to_string(),
            errors: ErrList(vec![report]),
        }
    }
}
