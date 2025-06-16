#[allow(unused_imports)]
use anyhow::anyhow;
use std::fmt;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                stream_handler(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

#[derive(Debug)]
struct Request {
    request_line: RequestLine,
    user_agent: String,
    // body: Option<String>,
    // raw: String,
    // headers: Option<Vec<String>>
}

#[derive(Debug)]
struct RequestLine {
    path: String,
    // method: String,
    // protocol: String
}

#[derive(Debug)]
struct Response {
    protocol: String,
    code: u16,
    status: String,
    body: String,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            protocol: String::from("HTTP/1.1"),
            code: 200,
            status: String::from("OK"),
            body: Default::default(),
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f.write_str(&format!(
            "{} {} {}\r\n",
            self.protocol, self.code, self.status
        ));
        let _ = f.write_str("Content-Type: text/plain\r\n");
        let _ = f.write_str(&format!("Content-Length: {}\r\n\r\n", self.body.len()));
        let _ = f.write_str(&self.body);
        Ok(())
    }
}

fn stream_reader(mut stream: &TcpStream) -> String {
    let mut buffer = [0; 1024];
    let n = stream
        .read(&mut buffer[..])
        .expect("Could not read bytes from stream.");
    str::from_utf8(&buffer[..n])
        .expect("Could not convert bytes to string")
        .to_string()
}

fn stream_parser(buf: String) -> anyhow::Result<Request> {
    let request = buf.split("\r\n").map(String::from).collect::<Vec<String>>();
    match request {
        n if n.len() > 1 => {
            Ok(Request {
                request_line: request_line_parser(&n[0]),
                user_agent: user_agent_parser(&n[2]), // body: n.last().cloned(), // raw: buf.clone(),
                                                      // headers: Some(n[1.. ]
                                                      //     .iter()
                                                      //     .map(String::from)
                                                      //     .collect::<Vec<String>>())
            })
        }
        _ => Err(anyhow!("Empty request")),
    }
}

fn stream_handler(stream: TcpStream) {
    let stream_str = stream_reader(&stream);
    // println!("stream_str: {}", stream_str);
    let req = stream_parser(stream_str);
    // println!("req: {:#?}", req);
    match req {
        Ok(r) => {
            request_handler(stream, r);
        }
        Err(e) => {
            println!("{}", e)
        }
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

fn user_agent_parser(line: &str) -> String {
    let str_split = line.split_once(" ").unwrap_or_default();
    str_split.1.to_string()
}

fn request_handler(stream: TcpStream, req: Request) {
    let prefix_echo = String::from("/echo/");
    let prefix_user_agent = String::from("/user-agent");
    match req.request_line.path.as_str() {
        "/" => respond_ok(stream),
        r if r.starts_with(&prefix_echo) => respond_echo(stream, req, &prefix_echo),
        r if r.starts_with(&prefix_user_agent) => respond_user_agent(stream, req),
        _ => respond_error(stream),
    }
}

fn respond_ok(stream: TcpStream) {
    let buf = String::from("HTTP/1.1 200 OK\r\n\r\n");
    stream_write_string(stream, &buf);
}

fn respond_echo(stream: TcpStream, req: Request, prefix: &str) {
    let body = req
        .request_line
        .path
        .strip_prefix(prefix)
        .unwrap_or_default()
        .to_string();
    let res = Response {
        body,
        ..Default::default()
    };
    stream_write_string(stream, &res.to_string());
}

fn respond_user_agent(stream: TcpStream, req: Request) {
    let body = req.user_agent;
    let res = Response {
        body,
        ..Default::default()
    };
    stream_write_string(stream, &res.to_string());
}

fn respond_error(stream: TcpStream) {
    let buf = String::from("HTTP/1.1 404 Not Found\r\n\r\n");
    stream_write_string(stream, &buf);
}

fn stream_write_string(mut stream: TcpStream, buf: &str) {
    stream
        .write_all(buf.as_bytes())
        .expect("Failed to write to stream.")
}
