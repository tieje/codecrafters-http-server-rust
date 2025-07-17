use std::{fmt, fs, net::TcpStream};
use crate::{parsers::RequestLine, stream::stream_write_string};

#[derive(Debug)]
pub struct Request {
    pub request_line: RequestLine,
    pub user_agent: String,
    // body: Option<String>,
    // raw: String,
    // headers: Option<Vec<String>>
}

#[derive(Debug)]
pub struct Response {
    pub protocol: String,
    pub code: u16,
    pub status: String,
    pub body: String,
    pub content_type: String
}

impl Default for Response {
    fn default() -> Self {
        Self {
            protocol: String::from("HTTP/1.1"),
            code: 200,
            status: String::from("OK"),
            body: Default::default(),
            content_type: String::from("Content-Type: text/plain\r\n")
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f.write_str(&format!(
            "{} {} {}\r\n",
            self.protocol, self.code, self.status
        ));
        let _ = f.write_str(&self.content_type);
        let _ = f.write_str(&format!("Content-Length: {}\r\n\r\n", self.body.len()));
        let _ = f.write_str(&self.body);
        Ok(())
    }
}

pub fn request_handler(stream: TcpStream, req: Request) {
    let prefix_echo = String::from("/echo/");
    let prefix_files = String::from("/files/");
    match req.request_line.path.as_str() {
        "/" => respond_ok(stream),
        r if r.starts_with(&prefix_echo) => respond_echo(stream, req, &prefix_echo),
        r if r.starts_with("/user-agent") => respond_user_agent(stream, req),
        r if r.starts_with(&prefix_files) => respond_files(stream, req, &prefix_files),
        _ => respond_error(stream),
    }
}

pub fn respond_ok(stream: TcpStream) {
    let buf = String::from("HTTP/1.1 200 OK\r\n\r\n");
    stream_write_string(stream, &buf);
}

pub fn respond_echo(stream: TcpStream, req: Request, prefix: &str) {
    let body = req
        .request_line
        .get_sub_path(prefix);
    let res = Response {
        body,
        ..Default::default()
    };
    stream_write_string(stream, &res.to_string());
}

pub fn respond_user_agent(stream: TcpStream, req: Request) {
    let body = req.user_agent;
    let res = Response {
        body,
        ..Default::default()
    };
    stream_write_string(stream, &res.to_string());
}

pub fn respond_error(stream: TcpStream) {
    let buf = String::from("HTTP/1.1 404 Not Found\r\n\r\n");
    stream_write_string(stream, &buf);
}

pub fn respond_files(stream: TcpStream, req: Request, prefix: &str) {
    let file = req.request_line.get_sub_path(prefix);
    let body = fs::read_to_string(file).unwrap_or_default();
    let content_type = String::from("Content-Type: application/octet-stream\r\n");
    let res = Response {
        body,
        content_type,
        ..Default::default()
    };
    stream_write_string(stream, &res.to_string());
}
