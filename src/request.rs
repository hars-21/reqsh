use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    pub fn new(method: Method, path: String) -> Self {
        Self {
            method,
            path,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn set_body(&mut self, content: &str) {
        self.body = Some(content.to_string());
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }
}

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
        }
    }
}
