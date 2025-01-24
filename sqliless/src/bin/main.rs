use diesel::{Connection, SqliteConnection};
use sqliless::{db::insert_post, env::get_args, Post};

fn main() {

    let connection : &mut SqliteConnection = &mut SqliteConnection::establish("./app.sqlite3").unwrap();

    let args : Vec<String> = get_args();

    match args[1].as_str() {
        "create" => {
            match args[2].as_str() {
                "post" => {
                    let new_post : Post  = Post {id:0, content: args[3].clone()};

                    insert_post(connection, &new_post);
                
                },
                _ => (),
            }
        },
        _ => (),
    }
}
