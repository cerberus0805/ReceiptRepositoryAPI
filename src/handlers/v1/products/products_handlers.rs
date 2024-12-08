use axum::{extract::{rejection::{JsonRejection, PathRejection}, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{models::v1::{commands::writer_command::WriterCommand, errors::api_error::ApiError, forms::patch_payload::PatchProductPayload, parameters::{pagination::Pagination, query_filters::KeywordFilters}, responses::response_product::{ResponseProductPayload, ResponseProductsPayload}}, services::v1::{converters::api_error_converter_service::ApiErrorConventerService, products::products_service::ProductService}, share_state::HandlerState};


pub struct ProductsHandlers {   
}

impl ProductsHandlers {
    pub async fn get_product(State(handler_state): State<HandlerState>, id: Result<Path<u32>, PathRejection>) -> impl IntoResponse {
        let service = ProductService::new(&handler_state.repository);
        if let Ok(s_id) = id {
            let response_product = service.get_product(s_id.0 as i32).await;
            match response_product {
                Ok(response) => {
                    let payload: ResponseProductPayload = ResponseProductPayload {
                        data: Some(response),
                        error: None
                    };
                    (StatusCode::OK, Json(payload))
                },
                Err(e) => {
                    let api_error_converter_service = ApiErrorConventerService::new();
                    let http_status_code = api_error_converter_service.get_http_status_from_api_error(&e);

                    let payload = ResponseProductPayload {
                        data: None,
                        error: Some(e)
                    };
                    (http_status_code, Json(payload))
                }
            }
        }
        else {
            let payload = ResponseProductPayload {
                data: None,
                error: Some(ApiError::InvalidParameter)
            };

            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }

    pub async fn get_products(State(handler_state): State<HandlerState>, pagination: Option<Query<Pagination>>) -> impl IntoResponse {
        let service = ProductService::new(&handler_state.repository);
        let product_collection = service.get_products(&pagination.unwrap_or_default().0).await;
        match product_collection {
            Ok(responses) => {
                let payload = ResponseProductsPayload {
                    data: Some(responses.partial_collection),
                    total: Some(responses.total_count),
                    error: None
                };
                (StatusCode::OK, Json(payload))
            },
            Err(e) => {
                let api_error_converter_service = ApiErrorConventerService::new();
                let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);
                let payload = ResponseProductsPayload {
                    data: None,
                    total: None,
                    error: Some(e)
                };
                (http_return_code, Json(payload))
            }
        }
    }

    pub async fn patch_product(State(handler_state): State<HandlerState>, id: Result<Path<u32>, PathRejection>,  payload: Result<Json<PatchProductPayload>, JsonRejection>) -> impl IntoResponse {
        if id.is_ok() && payload.is_ok() {
            let p_id = id.expect("id should be ok after we have checked").0;
            let p_payload = payload.expect("payload should be ok after we have checked").0;
            let patch_command = WriterCommand::PatchProduct(p_id as i32, p_payload);
            let _ = handler_state.sender.send(patch_command).await;
            let response = ResponseProductPayload {
                data: None,
                error: None
            };
            (StatusCode::ACCEPTED, Json(response))
        }
        else {
            let payload = ResponseProductPayload {
                data: None,
                error: Some(ApiError::InvalidParameter)
            };

            (StatusCode::BAD_REQUEST, Json(payload))
        }
    }

    pub async fn autocomplete_products(State(handler_state): State<HandlerState>, kw: Option<Query<KeywordFilters>>) -> impl IntoResponse {
        let service = ProductService::new(&handler_state.repository);
        let products_collection;
        if let Some(keyword) = kw {
            products_collection = service.autocomplete_products(&keyword.0.keyword).await;
        }
        else {
            products_collection = service.autocomplete_products(&None::<String>).await;
        }

        match products_collection {
            Ok(responses) => {
                let payload = ResponseProductsPayload {
                    data: Some(responses.partial_collection),
                    total: Some(responses.total_count),
                    error: None
                };
                (StatusCode::OK, Json(payload))
            },
            Err(e) => {
                let api_error_converter_service = ApiErrorConventerService::new();
                let http_return_code = api_error_converter_service.get_http_status_from_api_error(&e);
                let payload = ResponseProductsPayload {
                    data: None,
                    total: None,
                    error: Some(e)
                };
                (http_return_code, Json(payload))
            }
        }
    }
}