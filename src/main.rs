use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let reader = BufReader::new(&stream);
        let request = reader
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect::<Vec<String>>();
        println!("{:#?}", request);
        let response = "HTTP/1.1 200 OK\r\n\r\n";
            sss
        stream.write_all(response.as_bytes()).unwrap();
    }
}
