use miette::IntoDiagnostic;

use miette::Result;
use mlua::ExternalResult as _;
use mlua::FromLua;
use mlua::FromLuaMulti;
use mlua::IntoLua;
use mlua::IntoLuaMulti;
use mlua::Lua;
use mlua::MaybeSend;
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
        let ctx = Self { lua };
        stdlib::setup_tag_functions(&ctx)?;
        if mode == EvalMode::Local {
            stdlib::setup_impure_functions(&ctx)?;
        }
        Ok(ctx)
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
        let old_text = self.get_global::<Value>(YOLK_TEXT_NAME)?;
        self.set_global(YOLK_TEXT_NAME, text)?;
        let result = self.eval_lua("template tag", expr)?;
        self.set_global(YOLK_TEXT_NAME, old_text)?;
        Ok(result)
    }

    pub fn set_global<T: IntoLua>(&self, name: impl IntoLua, value: T) -> Result<()> {
        self.lua.globals().set(name, value).into_diagnostic()
    }
    pub fn get_global<T: FromLua>(&self, name: impl IntoLua) -> Result<T> {
        self.lua.globals().get::<T>(name).into_diagnostic()
    }

    pub fn register_fn<F, A, R>(&self, name: &str, func: F) -> Result<()>
    where
        F: Fn(&Lua, A) -> Result<R> + MaybeSend + 'static + Send + Sync,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        self.set_global(
            name,
            self.lua
                .create_function(move |lua, x| func(lua, x).into_lua_err())
                .into_diagnostic()?,
        )
    }

    pub fn lua(&self) -> &Lua {
        &self.lua
    }
    pub fn lua_mut(&mut self) -> &mut Lua {
        &mut self.lua
    }
}
