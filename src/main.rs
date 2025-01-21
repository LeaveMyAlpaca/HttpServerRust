use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
const GET: &[u8; 16] = b"GET / HTTP/1.1\r\n";

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 124];

    stream.read(&mut buffer).unwrap();

    if buffer.starts_with(GET) {
        let contents = fs::read_to_string("index.html").unwrap();
        send_response(contents, "200 OK", stream);
        return;
    }
    let contents = fs::read_to_string("404.html").unwrap();
    send_response(contents, "404 NOT FOUND", stream);
}

fn send_response(contents: String, status: &str, mut stream: TcpStream) {
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
