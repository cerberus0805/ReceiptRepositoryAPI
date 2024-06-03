use receipt_repository_api::application::Application;
use receipt_repository_api::configuration::*;

#[tokio::main]
async fn main() {
    let settings = get_config().expect("Get configuration error");
    let app = Application::build(settings).await.unwrap();
    app.run().await;
}
