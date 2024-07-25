use axum::{extract::{rejection::PathRejection, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{models::v1::{errors::api_error::ApiError, parameters::pagination::Pagination, responses::response_product::{ResponseProductPayload, ResponseProductsPayload}}, repository::DbRepository, services::v1::{converters::api_error_converter_service::ApiErrorConventerService, products::products_service::ProductService}};


pub struct ProductsHandlers {   
}

impl ProductsHandlers {
    pub async fn get_product(State(repository): State<DbRepository>, id: Result<Path<u32>, PathRejection>) -> impl IntoResponse {
        let service = ProductService::new(repository);
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

    pub async fn get_products(State(repository): State<DbRepository>, pagination: Option<Query<Pagination>>) -> impl IntoResponse {
        let service = ProductService::new(repository);
        let product_collection = service.get_products(pagination.unwrap_or_default().0).await;
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
}