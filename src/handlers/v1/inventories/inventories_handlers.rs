use axum::{extract::{rejection::{JsonRejection, PathRejection}, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{
    models::v1::{errors::api_error::ApiError, forms::patch_payload::PatchInventoryPayload, parameters::pagination::Pagination, responses::response_inventory::{ResponseInventoriesPayload, ResponseInventoryPayload}}, repository::DbRepository, services::v1::{converters::api_error_converter_service::ApiErrorConventerService, inventories::inventories_service::InventroyService}
};

pub struct InventoriesHandlers {
}

impl InventoriesHandlers {
    pub async fn get_inventory(State(repository): State<DbRepository>, id: Result<Path<u32>, PathRejection>) -> impl IntoResponse {
        let service = InventroyService::new(&repository);
        if let Ok(i_id) = id {
            let response_inventory = service.get_inventory(i_id.0 as i32).await;
            match response_inventory {
                Ok(response) => {
                    let payload = ResponseInventoryPayload {
                        data: Some(response),
                        error: None
                    };
            
                    (StatusCode::OK, Json(payload))
                },
                Err(e) => {
                    let api_error_converter_service = ApiErrorConventerService::new();
                    let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);

                    let payload = ResponseInventoryPayload {
                        data: None,
                        error: Some(e)
                    };
                    (http_return_code, Json(payload))
                }
            }
        }
        else {
            let payload = ResponseInventoryPayload {
                data: None,
                error: Some(ApiError::InvalidParameter)
            };
            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }

    pub async fn get_inventories(State(repository): State<DbRepository>, pagination: Option<Query<Pagination>>) -> impl IntoResponse {
        let service = InventroyService::new(&repository);
        let inventory_collection = service.get_inventories(&pagination.unwrap_or_default().0).await;
        match inventory_collection {
            Ok(responses) => {
                let payload = ResponseInventoriesPayload {
                    data: Some(responses.partial_collection),
                    total: Some(responses.total_count),
                    error: None
                };
                (StatusCode::OK, Json(payload))
            },
            Err(e) => {
                let api_error_converter_service = ApiErrorConventerService::new();
                let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);
                let payload = ResponseInventoriesPayload {
                    data: None,
                    total: None,
                    error: Some(e)
                };
                (http_return_code, Json(payload))
            }
        }
    }

    pub async fn patch_inventory(State(repository): State<DbRepository>, id: Result<Path<u32>, PathRejection>,  payload: Result<Json<PatchInventoryPayload>, JsonRejection>) -> impl IntoResponse {
        if id.is_ok() && payload.is_ok() {
            let i_id = id.expect("id should be ok after we have checked").0;
            let i_payload = payload.expect("payload should be ok after we have checked").0;
            let service = InventroyService::new(&repository);
            match service.patch_inventory(i_id as i32, &i_payload).await {
                Ok(_) => {
                    let payload = ResponseInventoryPayload {
                        data: None,
                        error: None
                    };
        
                    (StatusCode::OK, Json(payload))
                },
                Err(e) => {
                    let api_error_converter_service = ApiErrorConventerService::new();
                    let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);
                    let payload = ResponseInventoryPayload {
                        data: None,
                        error: Some(e)
                    };
                    (http_return_code, Json(payload))
                }
            }
        }
        else {
            let payload = ResponseInventoryPayload {
                data: None,
                error: Some(ApiError::InvalidParameter)
            };

            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }
}