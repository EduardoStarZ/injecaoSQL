use diesel::{RunQueryDsl, SqliteConnection};
use crate::schema::post;
use crate::Post;

pub fn insert_post(connection : &mut SqliteConnection, new_post : &Post) {
    diesel::insert_into(post::table)
        .values(new_post)
        .execute(connection)
        .unwrap();
}
