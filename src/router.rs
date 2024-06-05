use axum::{
    routing::get, Router
};

pub struct AppRouter {
    pub router: Router
}

impl AppRouter {
    pub fn new() -> Self {
        let router = Router::new().route("/", get(|| async { "Hello, World!" }));
        Self {
            router
        }
    }
}
