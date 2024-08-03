use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PatchCurrencyPayload {
    pub name: String
}

#[derive(Deserialize, Debug)]
pub struct PatchStorePayload {
    pub name: Option<String>,
    #[serde(
        default, 
        with = "::serde_with::rust::double_option",
    )]
    pub alias: Option<Option<String>>,
    #[serde(
        default, 
        with = "::serde_with::rust::double_option",
    )]
    pub branch: Option<Option<String>>,
    #[serde(
        default, 
        with = "::serde_with::rust::double_option",
    )]
    pub address: Option<Option<String>>
}

#[derive(Deserialize, Debug)]
pub struct PatchProductPayload {
    pub name: Option<String>,
    #[serde(
        default, 
        with = "::serde_with::rust::double_option",
    )]
    pub alias: Option<Option<String>>,
    #[serde(
        default, 
        with = "::serde_with::rust::double_option",
    )]
    pub specification_amount: Option<Option<i32>>,
    #[serde(
        default, 
        with = "::serde_with::rust::double_option",
    )]
    pub specification_unit: Option<Option<String>>,
    #[serde(
        default, 
        with = "::serde_with::rust::double_option",
    )]
    pub specification_others: Option<Option<String>>,
    #[serde(
        default, 
        with = "::serde_with::rust::double_option",
    )]
    pub brand: Option<Option<String>>,
}

#[derive(Deserialize, Debug)]
pub struct PatchInventoryPayload {
    pub price: Option<f64>,
    pub quantity: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct PatchReceiptPayload {
    pub transaction_date: Option<NaiveDateTime>,
    pub is_inventory_taxed: Option<bool>,
}