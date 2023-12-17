use std::io::Write;
use std::net::{TcpListener, TcpStream};

use aul::error;
use aul::level::Level;
use aul::log;

use crate::helper::{parse_stream, RawFunction};

pub struct RawServlet {
    listener: TcpListener,
    func: RawFunction,
}

impl RawServlet {
    pub fn start(&self) {
        for (string, mut stream) in self {
            let result = (self.func)(string);
            let _ = stream.write_all(result.as_bytes());
        }
    }
}

impl Iterator for &RawServlet {
    type Item = (String, TcpStream);
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.listener.incoming().next();
        if let Some(res) = next {
            if let Ok(stream) = res {
                return match parse_stream(&stream) {
                    Ok(string) => Some((string, stream)),
                    Err(err) => {
                        error!("Error parsing the Stream: {}", err);
                        None
                    }
                };
            } else {
                error!("Error establishing Connection: {}", res.err().unwrap())
            }
        }
        self.next()
    }
}
