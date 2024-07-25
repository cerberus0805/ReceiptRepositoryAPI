use axum::http::StatusCode;

use crate::models::v1::errors::api_error::ApiError;

pub struct ApiErrorConventerService {
}

impl ApiErrorConventerService {
    pub fn new() -> Self {
        Self {
        }
    }
    pub fn get_http_status_from_api_error(&self, error: &ApiError) -> StatusCode {
        match error {
            &ApiError::DatabaseConnectionBroken => StatusCode::INTERNAL_SERVER_ERROR,
            &ApiError::NoRecord => StatusCode::NOT_FOUND,
            &ApiError::InvalidParameter => StatusCode::BAD_REQUEST,
            &ApiError::Generic => StatusCode::NOT_ACCEPTABLE,
            &ApiError::FormFieldCurrencyInvalid => StatusCode::BAD_REQUEST,
            &ApiError::FormFieldCurrencyIdNotExisted => StatusCode::BAD_REQUEST,
            &ApiError::FormFieldCurrencyNameDuplicated => StatusCode::BAD_REQUEST,
            &ApiError::FormFieldStoreInvalid => StatusCode::BAD_REQUEST,
            &ApiError::FormFieldStoreIdNotExisted => StatusCode::BAD_REQUEST,
            &ApiError::FormFieldStoreNameDuplicated => StatusCode::BAD_REQUEST,
            &ApiError::FormFieldProductInvalid => StatusCode::BAD_REQUEST,
            &ApiError::FormFieldProductIdNotExisted => StatusCode::BAD_REQUEST,
            &ApiError::FormFieldProductNameDuplicated => StatusCode::BAD_REQUEST,
            &ApiError::InsertCurrencyFailed => StatusCode::CONFLICT,
            &ApiError::InsertStoreFailed => StatusCode::CONFLICT,
            &ApiError::InsertProductFailed => StatusCode::CONFLICT,
            &ApiError::InsertReceiptFailed => StatusCode::CONFLICT,
            &ApiError::InsertInventoryFailed => StatusCode::CONFLICT,
        }
    }
}