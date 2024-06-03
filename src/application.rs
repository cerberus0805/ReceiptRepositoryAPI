use axum::{ Error, Router };

use crate::configuration::Config;
use crate::router::get_router;

pub struct Application {
    router: Router,
    listener: tokio::net::TcpListener
}

impl Application {
    pub async fn build(settings: Config) -> Result<Self, Error> {
        let address = format!("{}:{}", settings.host, settings.port);

        let my_router = get_router().expect("Get router failed.");

        // run our app with hyper, listening globally on the specified address
        let my_listener = tokio::net::TcpListener::bind(address).await.unwrap();
    
        Ok(Self {
            router: my_router,
            listener: my_listener
        })
    }

    pub async fn run(self) {
        axum::serve(self.listener, self.router).await.unwrap();
    }
}