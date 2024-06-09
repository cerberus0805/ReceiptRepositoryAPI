use super::response_product::ResponseProduct;
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseInventory {
    pub id: i32,
    pub product: ResponseProduct,
    pub price: f64,
    pub quantity: i32
}
