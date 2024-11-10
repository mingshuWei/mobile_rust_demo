use std::collections::HashMap;

pub use log::Log;

mod log;
pub fn set_logger(logger: Box<dyn Log>) -> anyhow::Result<()> {
    log::set_logger(logger)
}

pub fn hello_rust(config: MobileConfig) -> anyhow::Result<()> {
    println!("hello rust {:?}", config);
    log::logger(&config.username);
    // call some fn ?
    Ok(())
}

#[derive(Debug)]
pub struct MobileConfig {
    pub id: String,
    pub username: String,
    pub pwd: String,
    pub user_properties: Option<HashMap<String, String>>,
}
