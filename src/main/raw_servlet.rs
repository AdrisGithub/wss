use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use aul::error;
use aul::level::Level;
use aul::log;

use crate::error::WBSLError;
use crate::helper::RawFunction;

pub struct RawServlet {
    listener: TcpListener,
    func: RawFunction,
}

impl RawServlet {
    fn parse_stream(stream: &TcpStream) -> Result<String, WBSLError> {
        let mut reader = BufReader::new(stream);
        let received: Vec<u8> = reader.fill_buf().map_err(|_err| WBSLError)?.to_vec();
        reader.consume(received.len());
        String::from_utf8(received).map_err(|_e| WBSLError)
    }
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
                return match RawServlet::parse_stream(&stream) {
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
