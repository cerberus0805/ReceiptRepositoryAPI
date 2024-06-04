use std::error::Error;

pub struct Config {
    pub host: String,
    pub port: u16
}

pub struct  ConfigBuilder {}

impl ConfigBuilder {
    pub fn get_config() -> Result<Config, Box<dyn Error>> {
        Ok(Config {
            host: "0.0.0.0".to_string(),
            port: 3000
        })
    }
}

