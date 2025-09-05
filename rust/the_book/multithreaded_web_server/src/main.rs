use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread::{self, Thread},
    time::Duration,
};

use multithreaded_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    match request_line.as_str() {
        "GET / HTTP/1.1" => write_response("HTTP/1.1 200 OK", "hello.html", stream),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            write_response("HTTP/1 200 OK", "hello.html", stream);
        }
        _ => write_response("HTTP/1.1 404 NOT FOUND", "404.html", stream),
    }
}

fn write_response(status_line: &str, contents_path: &str, mut stream: TcpStream) {
    let contents = fs::read_to_string(contents_path).unwrap();
    let contents_length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {contents_length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
