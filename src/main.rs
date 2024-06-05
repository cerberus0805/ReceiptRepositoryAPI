use receipt_repository_api::configuration::AppConfig;
use receipt_repository_api::router::AppRouter;
use receipt_repository_api::listener::AppListener;
use receipt_repository_api::application::Application;

#[tokio::main]
async fn main() {
    let settings = AppConfig::new();
    let router = AppRouter::new();
    let listener = AppListener::new(settings.get_address()).await.expect("TCP listener should be created successfully.");
    let app = Application::new(router, listener);
    app.run().await;
}
