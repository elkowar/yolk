use std::fmt::{Debug, Display};

use miette::{Diagnostic, LabeledSpan, Severity, SourceCode};

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error("{message}")]
#[diagnostic()]
pub struct MultiError<E: Debug + Diagnostic = ReportDiagnostic> {
    message: String,
    #[related]
    errors: Vec<E>,
}

impl MultiError {
    pub fn new(message: impl Into<String>, errors: Vec<miette::Report>) -> Self {
        Self {
            message: message.into(),
            errors: errors.into_iter().map(Into::into).collect(),
        }
    }
}

impl<E: Debug + Diagnostic> MultiError<E> {
    pub fn new_typed(message: impl Into<String>, errors: Vec<E>) -> Self {
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
            errors: vec![report.into()],
        }
    }
}

#[derive(Debug)]
pub struct ReportDiagnostic(miette::Report);

impl From<miette::Report> for ReportDiagnostic {
    fn from(report: miette::Report) -> Self {
        Self(report)
    }
}

impl Display for ReportDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl std::error::Error for ReportDiagnostic {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        let error: &dyn std::error::Error = self.0.as_ref();
        error.source()
    }
}

impl Diagnostic for ReportDiagnostic {
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        let diagnostic: &dyn Diagnostic = self.0.as_ref();
        diagnostic.code()
    }

    fn severity(&self) -> Option<Severity> {
        let diagnostic: &dyn Diagnostic = self.0.as_ref();
        diagnostic.severity()
    }

    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        let diagnostic: &dyn Diagnostic = self.0.as_ref();
        diagnostic.help()
    }

    fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        let diagnostic: &dyn Diagnostic = self.0.as_ref();
        diagnostic.url()
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        let diagnostic: &dyn Diagnostic = self.0.as_ref();
        diagnostic.source_code()
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        let diagnostic: &dyn Diagnostic = self.0.as_ref();
        diagnostic.labels()
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        let diagnostic: &dyn Diagnostic = self.0.as_ref();
        diagnostic.related()
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        let diagnostic: &dyn Diagnostic = self.0.as_ref();
        diagnostic.diagnostic_source()
    }
}
