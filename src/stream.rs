use std::{io::{Read, Write}, net::TcpStream};

use crate::{parsers::stream_parser, responses::request_handler};

pub fn stream_handler(stream: TcpStream) {
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

pub fn stream_write_string(mut stream: TcpStream, buf: &str) {
    stream
        .write_all(buf.as_bytes())
        .expect("Failed to write to stream.")
}

pub fn stream_reader(mut stream: &TcpStream) -> String {
    let mut buffer = [0; 1024];
    let n = stream
        .read(&mut buffer[..])
        .expect("Could not read bytes from stream.");
    str::from_utf8(&buffer[..n])
        .expect("Could not convert bytes to string")
        .to_string()
}
