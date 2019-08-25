use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
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
                    let peer_addr = stream.peer_addr();
                    println!("Incoming connection from {:?}", peer_addr);

                    let mut reader = BufReader::new(stream);
                    let mut buf_str = String::new();
                    let mut terminate_count = 0;

                    // read the headers
                    loop {
                        let mut line = String::new();
                        let bytes_read = reader.read_line(&mut line)?;

                        if bytes_read == 0 || line == "\r\n" {
                            break;
                        }
                        buf_str.push_str(&line);
                    }

                    if let Ok(req) = Request::from(&buf_str) {
                        let mut req = req;
                        println!("{}\nRequest: {:?}", "-".repeat(50), req);
                        
                        // TODO: Refactor!
                        // check if we need to read a body...
                        if req.header_exists("Content-Length") {
                            buf_str.clear();
                            if let Some(content_length) = req.get_header("Content-Length") {
                                let content_length = content_length.parse::<usize>()?;
                                let mut content_counter = 0;

                                println!("Reading {} bytes of content: ", content_length);

                                loop {
                                    content_counter += reader.read(&mut buffer)?;
                                    buf_str.push_str(&String::from_utf8_lossy(&buffer[..]));
                                    if content_counter >= content_length {
                                        break;
                                    }
                                }

                                println!("Done reading content: {}", buf_str);
                                req.set_body(Some(buf_str));
                            }
                        }

                        self.proxy_request(&req);

                    }
                }
                Err(_e) => {
                    println!("Connection failed");
                }
            }
        }

        Ok(())
    }

    fn proxy_request(&self, req: &Request) -> () {

    }
}
