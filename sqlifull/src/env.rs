use std::env::args;

pub fn get_args() -> Vec<String> {
    return args().map(|arg| arg.to_string()).collect::<Vec<String>>();
}
