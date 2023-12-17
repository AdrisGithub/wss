use std::io::Write;
use std::net::{TcpListener, TcpStream};

use aul::error;
use aul::level::Level;
use aul::log;
use wjp::{Deserialize, Serialize};

use crate::helper::{parse_stream, SerializableFunction};

pub struct SerializeServlet<I: Deserialize, O: Serialize> {
    listener: TcpListener,
    func: SerializableFunction<I, O>,
}

impl<I: Deserialize, O: Serialize> SerializeServlet<I, O> {
    pub fn start(&self) {
        for (input, mut stream) in self {
            let output = (self.func)(input);
            let _ = stream.write(output.json().as_bytes());
        }
    }
}

impl<I: Deserialize, O: Serialize> Iterator for &SerializeServlet<I, O> {
    type Item = (I, TcpStream);
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.listener.incoming().next();
        if let Some(res) = next {
            if let Ok(stream) = res {
                match parse_stream(&stream) {
                    Ok(string) => match I::deserialize(string) {
                        Ok(obk) => Some(obk),
                        Err(err) => {
                            error!("Error parsing the Payload: {:?}", err);
                            None
                        }
                    },
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
