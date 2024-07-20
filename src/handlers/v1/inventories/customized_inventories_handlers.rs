use axum::{extract::{rejection::PathRejection, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{
    models::v1::{errors::api_error::ApiError, parameters::pagination::Pagination, responses::response_inventory::{ResponseCustomizedInventoryPayload, ResponseCustomizedInventoriesPayload}}, repository::DbRepository, services::v1::{converters::api_error_converter_service::ApiErrorConventerService, inventories::{customized_inventories_service::CustomizedInventroyService}}
};

pub struct CustomizedInventoriesHandlers {
}

impl CustomizedInventoriesHandlers {
    pub async fn get_customized_inventory(State(repository): State<DbRepository>, id: Result<Path<u32>, PathRejection>) -> impl IntoResponse {
        let service = CustomizedInventroyService::new(repository);
        let api_error_converter_service = ApiErrorConventerService::new();
        if let Ok(r_id) = id {
            let response_inventory = service.get_customized_inventory(r_id.0 as i32).await;
            match response_inventory {
                Ok(response) => {
                    let payload = ResponseCustomizedInventoryPayload {
                        data: Some(response),
                        error: None
                    };
            
                    (StatusCode::OK, Json(payload))
                },
                Err(e) => {
                    let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);

                    let payload = ResponseCustomizedInventoryPayload {
                        data: None,
                        error: Some(e)
                    };
                    (http_return_code, Json(payload))
                }
            }
        }
        else {
            let payload = ResponseCustomizedInventoryPayload {
                data: None,
                error: Some(ApiError::InvalidParameter)
            };
            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }

    pub async fn get_customized_inventories(State(repository): State<DbRepository>, pagination: Option<Query<Pagination>>) -> impl IntoResponse {
        let service = CustomizedInventroyService::new(repository);
        let api_error_converter_service = ApiErrorConventerService::new();
        let inventories_collection = service.get_customized_inventories(pagination.unwrap_or_default().0).await;
        match inventories_collection {
            Ok(responses) => {
                let payload = ResponseCustomizedInventoriesPayload {
                    data: Some(responses.partial_collection),
                    total: Some(responses.total_count),
                    error: None
                };
                (StatusCode::OK, Json(payload))
            },
            Err(e) => {
                let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);
                let payload = ResponseCustomizedInventoriesPayload {
                    data: None,
                    total: None,
                    error: Some(e)
                };
                (http_return_code, Json(payload))
            }
        }
    }
}