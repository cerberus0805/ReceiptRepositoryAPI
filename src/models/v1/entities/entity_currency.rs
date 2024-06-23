use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::currencies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EntityCurrency {
    pub id: i32,
    pub name: String
}