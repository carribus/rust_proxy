use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub enum RequestMethod {
    CONNECT,
    GET,
    POST,
    UPDATE,
    DELETE,
    PATCH,
    HEAD,
    OPTONS,
}

impl RequestMethod {
    pub fn from_string(s: &str) -> Result<RequestMethod, String> {
        match s {
            "CONNECT" => Ok(RequestMethod::CONNECT),
            "GET" => Ok(RequestMethod::GET),
            "POST" => Ok(RequestMethod::POST),
            "UPDATE" => Ok(RequestMethod::UPDATE),
            "DELETE" => Ok(RequestMethod::DELETE),
            "PATCH" => Ok(RequestMethod::PATCH),
            "HEAD" => Ok(RequestMethod::HEAD),
            "OPTIONS" => Ok(RequestMethod::OPTONS),
            _ => Err(format!("Unknown HTTP method found: {}", s))
        }
    }
}

#[derive(Debug)]
pub struct Request {
    method: RequestMethod,
    uri: String,
    protocol: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Request {
    pub fn from(buffer: &str) -> Result<Self, String> {
        let mut lines = buffer.lines();
        let mut req;

        println!("{}\n{}", "-".repeat(80), buffer);

        if let Some(req_line) = lines.next() {
            let (method, uri, protocol) = Self::parse_req_method(&req_line);

            req = Request {
                method: RequestMethod::from_string(method)?,
                uri: String::from(uri),
                protocol: String::from(protocol),
                headers: HashMap::new(),
                body: None,
            };

            while let Some(line) = lines.next() {
                match line.len() {
                    0 => break,
                    _ => {
                        let kv = line.split(':').map(|v| v.trim()).collect::<Vec<_>>();
                        req.headers.insert(kv[0].to_string(), kv[1].to_string());
                    }
                }
            }
        } else {
            return Err("No method headers present in request".to_string());
        }

        Ok(req)
    }

    pub fn uri(&self) -> &String {
        &self.uri
    }

    pub fn set_body(&mut self, body: Option<String>) {
        self.body = body
    }

    pub fn method(&self) -> &RequestMethod {
        &self.method
    }

    pub fn get_header(&self, header: &str) -> Option<&String> {
        self.headers.get(header)
    }

    pub fn header_exists(&self, header: &str) -> bool {
        self.headers.contains_key(header)
    }

    fn parse_req_method(line: &str) -> (&str, &str, &str) {
        let mut iter = line.split_whitespace();

        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }

}
