use std::{collections::HashMap, fmt};

uniffi::setup_scaffolding!();

#[derive(uniffi::Error, Debug)]
pub enum MobileError {
    Generic(String),
}
impl fmt::Display for MobileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MobileError::Generic(err) => write!(f, "error: {}", err),
        }
    }
}


#[uniffi::export(callback_interface)]
pub trait FFILog: Send + Sync {
    fn log(&self, msg: String);
}

struct LogImpl {
    logger: Box<dyn FFILog>,
}

impl mobile::Log for LogImpl {
    fn log(&self, msg: &str) {
        self.logger.log(msg.to_string())
    }
}

#[derive(uniffi::Record)]
pub struct MobileConfig {
    pub id: String,
    pub username: String,
    pub pwd: String,
    pub user_properties: Option<HashMap<String, String>>,
}

impl MobileConfig {
    fn to_inner_config(self) -> mobile::MobileConfig {
        mobile::MobileConfig {
            id: self.id,
            username: self.username,
            pwd: self.pwd,
            user_properties: self.user_properties,
        }
    }
}

#[uniffi::export]
pub fn set_logger(logger: Box<dyn FFILog>) -> Result<(), MobileError> {
    mobile::set_logger(Box::new(LogImpl { logger }))
        .map_err(|e| MobileError::Generic(e.to_string()))
}

#[uniffi::export]
pub fn hello_rust(config: MobileConfig) -> Result<(), MobileError> {
    mobile::hello_rust(config.to_inner_config()).map_err(|e| MobileError::Generic(e.to_string()))
}


#[cfg(test)]
mod tests {
   
}
