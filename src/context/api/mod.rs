use self::computer::ComputerAPI;

use super::OCContext;

mod computer;
pub trait OCAPI {
    fn impl_on_context(ctx: OCContext) -> anyhow::Result<()>;
}

pub fn default_apis(ctx: OCContext) -> anyhow::Result<()> {
    ComputerAPI::impl_on_context(ctx)?;
    Ok(())
}