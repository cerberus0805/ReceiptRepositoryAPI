use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::schema::currencies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EntityCurrency {
    pub id: i32,
    pub name: String
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::currencies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEntityCurrency {
    pub name: String
}

#[derive(AsChangeset, Identifiable, Debug)]
#[diesel(table_name = crate::schema::currencies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateEntityCurrency<'a> {
    pub id: i32,
    pub name: &'a String
}