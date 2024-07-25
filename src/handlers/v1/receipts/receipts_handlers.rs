use axum::{
    extract::{
        Path, 
        State,
        Query,
        rejection::{PathRejection, JsonRejection}
    }, 
    http::StatusCode, 
    response::IntoResponse, Json
};

use crate::{
    models::v1::{
        errors::api_error::ApiError, 
        forms::create_payload::CreateReceiptPayload, 
        parameters::pagination::Pagination, 
        responses::response_receipt::{
            ReponseReceiptPayload, 
            ReponseReceiptsPayload, 
            ResponseCreateReceiptPayload
        }
    }, 
    repository::DbRepository, 
    services::v1::{
        converters::api_error_converter_service::ApiErrorConventerService, 
        receipts::receipts_service::ReceiptService
    }
};

pub struct ReceiptsHandlers {
}

impl ReceiptsHandlers {
    pub async fn get_receipt(State(repository): State<DbRepository>, id: Result<Path<u32>, PathRejection>) -> impl IntoResponse {
        let service = ReceiptService::new(&repository);
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
                    let api_error_converter_service = ApiErrorConventerService::new();
                    let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);

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
        let service = ReceiptService::new(&repository);
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
                let api_error_converter_service = ApiErrorConventerService::new();
                let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);
                let payload = ReponseReceiptsPayload {
                    data: None,
                    total: None,
                    error: Some(e)
                };
                (http_return_code, Json(payload))
            }
        }
    }

    pub async fn post_receipt(State(repository): State<DbRepository>,  payload: Result<Json<CreateReceiptPayload>, JsonRejection>) -> impl IntoResponse {
        if let Ok(r_payload) = payload { 
            let service = ReceiptService::new(&repository);
            let respone_create_receipt = service.create_receipt(&r_payload.0).await;
            match respone_create_receipt {
                Ok(response) => {
                    let payload = ResponseCreateReceiptPayload {
                        data: Some(response),
                        error: None
                    };
                    (StatusCode::CREATED, Json(payload))
                },
                Err(e) => {
                    let api_error_converter_service = ApiErrorConventerService::new();
                    let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);
                    let payload = ResponseCreateReceiptPayload {
                        data: None,
                        error: Some(e)
                    };
                    (http_return_code, Json(payload))
                }
            }
        }
        else {
            let payload = ResponseCreateReceiptPayload {
                data: None,
                error: Some(ApiError::InvalidParameter)
            };
            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }
}