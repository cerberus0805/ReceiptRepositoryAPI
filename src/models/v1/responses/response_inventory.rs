use crate::models::v1::errors::api_error::ApiError;

use super::response_product::ResponseProduct;
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