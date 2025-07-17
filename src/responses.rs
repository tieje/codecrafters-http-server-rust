use std::{fmt, net::TcpStream};
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

pub fn request_handler(stream: TcpStream, req: Request) {
    let prefix_echo = String::from("/echo/");
    let prefix_user_agent = String::from("/user-agent");
    match req.request_line.path.as_str() {
        "/" => respond_ok(stream),
        r if r.starts_with(&prefix_echo) => respond_echo(stream, req, &prefix_echo),
        r if r.starts_with(&prefix_user_agent) => respond_user_agent(stream, req),
        _ => respond_error(stream),
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

pub fn respond_ok(stream: TcpStream) {
    let buf = String::from("HTTP/1.1 200 OK\r\n\r\n");
    stream_write_string(stream, &buf);
}

pub fn respond_echo(stream: TcpStream, req: Request, prefix: &str) {
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
