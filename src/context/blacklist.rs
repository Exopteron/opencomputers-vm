use rlua::{Table, Value::Nil};

pub fn blacklist_globals(globals: &Table) -> anyhow::Result<()> {
    globals.set("io", Nil)?;
    //globals.set("print", Nil)?;
    globals.set("os", Nil)?;
    Ok(())
}