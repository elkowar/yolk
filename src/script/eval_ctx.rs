use miette::IntoDiagnostic;
use mlua::ExternalResult as _;
use mlua::FromLua;
use mlua::FromLuaMulti;
use mlua::IntoLua;
use mlua::IntoLuaMulti;
use mlua::Lua;
use mlua::MaybeSend;
use mlua::Value;

use crate::yolk::EvalMode;

use super::lua_error::LuaError;
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
        Self { lua: Lua::new() }
    }

    pub fn new_in_mode(mode: EvalMode) -> miette::Result<Self> {
        let ctx = Self::new_empty();

        // TODO: Properly set load path

        /*
        let load_path = ctx
            .lua
            .globals()
            .get::<mlua::Table>("package")
            .into_diagnostic()?
            .get::<String>("path")
            .into_diagnostic()?;
        let yolk_dir = std::path::PathBuf::from("/home/elk/.config/yolk/?.luau");
        let lua_path = match load_path.as_str() {
            "" => yolk_dir.to_string_lossy().to_string(),
            other => format!("{};{other}", yolk_dir.to_string_lossy()),
        };
        ctx.lua
            .globals()
            .get::<mlua::Table>("package")
            .into_diagnostic()?
            .set("path", lua_path)
            .into_diagnostic()?;
        */

        stdlib::setup_tag_functions(&ctx)?;
        stdlib::setup_stdlib(mode, &ctx)?;
        Ok(ctx)
    }

    /// Like [`Self::exec_lua`], but with sandboxing enabled[^sandbox].
    ///
    /// [^sandbox]: See: [`Lua::sandbox`]
    pub fn eval_template_lua<T: FromLuaMulti>(
        &self,
        name: &str,
        content: &str,
    ) -> Result<T, LuaError> {
        self.lua().sandbox(true)?;
        self.lua()
            .load(content)
            .set_name(name)
            .eval()
            .map_err(|e| LuaError::from_mlua_with_source(content, e))
    }

    /// Evaluate a lua expression.
    pub fn eval_lua<T: FromLuaMulti>(&self, name: &str, content: &str) -> Result<T, LuaError> {
        self.lua().sandbox(false)?;
        self.lua()
            .load(content)
            .set_name(name)
            .eval()
            .map_err(|e| LuaError::from_mlua_with_source(content, e))
    }

    /// Execute a bit of lua code.
    pub fn exec_lua(&self, name: &str, content: &str) -> Result<(), LuaError> {
        self.lua().sandbox(false)?;
        self.lua()
            .load(content)
            .set_name(name)
            .exec()
            .map_err(|e| LuaError::from_mlua_with_source(content, e))
    }

    pub fn eval_text_transformation(&self, text: &str, expr: &str) -> Result<String, LuaError> {
        let old_text = self.get_global::<Value>(YOLK_TEXT_NAME)?;
        self.set_global(YOLK_TEXT_NAME, text)?;
        let result = self.eval_template_lua("template tag", expr)?;
        self.set_global(YOLK_TEXT_NAME, old_text)?;
        Ok(result)
    }

    pub fn set_global<T: IntoLua>(&self, name: impl IntoLua, value: T) -> Result<(), LuaError> {
        Ok(self.lua.globals().set(name, value)?)
    }
    pub fn get_global<T: FromLua>(&self, name: impl IntoLua) -> Result<T, LuaError> {
        Ok(self.lua.globals().get::<T>(name)?)
    }

    pub fn register_fn<F, A, R>(&self, name: &str, func: F) -> Result<(), LuaError>
    where
        F: Fn(&Lua, A) -> Result<R, LuaError> + MaybeSend + 'static + Send + Sync,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        self.set_global(
            name,
            self.lua
                .create_function(move |lua, x| func(lua, x).into_lua_err())?,
        )
    }

    pub fn lua(&self) -> &Lua {
        &self.lua
    }
    pub fn lua_mut(&mut self) -> &mut Lua {
        &mut self.lua
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use mlua::Value;
    use testresult::TestResult;

    use super::{EvalCtx, EvalMode};

    #[test]
    pub fn test_globals_match_between_local_and_canonical() -> TestResult {
        let local = EvalCtx::new_in_mode(EvalMode::Local).unwrap();
        let canonical = EvalCtx::new_in_mode(EvalMode::Canonical).unwrap();
        let canonical_entries: HashSet<_> = canonical
            .lua
            .globals()
            .pairs::<Value, Value>()
            .map(|x| x.unwrap().0.to_string().unwrap())
            .collect();
        let local_entries: HashSet<_> = local
            .lua
            .globals()
            .pairs::<Value, Value>()
            .map(|x| x.unwrap().0.to_string().unwrap())
            .collect();
        assert_eq!(local_entries, canonical_entries);
        Ok(())
    }
}
