use axum::{extract::{rejection::PathRejection, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{
    models::v1::{errors::api_error::ApiError, parameters::pagination::Pagination, responses::response_inventory::{ResponseCustomizedInventoryPayload, ResponseCustomizedInventoriesPayload}}, repository::DbRepository, services::v1::{converters::api_error_converter_service::ApiErrorConventerService, inventories::{customized_inventories_service::CustomizedInventroyService}}
};

pub struct CustomizedInventoriesHandlers {
}

impl CustomizedInventoriesHandlers {
    pub async fn get_customized_inventory(State(repository): State<DbRepository>, id: Result<Path<u32>, PathRejection>) -> impl IntoResponse {
        let service = CustomizedInventroyService::new(repository);
        if let Ok(i_id) = id {
            let response_inventory = service.get_customized_inventory(i_id.0 as i32).await;
            match response_inventory {
                Ok(response) => {
                    let payload = ResponseCustomizedInventoryPayload {
                        data: Some(response),
                        error: None
                    };
            
                    (StatusCode::OK, Json(payload))
                },
                Err(e) => {
                    let api_error_converter_service = ApiErrorConventerService::new();
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
                let api_error_converter_service = ApiErrorConventerService::new();
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

    pub async fn get_customized_inventories_by_product_id(State(repository): State<DbRepository>, id: Result<Path<u32>, PathRejection>, pagination: Option<Query<Pagination>>) -> impl IntoResponse {
        let service = CustomizedInventroyService::new(repository);
        if let Ok(p_id) = id {
            let inventories_collection = service.get_customized_inventories_by_product_id(p_id.0 as i32, pagination.unwrap_or_default().0).await;
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
                    let api_error_converter_service = ApiErrorConventerService::new();
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
        else {
            let payload = ResponseCustomizedInventoriesPayload {
                data: None,
                total: None,
                error: Some(ApiError::InvalidParameter)
            };
            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }

    pub async fn get_customized_inventories_by_receipt_id(State(repository): State<DbRepository>, id: Result<Path<u32>, PathRejection>, pagination: Option<Query<Pagination>>) -> impl IntoResponse {
        let service = CustomizedInventroyService::new(repository);
        if let Ok(r_id) = id {
            let inventories_collection = service.get_customized_inventories_by_receipt_id(r_id.0 as i32, pagination.unwrap_or_default().0).await;
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
                    let api_error_converter_service = ApiErrorConventerService::new();
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
        else {
            let payload = ResponseCustomizedInventoriesPayload {
                data: None,
                total: None,
                error: Some(ApiError::InvalidParameter)
            };
            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }

    pub async fn get_customized_inventories_by_store_id(State(repository): State<DbRepository>, id: Result<Path<u32>, PathRejection>, pagination: Option<Query<Pagination>>) -> impl IntoResponse {
        let service = CustomizedInventroyService::new(repository);
        if let Ok(s_id) = id {
            let inventories_collection = service.get_customized_inventories_by_store_id(s_id.0 as i32, pagination.unwrap_or_default().0).await;
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
                    let api_error_converter_service = ApiErrorConventerService::new();
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
        else {
            let payload = ResponseCustomizedInventoriesPayload {
                data: None,
                total: None,
                error: Some(ApiError::InvalidParameter)
            };
            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }

    pub async fn get_customized_inventories_by_currency_id(State(repository): State<DbRepository>, id: Result<Path<u32>, PathRejection>, pagination: Option<Query<Pagination>>) -> impl IntoResponse {
        let service = CustomizedInventroyService::new(repository);
        if let Ok(c_id) = id {
            let inventories_collection = service.get_customized_inventories_by_currency_id(c_id.0 as i32, pagination.unwrap_or_default().0).await;
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
                    let api_error_converter_service = ApiErrorConventerService::new();
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
        else {
            let payload = ResponseCustomizedInventoriesPayload {
                data: None,
                total: None,
                error: Some(ApiError::InvalidParameter)
            };
            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }
}