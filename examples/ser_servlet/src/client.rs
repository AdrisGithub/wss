use std::io::{Read, Write};
use std::net::TcpStream;

use wjp::{Deserialize, Serialize};

use ser_servlet::{GET, POST};

fn main() {
    let stream = TcpStream::connect("0.0.0.0:6969");
    let mut stream = stream.unwrap();
    let _ = stream.write_all(POST(String::from("Hello")).json().as_bytes());
    let mut hopefully = String::new();
    let _ = stream.read_to_string(&mut hopefully);
    println!("{:?}",GET::deserialize(hopefully));
}