use axum::{
    extract::{
        Path, 
        State,
        Query,
        rejection::PathRejection
    }, 
    http::StatusCode, 
    response::IntoResponse, Json
};

use crate::{
    models::v1::{errors::api_error::ApiError, parameters::pagination::Pagination, responses::response_receipt::{ReponseReceiptPayload, ReponseReceiptsPayload}}, repository::DbRepository, services::v1::receipts::receipts_service::ReceiptService
};

pub struct ReceiptsHandlers {
}

impl ReceiptsHandlers {
    pub async fn get_receipt(State(repository): State<DbRepository>, id: Result<Path<u32>, PathRejection>) -> impl IntoResponse {
        let service = ReceiptService::new(repository);
        if let Ok(r_id) = id {
            let response_receipt = service.get_receipt(r_id.0 as i32).await;
            match response_receipt {
                Ok(response) => {
                    let payload = ReponseReceiptPayload {
                        data: Some(response),
                        error: None
                    };
            
                    (StatusCode::OK, Json(payload))
                },
                Err(e) => {
                    let http_return_code = Self::get_http_status_from_api_error(&e);

                    let payload = ReponseReceiptPayload {
                        data: None,
                        error: Some(e)
                    };
                    (http_return_code, Json(payload))
                }
            }
        }
        else {
            let payload = ReponseReceiptPayload {
                data: None,
                error: Some(ApiError::InvalidParameter)
            };
            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }

    pub async fn get_receipts(State(repository): State<DbRepository>, pagination: Option<Query<Pagination>>) -> impl IntoResponse {
        let service = ReceiptService::new(repository);
        let receipt_collection = service.get_receipts(pagination.unwrap_or_default().0).await;
        match receipt_collection {
            Ok(responses) => {
                let payload = ReponseReceiptsPayload {
                    data: Some(responses.partial_collection),
                    total: Some(responses.total_count),
                    error: None
                };
                (StatusCode::OK, Json(payload))
            },
            Err(e) => {
                let http_return_code = Self::get_http_status_from_api_error(&e);
                let payload = ReponseReceiptsPayload {
                    data: None,
                    total: None,
                    error: Some(e)
                };
                (http_return_code, Json(payload))
            }
        }
    }

    fn get_http_status_from_api_error(error: &ApiError) -> StatusCode {
        match error {
            &ApiError::DatabaseConnectionBroken => StatusCode::INTERNAL_SERVER_ERROR,
            &ApiError::NoRecord => StatusCode::NOT_FOUND,
            &ApiError::InvalidParameter => StatusCode::BAD_REQUEST,
            &ApiError::Generic => StatusCode::NOT_ACCEPTABLE
        }
    }
}