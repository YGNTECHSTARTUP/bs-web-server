use bs_web_server::ThreadPool;
use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use thiserror::Error;
#[derive(Debug, Error)]
enum ServerErrors {
    #[error("Connection Establishment Failed")]
    ConnectionFailed,
    #[error("Invalid Stream")]
    StreamError,
    #[error("Invalid Status line")]
    InvalidStatus,
    #[error("IO Error:{0}")]
    Io(#[from] io::Error),
}

fn map_io_error(err: io::Error) -> ServerErrors {
    match err.kind() {
        io::ErrorKind::PermissionDenied => ServerErrors::ConnectionFailed,
        io::ErrorKind::NotFound => ServerErrors::InvalidStatus,
        io::ErrorKind::ConnectionReset => ServerErrors::ConnectionFailed,
        _ => ServerErrors::Io(err),
    }
}

fn main() -> Result<(), ServerErrors> {
    let listner = TcpListener::bind("127.0.0.1:7878").map_err(map_io_error)?;
    let threadpool = ThreadPool::new(5);
    for stream in listner.incoming() {
        threadpool.execute(|| connection_control(stream.unwrap()))
    }
    Ok(())
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
