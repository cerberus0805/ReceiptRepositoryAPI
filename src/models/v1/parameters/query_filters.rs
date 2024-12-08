use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct QueryFilters {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub currency: Option<String>,
    pub store_name: Option<String>,
    pub store_alias: Option<String>,
    pub product_name: Option<String>,
    pub product_alias: Option<String>,
    pub product_brand: Option<String>
}

impl Default for QueryFilters {
    fn default() -> Self {
        Self {
            start_date: None,
            end_date: None,
            currency: None,
            store_name: None,
            store_alias: None,
            product_name: None,
            product_alias: None,
            product_brand: None
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct KeywordFilters {
    pub keyword: Option<String>
}

impl Default for KeywordFilters {
    fn default() -> Self {
        Self {
            keyword: None
        }
    }
}