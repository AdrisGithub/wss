use std::fs::{create_dir, File, read_dir, read_to_string, remove_file};
use std::io::Write;
use std::ops::Add;

use aul::{log, log_info};
use aul::level::Level;
use aul::log_error;

use crate::error::WSSError;

const DIRECTORY: &str = "./files";

pub fn init() -> Result<(), WSSError> {
    let dir = read_dir(DIRECTORY);
    if dir.is_err() {
        create_dir(DIRECTORY)?;
    }
    Ok(())
}


pub fn get_file(path: &str) -> Option<String> {
    log_info!("Accessing: {}",path);
    read_to_string(get_actual_path(path)).ok()
}

pub fn get_actual_path(path: &str) -> String {
    DIRECTORY.to_string().add(path)
}

pub fn create_file(path: &str, content: &String) -> bool {
    if get_file(path).is_some() {
        return false;
    }
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
    if get_file(path).is_none() {
        return false;
    }
    if let Ok(mut file) = File::create(get_actual_path(path)).map_err(|err| log_error!("{}",err)) {
        file.write_all(content.as_bytes()).is_ok()
    } else {
        false
    }
}

pub fn delete_file(path: &str) {
    let _ = remove_file(get_actual_path(path));
}