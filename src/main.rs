use std::io::{Read, Write};
#[allow(unused_imports)]
use std::net::{TcpListener,TcpStream};
use anyhow::{anyhow};

fn main() {
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                stream_handler(&_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

struct Request {
    request_line: RequestLine,
    // raw: String,
    // headers: Option<Vec<String>>
}

struct RequestLine {
    path: String,
    // method: String,
    // protocol: String
}

fn stream_reader(mut stream: &TcpStream) -> String {
    let mut buf = String::new();
    stream.read_to_string(&mut buf).expect("Could not read stream.");
    buf
}

fn stream_parser(buf: String) -> anyhow::Result<Request> {
    let request = buf
        .split("\r\n")
        .map(String::from)
        .collect::<Vec<String>>();
    match request {
        n if n.len() > 1 => {
            Ok(Request {
                request_line: request_line_parser(&n[0]),
                // raw: buf.clone(),
                // headers: Some(n[1.. ]
                //     .iter()
                //     .map(String::from)
                //     .collect::<Vec<String>>())
            })
        }
        _ => {
            Err(anyhow!("Empty request"))
        }
    }
}

fn stream_write_ok(mut stream: &TcpStream) {
    let buf = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();
    stream.write_all(buf).expect("failed to write to stream");
}

fn stream_write_error(mut stream: &TcpStream) {
    let buf = "HTTP/1.1 404 Not Found\r\n\r\n".as_bytes();
    stream.write_all(buf).expect("failed to write to stream");
}

fn stream_handler(stream: &TcpStream) {
    let stream_str = stream_reader(stream);
    let req = stream_parser(stream_str);
    match req {
        Ok(r) => {
            request_handler(stream, r);
        }
        Err(e) => {println!("{}", e)}
    }
}

fn request_line_parser(req_line: &str) -> RequestLine {
    let req_split = req_line
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();
    RequestLine {
        path: req_split[1].clone(),
        // method: req_split[0].clone(),
        // protocol: req_split[2].clone()
    }
}

fn request_handler(stream: &TcpStream, req: Request) {
    match req.request_line.path.as_str() {
        "/" => {
            stream_write_ok(stream);
        }
        _ => {
            stream_write_error(stream);
        }
    }
}
