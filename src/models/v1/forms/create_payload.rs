use chrono::NaiveDateTime;
use serde::Deserialize;
use uuid::Uuid;

pub trait FormRelationshipModelIdOrName {
    fn get_id_field(&self) -> Option<i32>;
    fn get_name_field(&self) -> Option<String>;
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateCurrencyInReceiptPayload {
    pub id: Option<i32>,
    pub name: Option<String>
}

impl FormRelationshipModelIdOrName for CreateCurrencyInReceiptPayload {
    fn get_id_field(&self) -> Option<i32> {
        self.id
    }

    fn get_name_field(&self) -> Option<String> {
        self.name.clone()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateStoreInReceiptPayload {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub alias: Option<String>,
    pub branch: Option<String>,
    pub address: Option<String>
}

impl FormRelationshipModelIdOrName for CreateStoreInReceiptPayload {
    fn get_id_field(&self) -> Option<i32> {
        self.id
    }

    fn get_name_field(&self) -> Option<String> {
        self.name.clone()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateProductInReceiptPayload {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub alias: Option<String>,
    pub specification_amount: Option<i32>,
    pub specification_unit: Option<String>,
    pub specification_others: Option<String>,
    pub brand: Option<String>
}

impl FormRelationshipModelIdOrName for CreateProductInReceiptPayload {
    fn get_id_field(&self) -> Option<i32> {
        self.id
    }

    fn get_name_field(&self) -> Option<String> {
        self.name.clone()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateInventoryInReceiptPayload {
    pub price: f64,
    pub quantity: i32,
    pub product: CreateProductInReceiptPayload
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateReceiptPayload {
    pub transaction_id: Option<Uuid>,
    pub transaction_date: NaiveDateTime,
    pub is_inventory_taxed: bool,
    pub currency: CreateCurrencyInReceiptPayload,
    pub store: CreateStoreInReceiptPayload,
    pub inventories: Vec<CreateInventoryInReceiptPayload>
}
