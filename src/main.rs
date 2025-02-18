use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};
fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listner.incoming() {
        connection_control(stream.unwrap());
    }
    println!("Hello, world!");
}

fn connection_control(mut stream: TcpStream) {
    let Buf = BufReader::new(&stream);
    let status_lines = Buf.lines().next().unwrap().unwrap();
    if status_lines == "GET / HTTP/1.1" {
        let content = fs::read_to_string("hello.html").unwrap();
        let length = content.len();
        let request_format = "HTTP/1.1 200 OKS";
        let response = format!("{request_format}\r\nContent-Length:{length}\r\n\r\n{content}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}
