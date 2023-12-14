use std::io;
use std::io::Write;
use std::net::{Incoming, SocketAddr, TcpListener};

use whdp::resp_presets::{internal_server_error, ok};
use whdp::{Request, TryRequest};

use crate::middleware::Middleware;

pub struct Server {
    listener: TcpListener,
    middlewares: Vec<Box<dyn Middleware>>,
}

impl Server {
    pub fn start(&self) {
        for stream in self.incoming() {
            let mut stream = stream.unwrap();
            let mut req = stream.try_to_request().unwrap();
            for middle in self.middlewares.iter() {
                req = middle.on_request(req);
            }

            // do the routing shit
            let mut resp = ok(String::new());

            for middle in self.middlewares.iter() {
                resp = middle.on_response(resp)
            }

            let _ = stream.write_all(resp.to_string().as_bytes());
        }
    }

    pub fn incoming(&self) -> Incoming<'_> {
        self.listener.incoming()
    }
}

impl TryFrom<SocketAddr> for Server {
    type Error = io::Error;
    fn try_from(value: SocketAddr) -> Result<Self, Self::Error> {
        Ok(Self {
            listener: TcpListener::bind(value)?,
            middlewares: Vec::new(),
        })
    }
}

impl Iterator for Server {
    type Item = Request;
    fn next(&mut self) -> Option<Self::Item> {
        let opt = self.incoming().next();
        if let Some(Ok(mut stream)) = opt {
            if let Ok(req) = stream.try_to_request() {
                return Some(req);
            } else {
                let _ = stream.write_all(internal_error().as_bytes());
            }
        }
        self.next()
    }
}

fn internal_error() -> String {
    internal_server_error("Internal Server Error".into()).to_string()
}
