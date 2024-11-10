use std::sync::OnceLock;

use anyhow::anyhow;

static LOGGER: OnceLock<Box<dyn Log>> = OnceLock::new();

pub trait Log: Send + Sync {
    fn log(&self, msg: &str);
}

pub fn set_logger(logger: Box<dyn Log>) -> anyhow::Result<()> {
    LOGGER
        .set(logger)
        .map_err(|e| anyhow!("set logger failed"))?;
    Ok(())
}

pub fn logger(msg: &str) {
    LOGGER.get().map(|log| {
        log.log(msg);
    });
}
