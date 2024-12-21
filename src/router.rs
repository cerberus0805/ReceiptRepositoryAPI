use std::time::Duration;

use axum::{
    body::Bytes, extract::MatchedPath, http::{HeaderMap, Request}, middleware, response::Response, routing::{delete, get, patch, post}, Router
};
use tower_cookies::CookieManagerLayer;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};

use crate::{
    handlers::v1::{currencies::currencies_handlers::CurrenciesHandlers, inventories::{customized_inventories_handlers::CustomizedInventoriesHandlers, inventories_handlers::InventoriesHandlers}, loginout::loginout_handlers::LoginoutHandlers, products::products_handlers::ProductsHandlers, receipts::receipts_handlers::ReceiptsHandlers, stores::stores_handlers::StoresHandlers}, mw_auth, response_mapper::response_mapper, share_state::HandlerState
};

pub struct AppRouter {
    pub router: Router
}

impl<'a> AppRouter {
    pub fn new(handler_state: HandlerState) -> Self {
        let v1_receipts_router = Router::new()
            .route("/receipts/:id", get(ReceiptsHandlers::get_receipt))
            .route("/receipts/transaction/:transaction_id", get(ReceiptsHandlers::get_receipt_by_transaction_id))
            .route("/receipts", get(ReceiptsHandlers::get_receipts))
            .route("/receipts", post(ReceiptsHandlers::post_receipt))
            .route("/receipts/:id", patch(ReceiptsHandlers::patch_receipt))
            .route("/receipts/:id", delete(ReceiptsHandlers::delete_receipt))
            .route("/receipts/:id/customized_inventories", get(CustomizedInventoriesHandlers::get_customized_inventories_by_receipt_id));
        
        let v1_stores_router = Router::new()
            .route("/stores/:id", get(StoresHandlers::get_store))
            .route("/stores", get(StoresHandlers::get_stores))
            .route("/stores/:id", patch(StoresHandlers::patch_store))
            .route("/stores/autocomplete", get(StoresHandlers::autocomplete_stores))
            .route("/stores/:id/customized_inventories", get(CustomizedInventoriesHandlers::get_customized_inventories_by_store_id));
        
        let v1_currencies_router = Router::new()
            .route("/currencies/:id", get(CurrenciesHandlers::get_currency))
            .route("/currencies", get(CurrenciesHandlers::get_currencies))
            .route("/currencies/:id", patch(CurrenciesHandlers::patch_currency))
            .route("/currencies/autocomplete", get(CurrenciesHandlers::autocomplete_currencies))
            .route("/currencies/:id/customized_inventories", get(CustomizedInventoriesHandlers::get_customized_inventories_by_currency_id));
        
        let v1_product_router = Router::new()
            .route("/products/:id", get(ProductsHandlers::get_product))
            .route("/products", get(ProductsHandlers::get_products))
            .route("/products/:id", patch(ProductsHandlers::patch_product))
            .route("/products/autocomplete", get(ProductsHandlers::autocomplete_products))
            .route("/products/:id/customized_inventories", get(CustomizedInventoriesHandlers::get_customized_inventories_by_product_id));
        
        let v1_inventories_router = Router::new()
            .route("/inventories/:id", get(InventoriesHandlers::get_inventory))
            .route("/inventories", get(InventoriesHandlers::get_inventories))
            .route("/inventories/:id", patch(InventoriesHandlers::patch_inventory));
        
        let v1_customized_inventories_router = Router::new()
            .route("/customized_inventories/:id", get(CustomizedInventoriesHandlers::get_customized_inventory))
            .route("/customized_inventories", get(CustomizedInventoriesHandlers::get_customized_inventories));

        let v1_login_router = Router::new()
            .route("/login", post(LoginoutHandlers::api_login));
        
        let api_v1_router = Router::new()
            .nest("/api/v1", v1_receipts_router)
            .nest("/api/v1", v1_stores_router)
            .nest("/api/v1", v1_currencies_router)
            .nest("/api/v1", v1_product_router)
            .nest("/api/v1", v1_inventories_router)
            .nest("/api/v1", v1_customized_inventories_router)
            .route_layer(middleware::from_fn(mw_auth::mw_require_auth));

        let router = Router::new()
            .nest("/api/v1", v1_login_router)
            .merge(api_v1_router)
            .layer(middleware::map_response(response_mapper))
            .layer(CookieManagerLayer::new())
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
