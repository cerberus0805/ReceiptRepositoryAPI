use axum::{extract::{rejection::{JsonRejection, PathRejection}, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{models::v1::{errors::api_error::ApiError, forms::patch_payload::PatchStorePayload, parameters::pagination::Pagination, responses::response_store::{ResponseStorePayload, ResponseStoresPayload}}, repository::DbRepository, services::v1::{converters::api_error_converter_service::ApiErrorConventerService, stores::stores_service::StoreService}};


pub struct StoresHandlers {   
}

impl StoresHandlers {
    pub async fn get_store(State(repository): State<DbRepository>, id: Result<Path<u32>, PathRejection>) -> impl IntoResponse {
        let service = StoreService::new(&repository);
        if let Ok(s_id) = id {
            let response_store = service.get_store(s_id.0 as i32).await;
            match response_store {
                Ok(response) => {
                    let payload = ResponseStorePayload {
                        data: Some(response),
                        error: None
                    };
                    (StatusCode::OK, Json(payload))
                },
                Err(e) => {
                    let api_error_converter_service = ApiErrorConventerService::new();
                    let http_status_code = api_error_converter_service.get_http_status_from_api_error(&e);

                    let payload = ResponseStorePayload {
                        data: None,
                        error: Some(e)
                    };
                    (http_status_code, Json(payload))
                }
            }
        }
        else {
            let payload = ResponseStorePayload {
                data: None,
                error: Some(ApiError::InvalidParameter)
            };

            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }

    pub async fn get_stores(State(repository): State<DbRepository>, pagination: Option<Query<Pagination>>) -> impl IntoResponse {
        let service = StoreService::new(&repository);
        let store_collection = service.get_stores(&pagination.unwrap_or_default().0).await;
        match store_collection {
            Ok(responses) => {
                let payload = ResponseStoresPayload {
                    data: Some(responses.partial_collection),
                    total: Some(responses.total_count),
                    error: None
                };
                (StatusCode::OK, Json(payload))
            },
            Err(e) => {
                let api_error_converter_service = ApiErrorConventerService::new();
                let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);
                let payload = ResponseStoresPayload {
                    data: None,
                    total: None,
                    error: Some(e)
                };
                (http_return_code, Json(payload))
            }
        }
    }

    pub async fn patch_store(State(repository): State<DbRepository>, id: Result<Path<u32>, PathRejection>,  payload: Result<Json<PatchStorePayload>, JsonRejection>) -> impl IntoResponse {
        if id.is_ok() && payload.is_ok() {
            let s_id = id.expect("id should be ok after we have checked").0;
            let s_payload = payload.expect("payload should be ok after we have checked").0;
            let service = StoreService::new(&repository);
            match service.patch_store(s_id as i32, &s_payload).await {
                Ok(_) => {
                    let payload = ResponseStorePayload {
                        data: None,
                        error: None
                    };
        
                    (StatusCode::OK, Json(payload))
                },
                Err(e) => {
                    let api_error_converter_service = ApiErrorConventerService::new();
                    let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);
                    let payload = ResponseStorePayload {
                        data: None,
                        error: Some(e)
                    };
                    (http_return_code, Json(payload))
                }
            }
        }
        else {
            let payload = ResponseStorePayload {
                data: None,
                error: Some(ApiError::InvalidParameter)
            };

            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }
}