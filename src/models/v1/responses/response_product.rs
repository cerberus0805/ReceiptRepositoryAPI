use serde::Serialize;

use crate::models::v1::errors::api_error::ApiError;

#[derive(Serialize)]
pub struct ResponseProduct {
    pub id: i32,
    pub name: String,
    pub alias: Option<String>,
    pub specification_amount: Option<i32>,
    pub specification_unit: Option<String>,
    pub specification_others: Option<String>,
    pub brand: Option<String>
}

#[derive(Serialize)]
pub struct ResponseProductPayload {
    pub data: Option<ResponseProduct>,
    pub error: Option<ApiError>
}

#[derive(Serialize)]
pub struct ResponseProductsPayload {
    pub data: Option<Vec<ResponseProduct>>,
    pub total: Option<i64>,
    pub error: Option<ApiError>
}
