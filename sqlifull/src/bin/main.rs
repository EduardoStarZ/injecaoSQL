use rusqlite::Connection;
use sqlifull::{db::{create_post, insert_post}, env::get_args, init::check_if_has_db};
use sqlifull::Post;

fn main() {
    let connection = Connection::open("./app.sqlite3").unwrap();

    if !check_if_has_db() {
        create_post(&connection);
    }


    let args : Vec<String> = get_args();

    match args[1].as_str() {
        "create" => {
            match args[2].as_str() {
                "post" => {
                    let new_post : Post  = Post {id:0, content: args[3].clone()};

                    insert_post(&connection, &new_post);
                
                },
                _ => (),
            }
        },
        _ => (),
    }
}
