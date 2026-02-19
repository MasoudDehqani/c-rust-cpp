#![allow(dead_code)]

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    spawn,
    time::{Duration, sleep},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:5454").await.unwrap();
    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                spawn(async {
                    handle_http_messaging(stream).await;
                });
            }
            Err(_) => panic!("Failed to read tcp stream"),
        }
    }
}

async fn handle_http_messaging(stream: TcpStream) {
    let (mut reader, mut writer) = stream.into_split();

    let mut buf = [0u8; 1024];
    let _n = reader.read(&mut buf).await.unwrap();

    let request_line_bytes = buf
        .into_iter()
        .take_while(|&b| b != b'\r')
        .collect::<Vec<_>>();

    let request_line = String::from_utf8(request_line_bytes).unwrap();

    let (content_file_path, status) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("hello.html", "200 OK"),
        "GET /about HTTP/1.1" => ("about.html", "200 OK"),
        "GET /slow HTTP/1.1" => {
            sleep(Duration::from_millis(5000)).await;
            ("hello.html", "200 OK")
        }
        _ => ("404.html", "404 NOT FOUND"),
    };

    let mut content_file = File::open(content_file_path).await.unwrap();
    let mut content = String::new();
    content_file.read_to_string(&mut content).await.unwrap();
    let content_length = content.len();

    let status_line = format!("HTTP/1.1 {status}");
    let response = format!("{status_line}\r\nContent-Length:{content_length}\r\n\r\n{content}");
    writer.write_all(response.as_bytes()).await.unwrap()
}
