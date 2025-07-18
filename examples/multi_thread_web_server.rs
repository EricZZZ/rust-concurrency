use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rust_concurrency::model::ThreadPool;
fn main() {
    // 监听地址：127.0.0.1:8787
    let listener = TcpListener::bind("127.0.0.1:8787").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| handle_connection(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap_or_else(|| Ok(String::from("")))
        .unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
