use http::{HeaderValue, Method};
use http::header::CONTENT_TYPE;
use tower_http::cors::CorsLayer;

use crate::router::AppRouter;
use crate::listener::AppListener;

pub struct Application {
    app_router: AppRouter,
    app_listener: AppListener
}

impl Application {
    pub fn new(app_router: AppRouter, app_listener: AppListener) -> Self {
        Self {
            app_router,
            app_listener
        }
    }

    pub async fn run(self, allow_origins: &Vec<String>) {
        tracing::info!("app start");

        let allow_origin_header_values = allow_origins.into_iter().map(|o| { o.parse::<HeaderValue>().unwrap() }).collect::<Vec<HeaderValue>>();
        let cors = 
            CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE, Method::OPTIONS])
            .expose_headers([CONTENT_TYPE])
            .allow_headers([CONTENT_TYPE])
            .allow_origin(allow_origin_header_values);
        axum::serve(self.app_listener.listener, self.app_router.router.layer(cors)).await.unwrap();
    }
}