use axum::{
    extract::{
        Path, 
        State,
        Query
    }, 
    http::StatusCode, 
    response::IntoResponse, Json
};

use crate::{
    models::v1::responses::response_receipt::{ReponseReceiptPayload, ReponseReceiptsPayload}, 
    models::v1::parameters::pagination::Pagination,
    repository::DbRepository,
    services::v1::receipts::receipts_service::ReceiptService
};

pub struct ReceiptsHandlers {
}

impl ReceiptsHandlers {
    pub async fn get_receipt(State(repository): State<DbRepository>, Path(id): Path<u32>) -> impl IntoResponse {
        let service = ReceiptService::new(repository);

        let response_receipt = service.get_receipt(id as i32).await;
        match response_receipt {
            Ok(response) => {
                let payload = ReponseReceiptPayload {
                    data: Some(response),
                    error: None
                };
        
                (StatusCode::OK, Json(payload))
            },
            Err(e) => {
                let payload = ReponseReceiptPayload {
                    data: None,
                    error: Some(e.to_string())
                };
                (StatusCode::NOT_FOUND, Json(payload))
            }
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
                let payload = ReponseReceiptsPayload {
                    data: None,
                    total: None,
                    error: Some(e.to_string())
                };
                (StatusCode::NOT_ACCEPTABLE, Json(payload))
            }
        }
    }
}