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
    println!("Request: {}", String::from_utf8_lossy(&buf[..len]));
}
