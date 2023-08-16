use std::fmt::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

pub struct Server {
    port: u16,
    address: String,
}

impl Server {
    pub fn new(address: String, port: u16) -> Server {
        Server {
            port,
            address,
        }
    }
    pub fn start(&mut self) -> Result<(), Error> {
        let listener = TcpListener::bind(format!("{}:{}", self.address, self.port)).unwrap();
        listener.incoming().flatten().for_each(|mut stream| {
            println!("{:#?}", self.handle_request(&stream));
            self.send_simple_response(&mut stream);
            stream.shutdown(std::net::Shutdown::Both).unwrap();
        });
        Ok::<(), Error>(())
    }
    pub fn handle_request(&self, request: &TcpStream) -> Vec<String> {
        let reader = BufReader::new(request);
        reader.lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect::<Vec<String>>()
    }
    pub fn send_simple_response(&self, stream: &mut TcpStream) {
        const RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(RESPONSE.as_bytes()).unwrap();
    }
}

impl Default for Server {
    fn default() -> Self {
        Server {
            port: 8080,
            address: "127.0.0.1".to_string(),
        }
    }
}