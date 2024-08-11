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
            &ApiError::CurrencyInvalid => StatusCode::BAD_REQUEST,
            &ApiError::CurrencyIdNotExisted => StatusCode::BAD_REQUEST,
            &ApiError::CurrencyNameDuplicated => StatusCode::BAD_REQUEST,
            &ApiError::StoreInvalid => StatusCode::BAD_REQUEST,
            &ApiError::StoreIdNotExisted => StatusCode::BAD_REQUEST,
            &ApiError::StoreNameDuplicated => StatusCode::BAD_REQUEST,
            &ApiError::ProductInvalid => StatusCode::BAD_REQUEST,
            &ApiError::ProductIdNotExisted => StatusCode::BAD_REQUEST,
            &ApiError::ProductNameDuplicated => StatusCode::BAD_REQUEST,
            &ApiError::InsertCurrencyFailed => StatusCode::CONFLICT,
            &ApiError::InsertStoreFailed => StatusCode::CONFLICT,
            &ApiError::InsertProductFailed => StatusCode::CONFLICT,
            &ApiError::InsertReceiptFailed => StatusCode::CONFLICT,
            &ApiError::InsertInventoryFailed => StatusCode::CONFLICT,
            &ApiError::UpdateCurrencyFailed => StatusCode::UNPROCESSABLE_ENTITY,
            &ApiError::UpdateStoreFailed => StatusCode::UNPROCESSABLE_ENTITY,
            &ApiError::UpdateProductFailed => StatusCode::UNPROCESSABLE_ENTITY,
            &ApiError::UpdateReceiptFailed => StatusCode::UNPROCESSABLE_ENTITY,
            &ApiError::UpdateInventoryFailed => StatusCode::UNPROCESSABLE_ENTITY,
            &ApiError::DeleteReceiptIdNotExisted => StatusCode::GONE,
            &ApiError::DeleteReceiptAssociatedEntryFailed => StatusCode::NOT_ACCEPTABLE,
            &ApiError::DeleteReceiptEntryFailed => StatusCode::UNPROCESSABLE_ENTITY,
            &ApiError::DeleteReceiptRelatedEntryFailed => StatusCode::GONE
        }
    }
}