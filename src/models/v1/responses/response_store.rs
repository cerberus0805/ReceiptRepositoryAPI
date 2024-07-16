use serde::Serialize;

use crate::models::v1::errors::api_error::ApiError;

#[derive(Serialize)]
pub struct ResponseStore {
    pub id: i32,
    pub name: String,
    pub alias: Option<String>,
    pub branch: Option<String>,
    pub address: Option<String>
}

#[derive(Serialize)]
pub struct ResponseStorePayload {
    pub data: Option<ResponseStore>,
    pub error: Option<ApiError>
}

#[derive(Serialize)]
pub struct ResponseStoresPayload {
    pub data: Option<Vec<ResponseStore>>,
    pub total: Option<i64>,
    pub error: Option<ApiError>
}
