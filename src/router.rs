use axum::{
    routing::get, Error, Router
};

pub fn get_router() -> Result<Router, Error> {
    let router = Router::new().route("/", get(|| async { "Hello, World!" }));
    Ok(router)
}
