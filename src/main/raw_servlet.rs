use std::io::Write;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use aul::error;
use aul::level::Level;
use aul::log;

use crate::error::WBSLError;
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
    pub fn builder() -> RawServletBuilder {
        RawServletBuilder::default()
    }
}

#[derive(Default)]
pub struct RawServletBuilder {
    listener: Option<TcpListener>,
    func: Option<RawFunction>,
}

impl RawServletBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_func(mut self, func: RawFunction) -> Self {
        self.func = Some(func);
        self
    }
    pub fn bind<A: ToSocketAddrs>(mut self, addr: A) -> Result<RawServlet, WBSLError> {
        let binding = TcpListener::bind(addr);
        match binding {
            Ok(listener) => self.listener = Some(listener),
            Err(_error) => return Err(WBSLError),
        }
        self.build()
    }
    pub fn with(mut self, listener: TcpListener) -> Self {
        self.listener = Some(listener);
        self
    }
    pub fn build(self) -> Result<RawServlet, WBSLError> {
        if self.validate() {
            Ok(RawServlet {
                func: self.func.unwrap(),
                listener: self.listener.unwrap(),
            })
        } else {
            Err(WBSLError)
        }
    }
    fn validate(&self) -> bool {
        self.listener.is_some() && self.func.is_some()
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
