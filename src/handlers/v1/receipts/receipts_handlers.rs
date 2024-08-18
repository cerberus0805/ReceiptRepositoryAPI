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
        commands::writer_command::WriterCommand, 
        errors::api_error::ApiError, 
        forms::{
            create_payload::CreateReceiptPayload, 
            patch_payload::PatchReceiptPayload
        }, 
        parameters::pagination::Pagination, 
        responses::response_receipt::{
            ResponseReceiptPayload, 
            ResponseReceiptsPayload
        }
    }, 
    services::v1::{
        converters::api_error_converter_service::ApiErrorConventerService, 
        receipts::receipts_service::ReceiptService
    }, share_state::HandlerState
};

pub struct ReceiptsHandlers {
}

impl ReceiptsHandlers {
    pub async fn get_receipt(State(handler_state): State<HandlerState>, id: Result<Path<u32>, PathRejection>) -> impl IntoResponse {
        let service = ReceiptService::new(&handler_state.repository);
        if let Ok(r_id) = id {
            let response_receipt = service.get_receipt(r_id.0 as i32).await;
            match response_receipt {
                Ok(response) => {
                    let payload = ResponseReceiptPayload {
                        data: Some(response),
                        error: None
                    };
            
                    (StatusCode::OK, Json(payload))
                },
                Err(e) => {
                    let api_error_converter_service = ApiErrorConventerService::new();
                    let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);

                    let payload = ResponseReceiptPayload {
                        data: None,
                        error: Some(e)
                    };
                    (http_return_code, Json(payload))
                }
            }
        }
        else {
            let payload = ResponseReceiptPayload {
                data: None,
                error: Some(ApiError::InvalidParameter)
            };
            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }

    pub async fn get_receipts(State(handler_state): State<HandlerState>, pagination: Option<Query<Pagination>>) -> impl IntoResponse {
        let service = ReceiptService::new(&handler_state.repository);
        let receipt_collection = service.get_receipts(&pagination.unwrap_or_default().0).await;
        match receipt_collection {
            Ok(responses) => {
                let payload = ResponseReceiptsPayload {
                    data: Some(responses.partial_collection),
                    total: Some(responses.total_count),
                    error: None
                };
                (StatusCode::OK, Json(payload))
            },
            Err(e) => {
                let api_error_converter_service = ApiErrorConventerService::new();
                let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);
                let payload = ResponseReceiptsPayload {
                    data: None,
                    total: None,
                    error: Some(e)
                };
                (http_return_code, Json(payload))
            }
        }
    }

    pub async fn post_receipt(State(handler_state): State<HandlerState>,  payload: Result<Json<CreateReceiptPayload>, JsonRejection>) -> impl IntoResponse {
        if let Ok(r_payload) = payload { 
            let create_command = WriterCommand::CreateReceipt(r_payload.0.clone());
            let _ = handler_state.sender.send(create_command).await;
            let response = ResponseReceiptPayload {
                data: None,
                error: None
            };
            (StatusCode::ACCEPTED, Json(response))
        }
        else {
            let payload = ResponseReceiptPayload {
                data: None,
                error: Some(ApiError::InvalidParameter)
            };
            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }

    pub async fn patch_receipt(State(handler_state): State<HandlerState>, id: Result<Path<u32>, PathRejection>,  payload: Result<Json<PatchReceiptPayload>, JsonRejection>) -> impl IntoResponse {
        if id.is_ok() && payload.is_ok() {
            let r_id = id.expect("id should be ok after we have checked").0;
            let r_payload = payload.expect("payload should be ok after we have checked").0;
            let service = ReceiptService::new(&handler_state.repository);
            match service.patch_receipt(r_id as i32, &r_payload).await {
                Ok(_) => {
                    let payload = ResponseReceiptPayload {
                        data: None,
                        error: None
                    };
        
                    (StatusCode::OK, Json(payload))
                },
                Err(e) => {
                    let api_error_converter_service = ApiErrorConventerService::new();
                    let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);
                    let payload = ResponseReceiptPayload {
                        data: None,
                        error: Some(e)
                    };
                    (http_return_code, Json(payload))
                }
            }
        }
        else {
            let payload = ResponseReceiptPayload {
                data: None,
                error: Some(ApiError::InvalidParameter)
            };

            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }

    pub async fn delete_receipt(State(handler_state): State<HandlerState>, id: Result<Path<u32>, PathRejection>) -> impl IntoResponse {
        if let Ok(r_id) = id {
            let delete_command = WriterCommand::DeleteReceipt(r_id.0 as i32);
            let _ = handler_state.sender.send(delete_command).await;
            let response = ResponseReceiptPayload {
                data: None,
                error: None
            };
            (StatusCode::ACCEPTED, Json(response))
        }
        else {
            let payload = ResponseReceiptPayload {
                data: None,
                error: Some(ApiError::InvalidParameter)
            };

            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }
}