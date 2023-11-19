use std::fs::{create_dir, File, read_dir, read_to_string, remove_file};
use std::io::Write;
use std::ops::Add;

use aul::{log, log_info};
use aul::level::Level;
use aul::log_error;

const DIRECTORY: &str = "./files";

pub fn init() {
    let dir = read_dir(DIRECTORY);
    if dir.is_err() {
        let _ = create_dir(DIRECTORY);
    }
}


pub fn get_file(path: &str) -> Option<String> {
    read_to_string(get_actual_path(path))
        .map(|str|{log_info!("{}",str); str})
        .map_err(|err| println!("{}", err)).ok()
}

pub fn get_actual_path(path: &str) -> String {
    DIRECTORY.to_string().add(path)
}

pub fn create_file(path: &str, content: &String) -> bool {
    let created = File::create(get_actual_path(path))
        .map_err(|err| log_error!("{}",err))
        .is_ok();
    if created {
        edit_file(path, content)
    } else {
        false
    }
}

pub fn edit_file(path: &str, content: &String) -> bool {
    if let Ok(mut file) = File::create(get_actual_path(path)).map_err(|err| log_error!("{}",err)) {
        log_info!("{}",content);
        log_error!("{:?}",file.write(content.as_bytes()));
        log_error!("{:?}",file.flush());
        true
    } else {
        false
    }
}

pub fn delete_file(path: &str) {
    let _ = remove_file(get_actual_path(path));
}