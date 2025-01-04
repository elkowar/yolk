#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error("{message}")]
#[diagnostic()]
pub struct MultiError {
    message: String,
    #[related]
    errors: Vec<miette::Report>,
}

impl MultiError {
    pub fn new(message: impl Into<String>, errors: Vec<miette::Report>) -> Self {
        Self {
            message: message.into(),
            errors,
        }
    }
}
impl From<miette::Report> for MultiError {
    fn from(report: miette::Report) -> Self {
        Self {
            message: "Something went wrong".to_string(),
            errors: vec![report],
        }
    }
}
