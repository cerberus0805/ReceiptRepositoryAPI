use diesel::prelude::*;
use serde::{
    Deserialize, 
    Serialize
};

#[derive(Queryable, Selectable, Insertable, AsChangeset, Identifiable, Serialize, Deserialize, Debug, PartialEq)]
#[diesel(table_name = crate::schema::currencies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EntityCurrency {
    pub id: i32,
    pub name: String
}