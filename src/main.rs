use std::sync::Arc;

use receipt_repository_api::configuration::AppConfig;
use receipt_repository_api::repository::DbRepository;
use receipt_repository_api::router::AppRouter;
use receipt_repository_api::listener::AppListener;
use receipt_repository_api::application::Application;

#[tokio::main]
async fn main() {
    let config = Arc::new(AppConfig::new());
    let repository = DbRepository::new(config.get_db_url());
    let router = AppRouter::new(repository);
    let listener = AppListener::new(config.get_address()).await.expect("TCP listener should be created successfully.");
    let app = Application::new(router, listener);
    app.run().await;
}
