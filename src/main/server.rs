use std::fmt::{Debug, Write as write};
use std::io::Write;
use std::net::{TcpListener, ToSocketAddrs};
use std::thread;

use aul::level::Level;
use aul::log;
use aul::log_info;
use whdp::{HttpMethod, HttpParseError, Request, Response, TryRequest};
use whdp::resp_presets::{bad_request as build_bad_request, created, no_content, not_found, not_implemented, ok};

use crate::error::WSSError;
use crate::io::{create_file, delete_file, edit_file, get_file};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn init<T: ToSocketAddrs+Debug>(addr: T) -> Result<Self, WSSError> {
        let listener = TcpListener::bind(&addr).map_err(WSSError::from)?;
        log_info!("Server listening on {:?}",listener.local_addr().map_err(WSSError::from)?);
        Ok(Self {
            listener
        })
    }
    pub fn start(&self) {
        for mut stream in self.listener.incoming().flatten() {
            let req = stream.try_to_request();
            if let Ok(req) = req {
                thread::spawn(move || {
                    let resp = handle_connection(req);
                    let _ = stream.write_all(resp.to_string().as_bytes());
                });
            } else {
                let _ = stream.write_all(bad_request(req).as_bytes());
            }
        }
    }
}

fn handle_connection(req: Request) -> Response {
    log_info!("{} {} {}",req.get_method(),req.get_uri(),req.get_version());
    match req.get_method() {
        HttpMethod::Post => handle_post(req),
        HttpMethod::Get => handle_get(req),
        HttpMethod::Put => handle_put(req),
        HttpMethod::Delete => handle_delete(req),
        _ => handle_not_implemented(req)
    }
}

fn bad_request(req: Result<Request, HttpParseError>) -> String {
    build_bad_request(req.err().unwrap().to_string()).to_string()
}

fn handle_not_implemented(request: Request) -> Response {
    let mut string = String::new();
    let _ = write!(string, "Method: {} is not implemented", request.get_method());
    not_implemented(string)
}

fn handle_get(req: Request) -> Response {
    let content = get_file(req.get_uri());
    if let Some(content) = content {
        ok(content)
    } else {
        not_found("".into())
    }
}

fn handle_post(req: Request) -> Response {
    if create_file(req.get_uri(), req.get_body()) {
        created("".into())
    } else {
        build_bad_request("File already exists".into())
    }
}

fn handle_put(req: Request) -> Response {
    if edit_file(req.get_uri(), req.get_body()) {
        no_content("".into())
    } else {
        build_bad_request("File doesn't exist".into())
    }
}

fn handle_delete(request: Request) -> Response {
    delete_file(request.get_uri());
    no_content("".into())
}