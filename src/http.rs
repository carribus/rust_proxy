use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub struct Request {
    method: String,
    uri: String,
    protocol: String,
    headers: HashMap<String, String>
}

impl Request {
    pub fn from(buffer: &str) -> Result<Self, String> {
        let mut lines = buffer.lines();
        let mut req;

        println!("{}\n{}", "-".repeat(80), buffer);

        if let Some(req_line) = lines.next() {
            let (method, uri, protocol) = Self::parse_req_method(&req_line);

            req = Request {
                method: String::from(method),
                uri: String::from(uri),
                protocol: String::from(protocol),
                headers: HashMap::new(),
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

    pub fn method(&self) -> &str {
        &self.method[..]
    }

    fn parse_req_method(line: &str) -> (&str, &str, &str) {
        let mut iter = line.split_whitespace();

        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }

}
