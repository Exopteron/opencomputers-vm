use super::OCAPI;

pub struct ComputerAPI {}
impl OCAPI for ComputerAPI {
    fn impl_on_context(ctx: crate::context::OCContext) -> anyhow::Result<()> {
        let ctx_i = ctx.clone();
        ctx.lua.lock().unwrap().context::<_, anyhow::Result<()>>(|lua_ctx| {
            let computer_object = lua_ctx.create_table()?;

            
            
            
            
            let total_memory = lua_ctx.create_function(move |_, (): ()| {
                let ctx = ctx_i.clone();
                Ok(ctx.get_memory_limit())
            })?;
            computer_object.set("totalMemory", total_memory)?;
            
            
            
            
            lua_ctx.globals().set("computer", computer_object)?;
            Ok(())
        })?;
        Ok(())
    }
} 