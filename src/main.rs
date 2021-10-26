use crate::context::OCContext;

mod logging;
mod context;
fn main() {
    logging::setup_logging();
    log::info!(target: "Main thread", "Logger!");
    let mut ctx = OCContext::new().unwrap();
    ctx.exec(r#"
    print(computer.totalMemory())
    "#).unwrap();
    println!("Hello, world!");
}
