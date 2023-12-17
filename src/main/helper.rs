use std::io::{BufRead, BufReader};
use std::net::TcpStream;

use whdp::{Request, Response};

use crate::error::WBSLError;

pub type HTTPFunction = fn(Request) -> Response;
pub type RawFunction = fn(String) -> String;
pub type SerializableFunction<I, O> = fn(I) -> O;

pub(crate) fn parse_stream(stream: &TcpStream) -> Result<String, WBSLError> {
    let mut reader = BufReader::new(stream);
    let received: Vec<u8> = reader.fill_buf().map_err(|_err| WBSLError)?.to_vec();
    reader.consume(received.len());
    String::from_utf8(received).map_err(|_e| WBSLError)
}
