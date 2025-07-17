use std::net::TcpListener;
use crate::{stream::stream_handler, threads::ThreadPool};

mod responses;
mod stream;
mod parsers;
mod threads;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                pool.execute(|| {
                    stream_handler(_stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
