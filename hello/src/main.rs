use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for req in listener.incoming() {
        let req = req.unwrap();
        con_req(req);
    }
}

fn con_req(mut con: TcpStream) {
    let mut buf = [0; 1024];
    let len = con.read(&mut buf).unwrap();

    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    con.write_all(response.as_bytes()).unwrap();
    con.flush().unwrap();
}
