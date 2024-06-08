use axum::{
    routing::get, Router
};

use crate::handlers::v1::receipts::receipts_handlers::ReceiptsHandlers;

pub struct AppRouter {
    pub router: Router
}

impl AppRouter {
    pub fn new() -> Self {
        let router = Router::new().route("/api/v1/receipts/:id", get(ReceiptsHandlers::get_one_receipt));
        Self {
            router
        }
    }
}
