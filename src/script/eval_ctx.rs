use miette::IntoDiagnostic;

use miette::Result;
use mlua::FromLuaMulti;
use mlua::Lua;
use mlua::Value;

use crate::yolk::EvalMode;

use super::stdlib;

pub const YOLK_TEXT_NAME: &str = "YOLK_TEXT";

pub struct EvalCtx {
    lua: Lua,
}

impl Default for EvalCtx {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl EvalCtx {
    pub fn new_empty() -> Self {
        let lua = Lua::new();
        Self { lua }
    }

    pub fn new_in_mode(mode: EvalMode) -> Result<Self> {
        let lua = Lua::new();
        stdlib::setup_tag_functions(&lua)?;
        if mode == EvalMode::Local {
            stdlib::setup_impure_functions(&lua)?;
        }
        Ok(Self { lua })
    }

    pub fn eval_lua<T: FromLuaMulti>(&self, name: &str, content: &str) -> Result<T> {
        self.lua()
            .load(content)
            .set_name(name)
            .eval()
            .into_diagnostic()
    }
    pub fn exec_lua(&self, name: &str, content: &str) -> Result<()> {
        self.lua()
            .load(content)
            .set_name(name)
            .exec()
            .into_diagnostic()
    }

    pub fn eval_text_transformation(&self, text: &str, expr: &str) -> Result<String> {
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
