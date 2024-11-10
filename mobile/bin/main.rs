use mobile::{hello_rust, set_logger, Log, MobileConfig};

struct LogImpl();

impl Log for LogImpl {
    fn log(&self, msg: &str) {
        println!("rust log {}", msg);
    }
}

fn main() -> anyhow::Result<()> {
    set_logger(Box::new(LogImpl()))?;
    let config = MobileConfig {
        id: "".to_string(),
        username: "".to_string(),
        pwd: "".to_string(),
        user_properties: None,
    };
    hello_rust(config)?;
    Ok(())
}
