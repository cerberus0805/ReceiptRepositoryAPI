use std::env;
use dotenvy::dotenv;

pub struct AppConfig {
    host: String,
    port: u16,
    db_url: String
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv().ok();
        let host = env::var("BIND_ADDR").unwrap_or("0.0.0.0".to_string()).to_string();
        let port: u16 = env::var("BIND_PORT").unwrap_or("3000".to_string()).to_string().parse().expect("Convert env port to u16 failed");
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

    pub fn get_log_filter(&self) -> String {
        let log_filter = env::var("RUST_LOG").unwrap_or("receipt_repository_api=debug,tower_http=debug,axum::rejection=trace".to_string());
        log_filter
    }

    pub fn log_to_file(&self) -> bool {
        let flag_str = env::var("LOG_TO_FILE").unwrap_or("0".to_string());
        flag_str != "0"
    }

    pub fn get_log_directory(&self) -> String {
        let log_directory = env::var("LOG_DIRECTORY").unwrap_or(".".to_string());
        log_directory
    }

    pub fn get_log_prefix(&self) -> String {
        let log_prefix = env::var("LOG_PREFIX").unwrap_or("receipt_repository_api".to_string());
        log_prefix
    }

    pub fn get_writer_channel_buffer_size(&self) -> usize {
        let buffer_size: usize = env::var("WRITER_CHANNEL_BUFFER_SIZE").unwrap_or("128".to_string()).parse().expect("Convert channel buffer size to usize failed");
        buffer_size
    }

    pub fn get_allow_origins(&self) -> Vec<String> {
        let allow_origins_str = env::var("ALLOW_ORIGINS").unwrap_or("".to_string());
        let allow_origin_vec = allow_origins_str.split(",").map(|o| { o.to_string() }).collect::<Vec<String>>();
        allow_origin_vec
    }
}
