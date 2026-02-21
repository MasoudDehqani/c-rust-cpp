use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread::sleep,
    time::Duration,
};

use multithreaded_web_server::ThreadPool;

mod async_impl;
mod old_impl;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || handle_http_messaging(stream));
    }
}

fn handle_http_messaging(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let maybe_request_line = request.get(0);

    let (status, content_file_path) = match maybe_request_line {
        Some(request_line) => {
            if request_line == "GET / HTTP/1.1" {
                ("200 OK", "hello.html")
            } else if request_line == "GET /about HTTP/1.1" {
                ("200 OK", "about.html")
            } else if request_line == "GET /slow HTTP/1.1" {
                sleep(Duration::from_millis(5000));
                ("200 OK", "hello.html")
            } else {
                ("404 NOT FOUND", "404.html")
            }
        }
        None => ("400 BAD REQUEST", "404.html"),
    };

    let mut content = String::new();
    let mut content_file = File::open(content_file_path).unwrap();
    content_file.read_to_string(&mut content).unwrap();
    let content_length = content.len();

    let status_line = format!("HTTP/1.1 {status}");
    let response = format!("{status_line}\r\nContent-Length:{content_length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
