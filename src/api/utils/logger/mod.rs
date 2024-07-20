use anyhow::Result;

pub fn init_logger() -> Result<()> {
    env_logger::init();

    Ok(())
}
