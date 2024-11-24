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

    #[allow(unused)]
    pub fn scope(&self) -> &rhai::Scope<'a> {
        &self.scope
    }
    pub fn scope_mut(&mut self) -> &mut rhai::Scope<'a> {
        &mut self.scope
    }
}
