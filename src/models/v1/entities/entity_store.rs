use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::stores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EntityStore {
    pub id: i32,
    pub name: String,
    pub alias: Option<String>,
    pub branch: Option<String>,
    pub address: Option<String>
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::stores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEntityStore {
    pub name: String,
    pub alias: Option<String>,
    pub branch: Option<String>,
    pub address: Option<String>
}

#[derive(AsChangeset, Identifiable, Debug)]
#[diesel(table_name = crate::schema::stores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateEntityStore {
    pub id: i32,
    pub name: Option<String>,
    pub alias: Option<Option<String>>,
    pub branch: Option<Option<String>>,
    pub address: Option<Option<String>>
}