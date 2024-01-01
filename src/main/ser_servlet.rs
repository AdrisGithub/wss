use std::io::Write;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use aul::error;
use aul::level::Level;
use aul::log;
use wjp::{Deserialize, Serialize};

use crate::error::WBSLError;
use crate::helper::{parse_stream, SerializableFunction};
#[derive(Debug)]
pub struct SerializeServlet<I: Deserialize, O: Serialize> {
    listener: TcpListener,
    func: SerializableFunction<I, O>,
}
#[derive(Debug)]
pub struct SerializeServletBuilder<I: Deserialize, O: Serialize> {
    listener: Option<TcpListener>,
    func: Option<SerializableFunction<I, O>>,
}

impl<I: Deserialize, O: Serialize> SerializeServletBuilder<I, O> {
    pub const fn new() -> Self {
        Self {
            listener: None,
            func: None,
        }
    }

    pub fn with_func(mut self, func: SerializableFunction<I, O>) -> Self {
        self.func = Some(func);
        self
    }
    pub fn bind<A: ToSocketAddrs>(mut self, addr: A) -> Result<SerializeServlet<I, O>, WBSLError> {
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
    pub fn build(self) -> Result<SerializeServlet<I, O>, WBSLError> {
        if self.validate() {
            Ok(SerializeServlet {
                func: self.func.unwrap(),
                listener: self.listener.unwrap(),
            })
        } else {
            Err(WBSLError)
        }
    }
    const fn validate(&self) -> bool {
        self.listener.is_some() && self.func.is_some()
    }
}

impl<I: Deserialize, O: Serialize> Default for SerializeServletBuilder<I, O> {
    fn default() -> Self {
        Self {
            func: None,
            listener: None,
        }
    }
}

impl<I: Deserialize, O: Serialize> SerializeServlet<I, O> {
    pub fn start(&self) {
        for (input, mut stream) in self {
            let output = (self.func)(input);
            let _ = stream.write(output.json().as_bytes());
        }
    }
    pub fn builder() -> SerializeServletBuilder<I, O> {
        SerializeServletBuilder::new()
    }
}

impl<I: Deserialize, O: Serialize> Iterator for &SerializeServlet<I, O> {
    type Item = (I, TcpStream);
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.listener.incoming().next();
        if let Some(res) = next {
            if let Ok(stream) = res {
                return match parse_stream(&stream) {
                    Ok(string) => match I::deserialize_str(string.as_str()) {
                        Ok(obj) => Some((obj, stream)),
                        Err(err) => {
                            error!("Error parsing the Payload: {:?}", err);
                            self.next()
                        }
                    },
                    Err(err) => {
                        error!("Error parsing the Stream: {}", err);
                        self.next()
                    }
                };
            } else {
                error!("Error establishing Connection: {}", res.err().unwrap())
            }
        }
        self.next()
    }
}
