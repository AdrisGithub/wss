use std::io::Write;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use crate::error::WBSLError;
use aul::error;
use aul::level::Level;
use aul::log;
use whdp::resp_presets::internal_server_error;
use whdp::{Request, TryRequest};

use crate::helper::HTTPFunction;

pub struct Servlet {
    listener: TcpListener,
    func: HTTPFunction,
}

pub struct ServletBuilder {
    listener: Option<TcpListener>,
    func: Option<HTTPFunction>,
}

impl ServletBuilder {
    pub const fn new() -> Self {
        Self {
            func: None,
            listener: None,
        }
    }
    pub fn with_func(mut self, func: HTTPFunction) -> Self {
        self.func = Some(func);
        self
    }
    pub fn bind<A: ToSocketAddrs>(mut self, addr: A) -> Result<Servlet, WBSLError> {
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
    pub fn build(self) -> Result<Servlet, WBSLError> {
        if self.validate() {
            Ok(Servlet {
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

impl Servlet {
    pub fn start(self) {
        for (req, mut stream) in &self {
            let resp = (self.func)(req);
            let _ = stream.write_all(resp.to_string().as_bytes());
        }
    }

    pub fn builder() -> ServletBuilder {
        ServletBuilder::new()
    }
}

impl Iterator for &Servlet {
    type Item = (Request, TcpStream);
    fn next(&mut self) -> Option<Self::Item> {
        let opt = self.listener.incoming().next();
        if let Some(res) = opt {
            if let Ok(mut stream) = res {
                let res_req = stream.try_to_request();
                if let Ok(req) = res_req {
                    return Some((req, stream));
                } else {
                    let _ = stream.write_all(
                        internal_server_error("Internal Server Error".into())
                            .to_string()
                            .as_bytes(),
                    );
                    error!("Error parsing Request: {}", res_req.err().unwrap())
                }
            } else {
                error!("Error establishing Connection: {}", res.err().unwrap())
            }
        }
        self.next()
    }
}
