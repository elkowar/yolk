use std::sync::Arc;

use miette::Result;
use rhai::Engine;
use rhai::Module;
use rhai::Scope;
use rhai::Variant;

use crate::yolk::EvalMode;

use super::rhai_error::RhaiError;
use super::stdlib;

pub const YOLK_TEXT_NAME: &str = "YOLK_TEXT";

#[derive(Debug)]
pub struct EvalCtx {
    engine: Engine,
    scope: Scope<'static>,
    yolk_file_module: Option<Arc<Module>>,
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

        engine.build_type::<super::sysinfo::SystemInfo>();
        engine.build_type::<super::sysinfo::SystemInfoPaths>();
        Self {
            engine,
            scope: Scope::new(),
            yolk_file_module: None,
        }
    }

    pub fn new_in_mode(mode: EvalMode) -> Result<Self> {
        let mut ctx = Self::new_empty();
        ctx.engine
            .register_global_module(Arc::new(stdlib::global_stuff()));
        ctx.engine
            .register_static_module("utils", Arc::new(stdlib::utils_module()));
        ctx.engine
            .register_static_module("io", Arc::new(stdlib::io_module(mode)));
        let template_module = Arc::new(stdlib::tag_module());
        ctx.engine.register_global_module(template_module.clone());
        ctx.engine
            .register_static_module("template", template_module);

        Ok(ctx)
    }

    pub fn load_as_global_module(&mut self, content: &str) -> Result<(), RhaiError> {
        let ast = self.compile(content)?;
        let module = Module::eval_ast_as_new(self.scope.clone(), &ast, &self.engine)
            .map_err(|e| RhaiError::from_rhai(content, *e))?;
        let module = Arc::new(module);
        self.engine.register_global_module(module.clone());
        self.yolk_file_module = Some(module.clone());
        Ok(())
    }

    pub fn eval_rhai<T: Variant + Clone>(&mut self, content: &str) -> Result<T, RhaiError> {
        let ast = self.compile(content)?;
        self.engine
            .eval_ast_with_scope(&mut self.scope, &ast)
            .map_err(|e| RhaiError::from_rhai(content, *e))
    }

    pub fn eval_text_transformation(
        &mut self,
        text: &str,
        expr: &str,
    ) -> Result<String, RhaiError> {
        let scope_before = self.scope.len();
        let text = text.to_string();
        self.engine
            .register_fn("get_yolk_text", move || text.clone());
        let result = self.eval_rhai::<String>(expr)?;
        self.scope.rewind(scope_before);
        Ok(result.to_string())
    }

    pub fn set_global<T: Variant + Clone>(&mut self, name: &str, value: T) {
        self.scope.set_or_push(name, value);
    }

    pub fn engine_mut(&mut self) -> &mut Engine {
        &mut self.engine
    }

    pub fn yolk_file_module(&self) -> Option<&Arc<Module>> {
        self.yolk_file_module.as_ref()
    }

    fn compile(&mut self, text: &str) -> Result<rhai::AST, RhaiError> {
        self.engine
            .compile(text)
            .map_err(|e| RhaiError::from_rhai_compile(text, e))
    }
}
