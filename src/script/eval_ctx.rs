use std::path::Path;
use std::sync::Arc;

use miette::Result;
use rhai::module_resolvers::FileModuleResolver;
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
    yolk_file_module: Option<(rhai::AST, Arc<Module>)>,
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

    /// Initialize a new [`EvalCtx`] with set up modules and module resolver.
    ///
    /// The given mode is used when initializing the `io` module,
    /// to determine whether to actually perform any IO or to just simulate it.
    pub fn new_in_mode(mode: EvalMode) -> Result<Self> {
        let mut ctx = Self::new_empty();
        ctx.engine
            .register_global_module(Arc::new(stdlib::global_stuff()));
        ctx.engine
            .register_static_module("utils", Arc::new(stdlib::utils_module()));
        ctx.engine
            .register_static_module("io", Arc::new(stdlib::io_module(mode)));
        let template_module = Arc::new(stdlib::tag_module());
        ctx.engine
            .register_static_module("template", template_module);

        Ok(ctx)
    }

    /// Set the directory to look for imports in.
    ///
    /// The given `path` is used as the path for the a [`FileModuleResolver`],
    /// such that `import` statements can be used in rhai code relative to this path.
    pub fn set_module_path(&mut self, path: &Path) {
        self.engine
            .set_module_resolver(FileModuleResolver::new_with_path(path));
    }

    /// Load a given rhai string as a global module, and store it as the `yolk_file_module`.
    pub fn load_rhai_file_to_module(&mut self, content: &str) -> Result<(), RhaiError> {
        let ast = self.compile(content)?;
        let module = Module::eval_ast_as_new(self.scope.clone(), &ast, &self.engine)
            .map_err(|e| RhaiError::from_rhai(&self.engine, content, *e))?;
        let module = Arc::new(module);
        self.engine.register_global_module(module.clone());
        self.yolk_file_module = Some((ast, module.clone()));
        Ok(())
    }

    /// Eval a given string of rhai and return the result. Execute in the scope of this [`EvalCtx`].
    pub fn eval_rhai<T: Variant + Clone>(&mut self, content: &str) -> Result<T, RhaiError> {
        let mut ast = self.compile(content)?;
        if let Some((yolk_file_ast, _)) = self.yolk_file_module.as_ref() {
            ast = yolk_file_ast.merge(&ast);
        }
        self.engine
            .eval_ast_with_scope(&mut self.scope, &ast)
            .map_err(|e| RhaiError::from_rhai(&self.engine, content, *e))
    }

    /// Eval a rhai expression in a scope that has a special `get_yolk_text()` function that returns the given `text`.
    /// After the expression is evaluated, the scope is rewound to its state before the function was added.
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

    pub fn yolk_file_module(&self) -> Option<&(rhai::AST, Arc<Module>)> {
        self.yolk_file_module.as_ref()
    }

    fn compile(&mut self, text: &str) -> Result<rhai::AST, RhaiError> {
        self.engine
            .compile(text)
            .map_err(|e| RhaiError::from_rhai_compile(&self.engine, text, e))
    }
}
