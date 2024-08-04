use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EntityProduct {
    pub id: i32,
    pub name: String,
    pub alias: Option<String>,
    pub brand: Option<String>,
    pub specification_amount: Option<i32>,
    pub specification_unit: Option<String>,
    pub specification_others: Option<String>
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEntityProduct {
    pub name: String,
    pub alias: Option<String>,
    pub brand: Option<String>,
    pub specification_amount: Option<i32>,
    pub specification_unit: Option<String>,
    pub specification_others: Option<String>
}

#[derive(AsChangeset, Identifiable, Debug)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateEntityProduct<'a> {
    pub id: i32,
    pub name: Option<&'a String>,
    pub alias: Option<Option<&'a String>>,
    pub brand: Option<Option<&'a String>>,
    pub specification_amount: Option<Option<&'a i32>>,
    pub specification_unit: Option<Option<&'a String>>,
    pub specification_others: Option<Option<&'a String>>
}