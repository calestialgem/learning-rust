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

    let get = b"GET / HTTP/1.1\r\n";

    let (status, file) = if buf.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404")
    };

    let contents = fs::read_to_string(format!("hello/{}.html", file)).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );

    con.write_all(response.as_bytes()).unwrap();
    con.flush().unwrap();
}
