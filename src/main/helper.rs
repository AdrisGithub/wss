use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use std::str::Split;

use aul::level::Level;
use aul::log;
use aul::sensitive::Sens;
use wbdl::Date;
use whdp::{Request, Response};
use whdp::resp_presets::ok;
use wjp::{map, Serialize, Values};

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

pub(crate) fn health(_: Request) -> Response {
    ok(Health::default().json())
}

pub struct Health {
    active: bool,
    time: String,
    ram: Option<Ram>,
}

impl Serialize for Health {
    fn serialize(&self) -> Values {
        Values::Struct(map!(
            ("active", self.active.serialize()),
            ("time", self.time.serialize()),
            ("ram", self.ram.serialize())
        ))
    }
}

pub struct Ram {
    total: u64,
    free: u64,
}

impl Serialize for Ram {
    fn serialize(&self) -> Values {
        let total = (self.total as f64 / 1_000_000_f64).to_string() + " GB";
        let free = (self.free as f64 / 1_000_000_f64).to_string() + " GB";
        Values::Struct(map!(
            ("total", total.serialize()),
            ("free", free.serialize())
        ))
    }
}

impl Default for Health {
    fn default() -> Self {
        Self {
            active: true,
            time: get_current_time(),
            ram: get_mem(),
        }
    }
}

fn get_mem() -> Option<Ram> {
    let mut s = String::new();
    File::open("/proc/meminfo")
        .ok()?
        .read_to_string(&mut s)
        .ok()?;
    let mut meminfo_hashmap = HashMap::new();
    for line in s.lines() {
        let mut split_line = line.split_whitespace();
        let label = split_line.next();
        let value = split_line.next();
        if value.is_some() && label.is_some() {
            let label = label.unwrap().split(':').next()?;
            let value = value.unwrap().parse::<u64>().ok()?;
            meminfo_hashmap.insert(label, value);
        }
    }
    let total = meminfo_hashmap.remove("MemTotal")?;
    let free = meminfo_hashmap.remove("MemFree")?;
    Some(Ram { free, total })
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
        resp.try_insert((String::from("Server"), self.0.clone()));
        resp.try_insert((String::from("Content-Type"), self.1.clone()));
        resp.try_insert((String::from("Date"), get_current_time()));

        resp
    }
}

pub(crate) fn get_current_time() -> String {
    Date::now().map(|s| s.to_string()).expect("Time went backwards")
}

pub fn query(str: &str) -> HashMap<&str, &str> {
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

pub(crate) fn remainder(mut split: Split<char>) -> Option<String> {
    let mut res = String::new();
    let mut next = split.next();
    while next.is_some() {
        res += next.unwrap();
        next = split.next()
    }
    Some(res)
}
