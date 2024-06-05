pub struct AppConfig {
    host: String,
    port: u16
}

impl AppConfig {
    pub fn new() -> Self {
        // TODO: extract settings from other config files or environment
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000
        }
    }

    pub fn get_address(self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
