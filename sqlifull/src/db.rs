use rusqlite::Connection;
use super::Post; 

pub fn insert_post(connection : &Connection, new_post : &Post) {
    let command : String = format!("
            INSERT INTO post (content) VALUES (\"{}\");
        ", new_post.content);

    let  _ = connection.execute_batch(command.as_str());
}

pub fn create_post(connection : &Connection) -> Option<usize> {
    return match connection.execute(
            "CREATE TABLE post (
                id    INTEGER PRIMARY KEY,
                content  TEXT NOT NULL
            )",
            (), // empty list of parameters.
        ) {
        Ok(value) => Some(value),
        Err(err) => {
            eprintln!("{}", err);
            return None;
        }
    };

}
