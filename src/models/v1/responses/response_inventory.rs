use super::response_product::ResponseProduct;

pub struct ResponseInventory {
    pub id: i32,
    pub product: ResponseProduct,
    pub price: f64,
    pub quantity: i32
}
