use diesel::prelude::*;
use serde::{
    Deserialize, 
    Serialize
};

#[derive(Queryable, Selectable, Insertable, AsChangeset, Identifiable, Serialize, Deserialize, Debug, PartialEq)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EntityProduct {
    pub id: i32,
    pub name: String,
    pub alias: Option<String>,
    pub specification_amount: Option<i32>,
    pub specification_unit: Option<String>,
    pub specification_others: Option<String>,
    pub brand: Option<String>
}