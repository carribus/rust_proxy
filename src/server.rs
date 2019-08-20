use std::error::Error;
use std::io::prelude::*;
use std::net::TcpListener;

use crate::http::Request;

const READ_BUFFER_SIZE: usize = 8096;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(host: &str, port: u16) -> Result<Server, Box<dyn Error>> {
        let server = Server {
            listener: TcpListener::bind(format!("{}:{}", host, port))?,
        };
        Ok(server)
    }

    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        for stream in self.listener.incoming() {
            let mut buffer = [0; READ_BUFFER_SIZE];
            match stream {
                Ok(stream) => {
                    println!("Incoming connection from {:?}", stream.peer_addr());
                    let mut stream = stream;
                    let mut buf_str = String::new();

                    // TODO: Make this be able to read all sizes of requests without relying 
                    //       on the buffer to read in one go
                    stream.read(&mut buffer)?;
                    buf_str.push_str(&String::from_utf8_lossy(&buffer[..]));

                    println!("buf_str: {}", buf_str);
                    if let Ok(req) = Request::from(&buf_str) {
                        println!("{}\nRequest: {:?}", "-".repeat(50), req);
                    }
                },
                Err(e) => {
                    println!("Connection failed");
                }
            }
        }

        Ok(())
    }
}