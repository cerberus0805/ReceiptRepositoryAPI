use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub pwd: String
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub error: Option<String>
}
