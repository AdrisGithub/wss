use std::io::Write;
use std::net::TcpListener;
use std::thread;

use whdp::{HttpParseError, Request, Response, TryRequest};
use whdp::resp_presets::{bad_request as build_bad_request, not_found, ok};

use crate::error::WSSError;

mod error;
mod io;

fn main() -> Result<(), WSSError> {
    let server = TcpListener::bind("0.0.0.0:8080")
        .map_err(WSSError::from)?;
    io::init();
    for mut stream in server.incoming().flatten() {
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


    Ok(())
}

fn bad_request(req: Result<Request, HttpParseError>) -> String {
    build_bad_request(req.err().unwrap().to_string()).to_string()
}

fn handle_connection(req: Request) -> Response {
    let content = io::get_file(req.get_uri());
    if let Some(content) = content{
        ok(content)
    }else{
        not_found("".into())
    }
}
