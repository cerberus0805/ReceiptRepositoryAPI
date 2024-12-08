use std::time::Duration;

use axum::{
    body::Bytes, extract::MatchedPath, http::{HeaderMap, Request}, response::Response, routing::{delete, get, patch, post}, Router
};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};

use crate::{
    handlers::v1::{currencies::currencies_handlers::CurrenciesHandlers, inventories::{customized_inventories_handlers::CustomizedInventoriesHandlers, inventories_handlers::InventoriesHandlers}, products::products_handlers::ProductsHandlers, receipts::receipts_handlers::ReceiptsHandlers, stores::stores_handlers::StoresHandlers}, 
    share_state::HandlerState
};

pub struct AppRouter {
    pub router: Router
}

impl<'a> AppRouter {
    pub fn new(handler_state: HandlerState) -> Self {
        let router = Router::new()
            .route("/api/v1/receipts/:id", get(ReceiptsHandlers::get_receipt))
            .route("/api/v1/receipts/transaction/:transaction_id", get(ReceiptsHandlers::get_receipt_by_transaction_id))
            .route("/api/v1/receipts", get(ReceiptsHandlers::get_receipts))
            .route("/api/v1/receipts", post(ReceiptsHandlers::post_receipt))
            .route("/api/v1/receipts/:id", patch(ReceiptsHandlers::patch_receipt))
            .route("/api/v1/receipts/:id", delete(ReceiptsHandlers::delete_receipt))
            .route("/api/v1/stores/:id", get(StoresHandlers::get_store))
            .route("/api/v1/stores", get(StoresHandlers::get_stores))
            .route("/api/v1/stores/:id", patch(StoresHandlers::patch_store))
            .route("/api/v1/stores/autocomplete", get(StoresHandlers::autocomplete_stores))
            .route("/api/v1/currencies/:id", get(CurrenciesHandlers::get_currency))
            .route("/api/v1/currencies", get(CurrenciesHandlers::get_currencies))
            .route("/api/v1/currencies/:id", patch(CurrenciesHandlers::patch_currency))
            .route("/api/v1/currencies/autocomplete", get(CurrenciesHandlers::autocomplete_currencies))
            .route("/api/v1/products/:id", get(ProductsHandlers::get_product))
            .route("/api/v1/products", get(ProductsHandlers::get_products))
            .route("/api/v1/products/:id", patch(ProductsHandlers::patch_product))
            .route("/api/v1/products/autocomplete", get(ProductsHandlers::autocomplete_products))
            .route("/api/v1/inventories/:id", get(InventoriesHandlers::get_inventory))
            .route("/api/v1/inventories", get(InventoriesHandlers::get_inventories))
            .route("/api/v1/inventories/:id", patch(InventoriesHandlers::patch_inventory))
            .route("/api/v1/customized_inventories/:id", get(CustomizedInventoriesHandlers::get_customized_inventory))
            .route("/api/v1/customized_inventories", get(CustomizedInventoriesHandlers::get_customized_inventories))
            .route("/api/v1/products/:id/customized_inventories", get(CustomizedInventoriesHandlers::get_customized_inventories_by_product_id))
            .route("/api/v1/receipts/:id/customized_inventories", get(CustomizedInventoriesHandlers::get_customized_inventories_by_receipt_id))
            .route("/api/v1/stores/:id/customized_inventories", get(CustomizedInventoriesHandlers::get_customized_inventories_by_store_id))
            .route("/api/v1/currencies/:id/customized_inventories", get(CustomizedInventoriesHandlers::get_customized_inventories_by_currency_id))
            .with_state(handler_state)
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
