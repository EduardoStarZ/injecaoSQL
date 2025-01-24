use diesel::prelude::*;

pub mod db;
pub mod env;
pub mod schema;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::post)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: i32,
    pub content: String
}
