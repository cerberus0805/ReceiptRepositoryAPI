use axum::response::Response;
use tracing::info;

pub async fn response_mapper(res: Response) -> Response {
    info!("response_mapper");
    info!("");
    res
}
