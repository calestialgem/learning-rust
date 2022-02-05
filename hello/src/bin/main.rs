use hello::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    const MAX_REQ: u32 = 10;
    let mut reqs = 0;

    for req in listener.incoming() {
        let req = req.unwrap();
        pool.execute(|| {
            con_req(req);
        });
        reqs += 1;
        if reqs >= MAX_REQ {
            break;
        }
    }
}

fn con_req(mut con: TcpStream) {
    let mut buf = [0; 1024];
    let len = con.read(&mut buf).unwrap();

    const GET: &[u8] = b"GET / HTTP/1.1\r\n";
    const SLEEP: &[u8] = b"GET /sleep HTTP/1.1\r\n";
    const SUCCESS: (&str, &str) = ("HTTP/1.1 200 OK", "hello");
    const FAILURE: (&str, &str) = ("HTTP/1.1 404 NOT FOUND", "404");

    let (status, file) = if buf.starts_with(GET) {
        SUCCESS
    } else if buf.starts_with(SLEEP) {
        thread::sleep(Duration::from_secs(5));
        SUCCESS
    } else {
        FAILURE
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
