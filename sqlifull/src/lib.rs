pub mod db;
pub mod init;
pub mod env;

#[derive(Debug)]
pub struct Post {
    pub id: i32,
    pub content: String,
}

