use std::io::Error;

use tokio::net::TcpListener;
pub struct AppListener {
    pub listener: TcpListener
}

impl AppListener {
    pub async fn new(address: String) -> Result<Self, Error> {
        let listener = tokio::net::TcpListener::bind(&address).await;
        match listener {
            Ok(listener) => {
                tracing::info!("listening on {}", &address);
                Ok(Self {
                    listener
                })
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}
