use std::collections::HashMap;
use std::fmt::Write;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};

use aul::level::Level;
use aul::log;
use aul::sensitive::Sens;
use whdp::{Request, Response};

use crate::error::WBSLError;
use crate::middleware::Middleware;

pub type HTTPFunction = fn(Request) -> Response;
pub type RawFunction = fn(String) -> String;
pub type SerializableFunction<I, O> = fn(I) -> O;

pub(crate) fn parse_stream(stream: &TcpStream) -> Result<String, WBSLError> {
    let mut reader = BufReader::new(stream);
    let received: Vec<u8> = reader.fill_buf().map_err(|_err| WBSLError)?.to_vec();
    reader.consume(received.len());
    String::from_utf8(received).map_err(|_e| WBSLError)
}

pub(crate) struct Logger(Level);

impl From<Level> for Logger {
    fn from(value: Level) -> Self {
        Self(value)
    }
}

unsafe impl Middleware for Logger {
    fn on_request(&self, req: Request) -> Request {
        log!(self.0, "Request: \n{}", Sens(&req));
        req
    }
    fn on_response(&self, resp: Response) -> Response {
        log!(self.0, "Response: \n{}", Sens(&resp));
        resp
    }
}

pub(crate) struct AdditionalHeaders(String, String);

impl From<(String, String)> for AdditionalHeaders {
    fn from(value: (String, String)) -> Self {
        Self(value.0, value.1)
    }
}

unsafe impl Middleware for AdditionalHeaders {
    fn on_request(&self, req: Request) -> Request {
        req
    }
    fn on_response(&self, mut resp: Response) -> Response {
        resp.add_header((String::from("Server"), self.0.clone()));
        resp.add_header((String::from("Content-Type"), self.1.clone()));
        resp.add_header((String::from("Date"), get_current_time()));

        resp
    }
}

pub(crate) fn get_current_time() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let mut out = String::new();
    let _ = write!(out, "{:?}", since_the_epoch);
    out
}
pub fn query(str: &str) -> HashMap<&str,&str>{
    let split = str.split('?');
    let wtf = split.last().map(|func| func.split('&'));
    if let Some(w) = wtf {
        let mut map = HashMap::new();
        for e in w {
            let mut e = e.split('=');
            let key = e.next();
            let val = e.next();
            if let Some(key) = key {
                if let Some(val) = val {
                    map.insert(key, val);
                }
            }
        }
        return map;
    }
    HashMap::new()
}