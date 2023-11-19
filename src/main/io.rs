use std::fs::{create_dir, read_dir, read_to_string};
use std::ops::Add;


const DIRECTORY: &str = "./files";
pub fn init() {
    let dir = read_dir(DIRECTORY);
    if dir.is_err() {
        let _ = create_dir(DIRECTORY);
    }
}


pub fn get_file(path: &str) -> Option<String> {
    read_to_string(get_actual_path(path))
        .map_err(|err|println!("{}",err)).ok()
}

pub fn get_actual_path(path: &str) -> String{
    DIRECTORY.to_string().add(path)
}