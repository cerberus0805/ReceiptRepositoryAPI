use std::time::Duration;

use axum::{
    body::Bytes, extract::MatchedPath, http::{HeaderMap, Request}, response::Response, routing::get, Router
};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};

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
            .with_state(repository)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|request: &Request<_>| {
                        let matched_path = request
                            .extensions()
                            .get::<MatchedPath>()
                            .map(MatchedPath::as_str);

                        info_span!(
                            "http_request", 
                            method = ?request.method(), 
                            matched_path,
                            uri = ?request.uri(),
                        )
                    })
                    .on_request(|_request: &Request<_>, _span: &Span| {
                        tracing::info!("on_request");
                    })
                    .on_response(|response: &Response, latency: Duration, _span: &Span| {
                        tracing::info!("on_response - latency: {:#?}, status: {}", latency, response.status());
                    })
                    .on_body_chunk(|_chunk: &Bytes, latency: Duration, _span: &Span| {
                        tracing::debug!("on_body_chunk - latency: {:#?}", latency);
                    })
                    .on_eos(
                        |_trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
                            tracing::info!("on_eos - stream_duration: {:#?}", stream_duration);
                        },
                    )
                    .on_failure(
                        |_error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                            tracing::error!("on_failure - latency: {:#?}", latency);
                        },
                    ),

            );
        Self {
            router
        }
    }
}
