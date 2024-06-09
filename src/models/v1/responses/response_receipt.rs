use chrono::NaiveDateTime;
use super::{response_currency::ResponseCurrency, response_inventory::ResponseInventory, response_store::ResponseStore};

pub struct ResponseReceipt {
    pub id: i32,
    pub transaction_date: NaiveDateTime,
    pub is_inventory_taxed: bool,
    pub currency: ResponseCurrency,
    pub store: ResponseStore,
    pub inventories: Vec<ResponseInventory>
}

pub struct ReponseReceiptPayload {
    pub data: Option<ResponseReceipt>,
    pub error: Option<String>
}
