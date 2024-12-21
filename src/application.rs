use std::net::SocketAddr;
use std::str::FromStr;

use axum_server::tls_rustls::RustlsConfig;
use http::{HeaderValue, Method};
use http::header::{ACCEPT, CONTENT_TYPE, COOKIE};
use tower_http::cors::CorsLayer;

use crate::router::AppRouter;

pub struct Application {
    app_router: AppRouter,
    address: String,
    tls_config: RustlsConfig
}

impl Application {
    pub fn new(app_router: AppRouter, address: String, tls_config: RustlsConfig) -> Self {
        Self {
            app_router,
            address,
            tls_config
        }
    }

    pub async fn run(self, allow_origins: &Vec<String>) {
        tracing::info!("app start");

        let allow_origin_header_values = allow_origins.into_iter().map(|o| { o.parse::<HeaderValue>().unwrap() }).collect::<Vec<HeaderValue>>();
        let cors = 
            CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE, Method::OPTIONS])
            .expose_headers([CONTENT_TYPE])
            .allow_headers([CONTENT_TYPE, ACCEPT, COOKIE])
            .allow_credentials(true)
            .allow_origin(allow_origin_header_values);

        let addr = SocketAddr::from_str(self.address.as_str()).unwrap();
        axum_server::bind_rustls(addr, self.tls_config)
            .serve(self.app_router.router.layer(cors).into_make_service())
            .await
            .unwrap();
    }
}