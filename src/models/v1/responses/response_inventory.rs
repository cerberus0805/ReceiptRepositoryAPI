use crate::models::v1::errors::api_error::ApiError;

use super::{response_currency::ResponseCurrency, response_product::ResponseProduct};
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseInventory {
    pub id: i32,
    pub product: ResponseProduct,
    pub price: f64,
    pub quantity: i32
}


#[derive(Serialize)]
pub struct ResponseInventoryPayload {
    pub data: Option<ResponseInventory>,
    pub error: Option<ApiError>
}

#[derive(Serialize)]
pub struct ResponseInventoriesPayload {
    pub data: Option<Vec<ResponseInventory>>,
    pub total: Option<i64>,
    pub error: Option<ApiError>
}

#[derive(Serialize)]
pub struct ResponseCustomizedInventory {
    pub id: i32,
    pub product: ResponseProduct,
    pub price: f64,
    pub receipt_id: i32,
    pub taxed: bool,
    pub transaction_date: NaiveDateTime,
    pub store_id: i32,
    pub store_name: String,
    pub store_alias: Option<String>,
    pub currency: ResponseCurrency
}

#[derive(Serialize)]
pub struct ResponseCustomizedInventoryPayload {
    pub data: Option<ResponseCustomizedInventory>,
    pub error: Option<ApiError>
}

#[derive(Serialize)]
pub struct ResponseCustomizedInventoriesPayload {
    pub data: Option<Vec<ResponseCustomizedInventory>>,
    pub total: Option<i64>,
    pub error: Option<ApiError>
}