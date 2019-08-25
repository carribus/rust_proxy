use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

use colored::*;

use crate::http::{Request, RequestMethod};

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
            match stream {
                Ok(stream) => {
                    let peer_addr = stream.peer_addr();
                    println!("Incoming connection from {:?}", peer_addr);

                    let mut reader = BufReader::new(stream);
                    let mut buf_str = self.read_headers(&mut reader)?;

                    if let Ok(req) = Request::from(&buf_str) {
                        let mut req = req;
                        println!("{}\nRequest: {:?}", "-".repeat(80), req);
                        
                        // if there is a body, read it
                        if req.header_exists("Content-Length") {
                            buf_str.clear();
                            buf_str = self.read_body(&req, &mut reader)?;
                            req.set_body(Some(buf_str));
                        }

                        if let Err(e) = self.proxy_request(&req) {
                            println!("{} {}", "ERROR: ".bright_yellow(), e);
                        }
                    }
                }
                Err(_e) => {
                    println!("Connection failed");
                }
            }
        }

        Ok(())
    }

    fn read_headers(&self, reader: &mut dyn BufRead) -> Result<String, Box<dyn Error>> {
        let mut buf = String::new();

        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;

            if bytes_read == 0 || line == "\r\n" {
                break;
            }
            buf.push_str(&line);
        }

        Ok(buf)
    }

    fn read_body(&self, req: &Request, reader: &mut dyn BufRead) -> Result<String, Box<dyn Error>> {
        let mut buffer = [0; READ_BUFFER_SIZE];
        let mut body = String::new();

        if let Some(content_length) = req.get_header("Content-Length") {
            let content_length = content_length.parse::<usize>()?;
            let mut content_counter = 0;

            println!("Reading {} bytes of content: ", content_length);

            loop {
                content_counter += reader.read(&mut buffer)?;
                body.push_str(&String::from_utf8_lossy(&buffer[..]));
                if content_counter >= content_length {
                    break;
                }
            }

            println!("Done reading body: {}", body);
        }

        Ok(body)
    }

    fn proxy_request(&self, req: &Request) -> Result<(), Box<dyn Error>> {
        println!("Proxying request.. preparing connection");
        let mut client = TcpStream::connect(req.uri())?;

        println!("Selecting method...");
        match &req.method() {
            RequestMethod::CONNECT => {
                println!("Sending CONNECT request");
            },
            RequestMethod::GET => {
                println!("Sending GET request");
            },
            RequestMethod::POST => {
                println!("Sending POST request");
            },
            _ => unimplemented!(),
        }

        Ok(())
    }
}
