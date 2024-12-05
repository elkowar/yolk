use miette::Context;
use miette::IntoDiagnostic;

use miette::Result;
use rhai::Dynamic;
use rhai::Engine;
use rhai::Scope;

use crate::yolk::EvalMode;

use super::stdlib;

pub const YOLK_TEXT_NAME: &str = "YOLK_TEXT";

pub struct EvalCtx {
    engine: Engine,
    scope: Scope<'static>,
}

impl Default for EvalCtx {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl EvalCtx {
    pub fn new_empty() -> Self {
        let mut engine = Engine::new();
        engine.set_optimization_level(rhai::OptimizationLevel::Simple);
        Self {
            engine,
            scope: Scope::new(),
        }
    }

    pub fn new_in_mode(mode: EvalMode) -> Result<Self> {
        let mut ctx = Self::new_empty();
        stdlib::setup_tag_functions(&mut ctx)?;
        if mode == EvalMode::Local {
            stdlib::setup_impure_functions(&mut ctx)?;
        }
        Ok(ctx)
    }

    pub fn eval_rhai(&mut self, name: &str, content: &str) -> Result<Dynamic> {
        self.engine
            .eval_expression_with_scope(&mut self.scope, content)
            .into_diagnostic()
    }
    pub fn exec_rhai(&mut self, name: &str, content: &str) -> Result<()> {
        self.engine
            .run_with_scope(&mut self.scope, content)
            .into_diagnostic()
    }

    pub fn eval_text_transformation(&mut self, text: &str, expr: &str) -> Result<String> {
        let scope_before = self.scope.len();
        let text = text.to_string();
        self.engine
            .register_fn("get_yolk_text", move || text.clone());
        let result = self.eval_rhai("template tag", expr)?;
        self.scope.rewind(scope_before);
        Ok(result.to_string())
    }

    pub fn set_global(&mut self, name: &str, value: Dynamic) {
        self.scope.set_or_push(name, value);
    }
    pub fn get_global(&self, name: &str) -> Result<Dynamic> {
        self.scope
            .get_value(name)
            .with_context(|| format!("variable {} not found", name))
    }

    pub fn engine_mut(&mut self) -> &mut Engine {
        &mut self.engine
    }
}
