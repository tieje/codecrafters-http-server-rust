use crate::{
    controller::services::files,
    parsers::{Methods, RequestLine},
    stream::stream_write_string,
};
use std::{fmt, net::TcpStream};

#[derive(Debug)]
pub struct Request {
    pub request_line: RequestLine,
    pub user_agent: String,
    pub body: String,
    // raw: String,
    // headers: Option<Vec<String>>
}

#[derive(Debug)]
pub struct RequestHandler {
    pub stream: TcpStream,
    pub req: Request,
    pub prefix: String,
}

impl RequestHandler {
    pub fn new(stream: TcpStream, req: Request) -> Self {
        Self {
            stream,
            req,
            prefix: Default::default(),
        }
    }

    pub fn new_prefix(self, prefix: &str) -> Self {
        Self {
            stream: self.stream,
            req: self.req,
            prefix: prefix.to_owned(),
        }
    }

    pub fn route_response(self) {
        let prefix_echo = String::from("/echo/");
        let prefix_files = String::from("/files/");
        match self.req.request_line.path.as_str() {
            "/" => respond_ok(self.stream),
            r if r.starts_with(&prefix_echo) => respond_echo(self.new_prefix(&prefix_echo)),
            r if r.starts_with("/user-agent") => respond_user_agent(self),
            r if r.starts_with(&prefix_files) => respond_files(self.new_prefix(&prefix_files)),
            _ => respond_error(self.stream),
        }
    }
}

#[derive(Debug)]
pub enum ContentTypes {
    TextPlain,
    ApplicationOctetStream
}

impl fmt::Display for ContentTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentTypes::TextPlain => write!(f, "text/plain"),
            ContentTypes::ApplicationOctetStream => write!(f, "application/octet-stream")
        }
    }
}

#[derive(Debug)]
pub enum Statuses {
    Ok,
    Created
}

impl fmt::Display for Statuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statuses::Ok => write!(f, "OK"),
            Statuses::Created => write!(f, "Created")
        }
    }
}

#[derive(Debug)]
pub struct Response {
    pub protocol: String,
    pub code: u16,
    pub status: Statuses,
    pub content_type: Option<ContentTypes>,
    pub body: Option<String>,
    pub content_length: Option<u64>,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            protocol: String::from("HTTP/1.1"),
            code: 200,
            status: Statuses::Ok,
            content_type: None,
            content_length: None,
            body: None,
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f.write_str(&format!(
            "{} {} {}\r\n",
            self.protocol, self.code, self.status
        ));

        if let Some(c) = &self.content_type {
            let _ = f.write_str(&format!("Content-Type: {}\r\n", c));
        }

        if let Some(c) = &self.content_length {
            let _ = f.write_str(&format!("Content-Length: {}\r\n", c));
        }

        let _ = f.write_str("\r\n");

        if let Some(c) = &self.body {
            let _ = f.write_str(&format!("{}\r\n", c));
        }

        Ok(())
    }
}

pub fn respond_ok(stream: TcpStream) {
    let buf = String::from("HTTP/1.1 200 OK\r\n\r\n");
    stream_write_string(stream, &buf);
}

pub fn respond_echo(handle: RequestHandler) {
    let RequestHandler {
        stream,
        req,
        prefix,
    } = handle;

    let body = req.request_line.get_sub_path(&prefix);
    let content_length = Some(body.len() as u64);
    let res = Response {
        content_type: Some(ContentTypes::TextPlain),
        content_length,
        body: Some(body),
        ..Default::default()
    };
    stream_write_string(stream, &res.to_string());
}

pub fn respond_user_agent(handle: RequestHandler) {
    let RequestHandler {
        stream,
        req,
        prefix: _,
    } = handle;

    let body = req.user_agent;
    let content_length = Some(body.len() as u64);
    let res = Response {
        content_type: Some(ContentTypes::TextPlain),
        content_length,
        body: Some(body),
        ..Default::default()
    };
    stream_write_string(stream, &res.to_string());
}

pub fn respond_error(stream: TcpStream) {
    let buf = String::from("HTTP/1.1 404 Not Found\r\n\r\n");
    stream_write_string(stream, &buf);
}

pub fn respond_files(handle: RequestHandler) {
    match handle.req.request_line.method() {
        Methods::Get => files::get(handle),
        Methods::Post => files::post(handle),
    }
}
