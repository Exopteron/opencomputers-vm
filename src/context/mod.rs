use std::{cell::RefCell, sync::{Arc, Mutex, MutexGuard}};

use rlua::{Lua, Value::Nil};

use crate::context::api::default_apis;

use self::api::OCAPI;
mod api;
mod blacklist;
#[derive(Clone)]
pub struct OCContext {
    pub lua: Arc<Mutex<Lua>>,
    context: Arc<Mutex<InternalCtx>>,
}
pub struct InternalCtx {
    pub mem_limit: usize,
}
impl OCContext {
    pub fn new() -> anyhow::Result<Self> {
        log::debug!(target: "OC Context", "Created context");
        let lua = Lua::new();
        let mem_limit = 1024 * 1000;
        lua.set_memory_limit(Some(mem_limit));
        lua.context::<_, anyhow::Result<()>>(|lua_ctx| {
            let globals = lua_ctx.globals();
            blacklist::blacklist_globals(&globals)?;
            Ok(())
        })?;
        let self_obj = Self {
            context: Arc::new(Mutex::new(InternalCtx { mem_limit: mem_limit })),
            lua: Arc::new(Mutex::new(lua)),
        };
        default_apis(self_obj.clone())?;
        Ok(self_obj)
    }
    pub fn set_memory_limit(&self, num: usize) -> anyhow::Result<()> {
        let mut ctx = self.context.lock().unwrap();
        ctx.mem_limit = num;
        drop(ctx);
        let lua = self.lua.lock().unwrap();
        lua.set_memory_limit(Some(num));
        Ok(())
    }
    pub fn get_memory_limit(&self) -> usize {
        self.context.lock().unwrap().mem_limit
    }
    pub fn get_internal(&mut self) -> MutexGuard<'_, InternalCtx> {
        self.context.lock().unwrap()
    }
    pub fn exec(&mut self, src: &str) -> anyhow::Result<()> {
        log::debug!(target: "OC Context", "Executing in context");
        self.lua.lock().unwrap().context::<_, anyhow::Result<()>>(|lua_ctx| {
            lua_ctx.load(src).exec()?;
            Ok(())
        })?;
        Ok(())
    }
}
