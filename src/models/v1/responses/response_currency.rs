use serde::Serialize;

use crate::models::v1::errors::api_error::ApiError;

#[derive(Serialize)]
pub struct ResponseCurrency {
    pub id: i32,
    pub name: String
}


#[derive(Serialize)]
pub struct ResponseCurrencyPayload {
    pub data: Option<ResponseCurrency>,
    pub error: Option<ApiError>
}

#[derive(Serialize)]
pub struct ResponseCurrenciesPayload {
    pub data: Option<Vec<ResponseCurrency>>,
    pub total: Option<i64>,
    pub error: Option<ApiError>
}