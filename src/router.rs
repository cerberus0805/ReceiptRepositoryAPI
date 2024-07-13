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
                            uri = %request.uri(),
                            status = tracing::field::Empty,
                            latency_on_response = tracing::field::Empty,
                            latency_on_body_chunk = tracing::field::Empty,
                            stream_duration_on_eos = tracing::field::Empty,
                            latency_on_failure = tracing::field::Empty,
                        )
                    })
                    .on_request(|_request: &Request<_>, _span: &Span| {
                        tracing::info!("on_request");
                    })
                    .on_response(|response: &Response, latency: Duration, span: &Span| {
                        let status_str = format!("{}", response.status());
                        let latency_str = format!("{:?}", latency);
                        span.record("status", status_str);
                        span.record("latency_on_response", &latency_str);
                        tracing::info!("on_response");
                    })
                    .on_body_chunk(|chunk: &Bytes, latency: Duration, span: &Span| {
                        let latency_str = format!("{:?}", latency);
                        span.record("latency_on_body_chunk", &latency_str);
                        tracing::debug!("on_body_chunk, {} bytes sent", chunk.len());
                    })
                    .on_eos(
                        |_trailers: Option<&HeaderMap>, stream_duration: Duration, span: &Span| {
                            let stream_duration_str = format!("{:?}", stream_duration);
                            span.record("stream_duration_on_eos", &stream_duration_str);
                            tracing::info!("on_eos - stream_duration");
                        },
                    )
                    .on_failure(
                        |_error: ServerErrorsFailureClass, latency: Duration, span: &Span| {
                            let latency_str = format!("{:?}", latency);
                            span.record("latency_on_failure", &latency_str);
                            tracing::error!("on_failure");
                        },
                    ),
            );
        Self {
            router
        }
    }
}
