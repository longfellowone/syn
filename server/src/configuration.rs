use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub server: Server,
}

impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        let mut c = Config::default();

        c.merge(File::with_name("Configuration")).unwrap();
        // c.merge(Environment::new().separator("_")).unwrap();

        c.try_into()
    }
}

#[derive(Debug, Deserialize)]
pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
