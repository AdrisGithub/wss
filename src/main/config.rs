use std::fs::read_to_string;

use crate::error::WSSError;

const CONFIG_FILE: &str = ".config";
const EQUALS : char = '=';
pub fn init() {
    if let Ok(content) = read_to_string(CONFIG_FILE) {
        for line in content.lines() {
            let mut split = line.split(EQUALS);
            let _ = set_variable(split.next(),split.next());
        }
    }
}

fn set_variable(first: Option<&str>, second: Option<&str>) -> Result<(), WSSError> {
    let first = first.ok_or(WSSError::new("".into()))?.trim();
    let second = second.ok_or(WSSError::new("".into()))?.trim();
    std::env::set_var(first, second);
    Ok(())
}