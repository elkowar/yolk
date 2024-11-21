use anyhow::Context;
use anyhow::{anyhow, Result};
use rhai::CustomType;
use rhai::TypeBuilder;

pub fn make_engine() -> rhai::Engine {
    let mut engine = rhai::Engine::new();
    engine
        .register_type::<SystemInfo>()
        .build_type::<SystemInfo>();
    engine
}

// TODO: Ensure an EvalCtx contains info about what file is being parsed,
// the thing name, etc etc
pub struct EvalCtx<'a> {
    scope: rhai::Scope<'a>,
}

impl<'a> EvalCtx<'a> {
    pub fn new(sysinfo: SystemInfo) -> Self {
        let mut scope = rhai::Scope::new();
        scope.push_constant("system", sysinfo);
        Self { scope }
    }

    pub fn eval<T: Clone + 'static + Send + Sync>(&mut self, expr: &str) -> Result<T> {
        let engine = make_engine();
        Ok(engine
            .eval_expression_with_scope::<T>(&mut self.scope, expr)
            .with_context(|| format!("Failed to evaluate expression: {}", expr))?)
    }

    #[allow(unused)]
    pub fn scope(&self) -> &rhai::Scope<'a> {
        &self.scope
    }
    pub fn scope_mut(&mut self) -> &mut rhai::Scope<'a> {
        &mut self.scope
    }
}

#[derive(Debug, Clone, CustomType)]
pub struct SystemInfo {
    hostname: String,
    username: String,
}

impl SystemInfo {
    pub fn generate() -> Self {
        // lmao make this not garbage
        Self {
            hostname: std::env::var("HOSTNAME").unwrap(),
            username: std::env::var("USER").unwrap(),
        }
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        Self {
            hostname: "host".to_string(),
            username: "johndoe".to_string(),
        }
    }
}

#[cfg(test)]
mod test {}
