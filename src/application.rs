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

    pub async fn run(self) {
        tracing::info!("app start");
        axum::serve(self.app_listener.listener, self.app_router.router).await.unwrap();
    }
}