use miette::Context as _;
use miette::Error;
use miette::IntoDiagnostic;
use miette::LabeledSpan;
use miette::Result;
use mlua::FromLuaMulti;
use mlua::Lua;
use mlua::Value;

use super::stdlib;

pub const YOLK_TEXT_NAME: &str = "YOLK_TEXT";

// TODO: Ensure an EvalCtx contains info about what file is being parsed,
// the egg name, etc etc
pub struct EvalCtx {
    lua: Lua,
}

impl Default for EvalCtx {
    fn default() -> Self {
        Self::new()
    }
}

impl EvalCtx {
    pub fn new() -> Self {
        let lua = Lua::new();
        Self { lua }
    }
    pub fn new_for_tag() -> Result<Self> {
        let lua = Lua::new();
        stdlib::setup_tag_functions(&lua)?;
        Ok(Self { lua })
    }

    pub fn eval_expr<T: FromLuaMulti>(&mut self, expr: &str) -> Result<T> {
        self.lua
            .load(expr)
            .set_name("Expression")
            .eval::<T>()
            .into_diagnostic()
    }

    pub fn eval_text_transformation(&mut self, text: &str, expr: &str) -> Result<String> {
        let globals = self.lua.globals();
        let old_text = globals.get::<Value>(YOLK_TEXT_NAME).into_diagnostic()?;
        self.lua
            .globals()
            .set(YOLK_TEXT_NAME, text)
            .into_diagnostic()?;
        let result = self
            .lua
            .load(expr)
            .set_name("text transformation expr")
            .eval::<String>()
            .into_diagnostic()?;
        self.lua
            .globals()
            .set(YOLK_TEXT_NAME, old_text)
            .into_diagnostic()?;
        Ok(result)
    }

    pub fn lua(&self) -> &Lua {
        &self.lua
    }
    pub fn lua_mut(&mut self) -> &mut Lua {
        &mut self.lua
    }
}
