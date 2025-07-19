use crate::controller::responses::Request;
use anyhow::anyhow;

pub enum Methods {
    Get,
    Post,
}

#[derive(Debug)]
pub struct RequestLine {
    pub path: String,
    pub raw_method: String,
    // protocol: String
}

impl RequestLine {
    pub fn get_sub_path(&self, prefix: &str) -> String {
        self.path
            .strip_prefix(prefix)
            .unwrap_or_default()
            .to_string()
    }

    pub fn method(&self) -> Methods {
        match self.raw_method.as_str() {
            "GET" => Methods::Get,
            "POST" => Methods::Post,
            e => panic!("Method, {}, not handled.", e),
        }
    }
}

pub fn request_line_parser(req_line: &str) -> RequestLine {
    let req_split = req_line
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();
    RequestLine {
        path: req_split[1].clone(),
        raw_method: req_split[0].clone(),
        // protocol: req_split[2].clone()
    }
}

pub fn user_agent_parser(line: &str) -> String {
    let str_split = line.split_once(" ").unwrap_or_default();
    str_split.1.to_string()
}

pub fn stream_parser(buf: String) -> anyhow::Result<Request> {
    let request = buf.split("\r\n").map(String::from).collect::<Vec<String>>();
    match request {
        n if n.len() > 1 => {
            Ok(Request {
                request_line: request_line_parser(&n[0]),
                user_agent: user_agent_parser(&n[2]),
                body: n.last().cloned().unwrap_or_default(), // raw: buf.clone(),
                                         // headers: Some(n[1.. ]
                                         //     .iter()
                                         //     .map(String::from)
                                         //     .collect::<Vec<String>>())
            })
        }
        _ => Err(anyhow!("Empty request")),
    }
}
