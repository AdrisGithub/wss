use std::io;
use std::io::Write;
use std::net::{SocketAddr, TcpListener};

use whdp::resp_presets::ok;
use whdp::TryRequest;

use crate::middleware::Middleware;

pub struct Server {
    listener: TcpListener,
    middlewares: Vec<Box<dyn Middleware>>,
}

impl Server {
    pub fn start(&self) {
        for stream in self.listener.incoming() {
            let mut stream = stream
                .unwrap();
            let mut req = stream.try_to_request()
                .unwrap();
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