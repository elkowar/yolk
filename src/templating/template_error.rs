use std::fmt::Display;

use miette::{Diagnostic, NamedSource, SourceSpan};

use crate::script::lua_error::LuaError;

use super::parse_error::YolkParseFailure;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum TemplateError {
    #[error(transparent)]
    ParseError(
        #[diagnostic_source]
        #[from]
        YolkParseFailure,
    ),
    #[error(transparent)]
    #[diagnostic(transparent)]
    LuaEvalError(#[from] LuaEvalError),
}

#[derive(Debug, thiserror::Error)]
#[error("{lua_error}")]
pub struct LuaEvalError {
    #[source]
    lua_error: LuaError,
    template_source: NamedSource<String>,
    lua_expr_span: SourceSpan,
    template_element_span: SourceSpan,
}

impl LuaEvalError {
    pub fn from_lua_error(
        lua_error: LuaError,
        source: NamedSource<String>,
        lua_expr_span: SourceSpan,
        template_element_span: SourceSpan,
    ) -> Self {
        Self {
            lua_error,
            template_source: source,
            lua_expr_span,
            template_element_span,
        }
    }
}

impl Diagnostic for LuaEvalError {
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        None
    }

    fn severity(&self) -> Option<miette::Severity> {
        Some(miette::Severity::Error)
    }

    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        None
    }

    fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        None
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        Some(&self.template_source)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        let lua_start = self.lua_expr_span.offset() + self.lua_error.span.start;
        let lua_end = self.lua_expr_span.offset() + self.lua_error.span.end;
        let labels = vec![
            miette::LabeledSpan::at(lua_start..lua_end, "here"),
            miette::LabeledSpan::at(self.template_element_span, "in this template element"),
        ];
        Some(Box::new(labels.into_iter()))
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        None
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        None
    }
}
