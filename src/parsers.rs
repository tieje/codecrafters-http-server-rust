use anyhow::anyhow;
use crate::responses::Request;

#[derive(Debug)]
pub struct RequestLine {
    pub path: String,
    // method: String,
    // protocol: String
}

pub fn request_line_parser(req_line: &str) -> RequestLine {
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
