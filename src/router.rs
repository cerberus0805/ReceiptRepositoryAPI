use axum::{
    routing::get, Router
};

use crate::{
    handlers::v1::receipts::receipts_handlers::ReceiptsHandlers, 
    repository::DbRepository
};

pub struct AppRouter {
    pub router: Router,
}

impl AppRouter {
    pub fn new(repository: DbRepository) -> Self {
        let router = Router::new()
            .route("/api/v1/receipts/:id", get(ReceiptsHandlers::get_receipt))
            .route("/api/v1/receipts", get(ReceiptsHandlers::get_receipts))
            .with_state(repository);
        Self {
            router
        }
    }
}
