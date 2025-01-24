use std::path::Path;
use std::fs::File;

pub fn check_if_has_db() -> bool {
    let path : &Path = Path::new("./app.sqlite3");

    if File::create(path).is_err() {
        return true;
    }

    return false;
}
