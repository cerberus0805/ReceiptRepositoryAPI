use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseCurrency {
    pub id: i32,
    pub name: String
}
