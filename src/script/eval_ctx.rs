use miette::Context;

use miette::Result;
use rhai::Dynamic;
use rhai::Engine;
use rhai::RhaiNativeFunc;
use rhai::Scope;
use rhai::Variant;

use crate::yolk::EvalMode;

use super::lua_error::RhaiError;
use super::stdlib;

pub const YOLK_TEXT_NAME: &str = "YOLK_TEXT";

pub struct EvalCtx {
    engine: Engine,
    scope: Scope<'static>,
    header_ast: Option<rhai::AST>,
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
            header_ast: None,
        }
    }

    pub fn new_in_mode(mode: EvalMode) -> Result<Self> {
        let mut ctx = Self::new_empty();
        stdlib::setup(mode, &mut ctx)?;
        Ok(ctx)
    }

    pub fn set_and_run_header_ast(&mut self, content: &str) -> Result<(), RhaiError> {
        let ast = self.compile(content)?;
        self.engine
            .run_ast_with_scope(&mut self.scope, &ast)
            .map_err(|e| RhaiError::from_rhai(content, *e))?;
        self.header_ast = Some(ast);
        Ok(())
    }

    pub fn eval_rhai<T: Variant + Clone>(&mut self, content: &str) -> Result<T, RhaiError> {
        let mut ast = self.compile(content)?;
        if let Some(header_ast) = &self.header_ast {
            ast = header_ast.merge(&ast);
        }
        self.engine
            .eval_ast_with_scope(&mut self.scope, &ast)
            .map_err(|e| RhaiError::from_rhai(content, *e))
    }

    pub fn exec_rhai(&mut self, content: &str) -> Result<(), RhaiError> {
        let mut ast = self.compile(content)?;
        if let Some(header_ast) = &self.header_ast {
            ast = header_ast.merge(&ast);
        }
        self.engine
            .run_ast_with_scope(&mut self.scope, &ast)
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

    pub fn get_global(&self, name: &str) -> Result<Dynamic> {
        self.scope
            .get_value(name)
            .with_context(|| format!("variable {} not found", name))
    }

    pub fn engine_mut(&mut self) -> &mut Engine {
        &mut self.engine
    }
    pub fn scope_mut(&mut self) -> &mut Scope<'static> {
        &mut self.scope
    }

    pub fn header_ast(&self) -> Option<&rhai::AST> {
        self.header_ast.as_ref()
    }

    pub fn call_fn<T: Variant + Clone>(&mut self, ast: &rhai::AST) -> Result<T, RhaiError> {
        Ok(self
            .engine
            .call_fn(&mut self.scope, ast, "eggs", ())
            .map_err(|e| RhaiError::from_rhai(&ast.source().unwrap(), *e))?)
    }

    #[inline]
    pub fn register_fn<
        A: 'static,
        const N: usize,
        const X: bool,
        R: Variant + Clone,
        const F: bool,
    >(
        &mut self,
        name: impl AsRef<str> + Into<rhai::Identifier>,
        func: impl RhaiNativeFunc<A, N, X, R, F> + Send + Sync + 'static,
    ) -> &mut Self {
        rhai::FuncRegistration::new(name.into()).register_into_engine(self.engine_mut(), func);
        self
    }

    fn compile(&mut self, text: &str) -> Result<rhai::AST, RhaiError> {
        self.engine
            .compile(text)
            .map_err(|e| RhaiError::from_rhai_compile(text, e))
    }
}
