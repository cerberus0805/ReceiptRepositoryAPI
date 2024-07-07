use std::sync::Arc;
use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use receipt_repository_api::configuration::AppConfig;
use receipt_repository_api::repository::DbRepository;
use receipt_repository_api::router::AppRouter;
use receipt_repository_api::listener::AppListener;
use receipt_repository_api::application::Application;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Arc::new(AppConfig::new());
    let (non_blocking_writer, _guard);
    if config.log_to_file() {
        let file_appender = tracing_appender::rolling::hourly(config.get_log_directory(), config.get_log_prefix());
        (non_blocking_writer, _guard) = tracing_appender::non_blocking(file_appender);
    }
    else {
        (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    }

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "receipt_repository_api=debug,axum::rejection=trace".into()
            }),
        )
        .with(
            tracing_subscriber::fmt::layer().with_writer(
                non_blocking_writer
            )
        )
        .init();
    let repository = DbRepository::new(config.get_db_url());
    let router = AppRouter::new(repository);
    let listener = AppListener::new(config.get_address()).await.expect("TCP listener should be created successfully.");
    let app = Application::new(router, listener);
    app.run().await;
}
