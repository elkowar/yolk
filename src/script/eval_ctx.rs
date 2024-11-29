use anyhow::Context;
use anyhow::Result;

use super::make_engine;

// TODO: Ensure an EvalCtx contains info about what file is being parsed,
// the egg name, etc etc
pub struct EvalCtx<'a> {
    scope: rhai::Scope<'a>,
}

impl<'a> EvalCtx<'a> {
    pub fn new() -> Self {
        let scope = rhai::Scope::new();
        Self { scope }
    }

    pub fn eval<T: Clone + 'static + Send + Sync>(&mut self, expr: &str) -> Result<T> {
        let engine = make_engine();
        engine
            .eval_expression_with_scope::<T>(&mut self.scope, expr)
            .with_context(|| format!("Failed to evaluate expression: {}", expr))
    }

    pub fn eval_text_transformation(&mut self, text: &str, expr: &str) -> Result<String> {
        let engine = make_engine();
        let scope_before = self.scope.len();
        self.scope.push_constant("TAG_BLOCK", text.to_string());
        let new_text = engine
            .eval_expression_with_scope::<String>(&mut self.scope, expr)
            .with_context(|| format!("Failed to evaluate expression: {}", expr))?;
        self.scope.rewind(scope_before);
        Ok(new_text)
    }

    #[allow(unused)]
    pub fn scope(&self) -> &rhai::Scope<'a> {
        &self.scope
    }
    pub fn scope_mut(&mut self) -> &mut rhai::Scope<'a> {
        &mut self.scope
    }
}
