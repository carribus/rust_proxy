use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;
use std::cell::{RefCell};

use colored::*;

use crate::http::*; //{Request, RequestMethod};

const READ_BUFFER_SIZE: usize = 8096;

#[derive(Debug)]
struct ConnectError;

impl std::fmt::Display for ConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Proxy CONNECT error handling error")
    }
}

impl Error for ConnectError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}


pub struct Server {
    listener: TcpListener,
    connections: Rc<RefCell<HashMap<String, TcpStream>>>
}

impl Server {
    pub fn new(host: &str, port: u16) -> Result<Server, Box<dyn Error>> {
        let server = Server {
            listener: TcpListener::bind(format!("{}:{}", host, port))?,
            connections: Rc::new(RefCell::new(HashMap::new())),
        };
        Ok(server)
    }

    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = self.handle_incoming_connection(&stream) {
                        println!("{} {}", "ERROR:".bright_yellow(), e);
                    }
                }
                Err(e) => {
                    println!("{} {}", "ERROR: Listener connection failed".bright_red(), e);
                }
            }
        }

        Ok(())
    }

    fn handle_incoming_connection(&self, stream: &TcpStream) -> Result<(), Box<dyn Error>> {

        fn read_headers(reader: &mut dyn BufRead) -> Result<String, Box<dyn Error>> {
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

        fn read_body(req: &Request, reader: &mut dyn BufRead) -> Result<String, Box<dyn Error>> {
            let mut buffer = [0; READ_BUFFER_SIZE];
            let mut body = String::new();

            if let Some(content_length) = req.get_header("Content-Length") {
                let content_length = content_length.parse::<usize>()?;
                let mut content_counter = 0;

                // println!("Reading {} bytes of content: ", content_length);

                loop {
                    content_counter += reader.read(&mut buffer)?;
                    body.push_str(&String::from_utf8_lossy(&buffer[..]));
                    if content_counter >= content_length {
                        break;
                    }
                }

                // println!("Done reading body: {}", body);
            }

            Ok(body)
        }


        if let Ok(peer_addr) = stream.peer_addr() {
            println!("Incoming connection from {:?}", peer_addr);
        }

        let mut req = {
            let mut reader = BufReader::new(stream);
            let mut buf_str = read_headers(&mut reader)?;
            let mut req = Request::from(&buf_str)?;

            // if there is a body, read it
            if req.header_exists("Content-Length") {
                buf_str.clear();
                buf_str = read_body(&req, &mut reader)?;
                req.set_body(Some(buf_str));
            }

            println!("Request: {:?}", req);
            req
        };

        let mut stream = stream;
        if let Err(e) = self.proxy_request(&req, &stream) {
            println!("{} {}", "ERROR: ".bright_yellow(), e);
        }

        // stream.write("200 OK".as_bytes())?;

        Ok(())
    }

    fn proxy_request(&self, req: &Request, stream: &TcpStream) -> Result<(), Box<dyn Error>> {
        println!("Proxying request.. preparing connection");

        match &req.method() {
            RequestMethod::CONNECT => {
                let client = self.handle_connect_request(&req, stream)?; 
                // TODO: You left off here... need to inform the source connection that the tunnel is established
            },
            // RequestMethod::GET => {
            //     println!("Sending GET request");
            // },
            // RequestMethod::POST => {
            //     println!("Sending POST request");
            // },
            _ => unimplemented!(),
        }

        Ok(())
    }

    fn handle_connect_request(&self, req: &Request, stream: &TcpStream) -> Result<TcpStream, Box<dyn Error>> {
        // println!("Handling CONNECT request: {:?}", req);
        let mut conn_map = self.connections.borrow_mut();

        if !conn_map.contains_key(req.uri()) {
            println!("Creating new connection for request: {:?}", req);
            let client = TcpStream::connect(req.uri())?;
            conn_map.insert(req.uri().clone(), client.try_clone()?);
        }

        match conn_map.get(req.uri()) {
            Some(client) => Ok(client.try_clone()?),
            None => Err(Box::new(ConnectError))
        }
    }
}

