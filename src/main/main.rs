use std::io::Write;
use std::net::TcpListener;
use std::thread;

use whdp::{HttpParseError, Request, Response, TryRequest};
use whdp::resp_presets::{bad_request as build_bad_request, ok};

use crate::error::WSSError;

mod error;

fn main() -> Result<(), WSSError> {
    let server = TcpListener::bind("0.0.0.0:8080")
        .map_err(WSSError::from)?;
    for incoming in server.incoming() {
        let stream = incoming.map_err(WSSError::from);
        if stream.is_ok() {
            let mut stream = stream.unwrap();
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


    Ok(())
}

fn bad_request(req: Result<Request, HttpParseError>) -> String {
    build_bad_request(req.err().unwrap().to_string()).to_string()
}

fn handle_connection(_req: Request) -> Response {
    ok("Hello World".into())
}
