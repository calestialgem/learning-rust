use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for req in listener.incoming() {
        let req = req.unwrap();
        con_req(req);
    }
}

fn con_req(mut req: TcpStream) {
    let mut buf = [0; 1024];
    let len = req.read(&mut buf).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    req.write_all(response.as_bytes()).unwrap();
    req.flush().unwrap();
}
