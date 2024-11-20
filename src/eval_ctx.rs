use anyhow::{anyhow, Result};

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

    pub fn eval<T: Clone + 'static>(&mut self, expr: &str) -> Result<T> {
        let engine = rhai::Engine::new();
        let result = engine.eval_expression_with_scope::<T>(&mut self.scope, expr);
        match result {
            Ok(x) => Ok(x),
            Err(err) => Err(anyhow!(err.to_string())),
        }
    }

    pub fn scope(&self) -> &rhai::Scope<'a> {
        &self.scope
    }
    pub fn scope_mut(&mut self) -> &mut rhai::Scope<'a> {
        &mut self.scope
    }
}

#[derive(Debug, Clone)]
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

    pub fn mock() -> Self {
        Self {
            hostname: "host".to_string(),
            username: "johndoe".to_string(),
        }
    }
}

pub fn foo() -> Result<()> {
    let engine = rhai::Engine::new();
    let mut scope = rhai::Scope::new();
    let mut obj = rhai::Map::new();
    obj.insert("bar".into(), rhai::Dynamic::from_int(10));
    scope.set_value("foo", obj);

    let result = engine
        .eval_expression_with_scope::<i64>(&mut scope, "foo.bar * 2")
        .unwrap();
    println!("{}", result);

    Ok(())
}
#[cfg(test)]
mod test {}
