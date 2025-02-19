use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listner.incoming() {
        connection_control(stream.unwrap());
    }
    println!("Hello, world!");
}

fn connection_control(mut stream: TcpStream) {
    let buf = BufReader::new(&stream);
    let status_lines = buf.lines().next().unwrap().unwrap();
    let (filename, request_format) = match &status_lines[..] {
        "GET / HTTP/1.1" => ("hello.html", "HTTP/1.1 200 OK"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("hello.html", "HTTP/1.1 200 OK")
        }
        _ => ("notfound.html", "HTTP/1.1 404 NOT FOUND"),
    };
    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();
    // dbg!(content, length, request_format);
    let response = format!("{request_format}\r\nContent-Length:{length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
