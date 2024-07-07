use std::env;

pub struct AppConfig {
    host: String,
    port: u16,
    db_url: String
}

impl AppConfig {
    pub fn new() -> Self {
        let host = env::var("BIND_ADDR").expect("BIND_ADDR is not exised in environment variable").to_string();
        let port: u16 = env::var("BIND_PORT").expect("BIND_ADDR is not exised in environment variable").to_string().parse().expect("Convert env port to u16 failed");
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exised in environment variable").to_string();
        Self {
            host, 
            port,
            db_url
        }
    }

    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn get_db_url(&self) -> String {
        self.db_url.to_string()
    }

    pub fn log_to_file(&self) -> bool {
        let flag_str = env::var("LOG_TO_FILE").unwrap_or_else(|_| "0".to_string());
        flag_str != "0"
    }

    pub fn get_log_directory(&self) -> String {
        let log_directory = env::var("LOG_DIRECTORY").unwrap_or_else(|_| ".".to_string());
        log_directory
    }

    pub fn get_log_prefix(&self) -> String {
        let log_prefix = env::var("LOG_PREFIX").unwrap_or_else(|_| "receipt_repository_api".to_string());
        log_prefix
    }
}
