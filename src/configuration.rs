use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct AppConfig {
    host: String,
    port: u16,
    db_url: String
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exised in environment variable").to_string();
        // TODO: extract settings from other config files or environment
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
            db_url
        }
    }

    pub fn get_address(self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn get_db_url(self) -> String {
        self.db_url
    }
}
