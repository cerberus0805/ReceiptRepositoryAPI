use axum::{
    routing::get, Error, Router
};

pub struct RouterBuilder {}

impl RouterBuilder {
    pub fn build() -> Result<Router, Error> {
        let router = Router::new().route("/", get(|| async { "Hello, World!" }));
        Ok(router)
    }
}
