use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub server: Server,
}

impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        let mut c = Config::default();

        println!("{:?}", std::env::current_dir());

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
    pub fn addr(&self) -> (&str, u16) {
        (&self.host, self.port)
    }
}
