use axum::extract::Path;

pub struct ReceiptsHandlers {
}

impl ReceiptsHandlers {
    pub async fn get_receipt(Path(id): Path<u32>) -> String {
        println!("id: {}", id);
        "Hello my receipt".to_string()
    }
}