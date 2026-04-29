use std::collections::HashMap;

pub struct Request {
    pub method: Method,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    pub fn new(method: Method, url: String) -> Self {
        Self {
            method,
            url,
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

pub enum Method {
    GET,
    POST,
    PUT,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
        }
    }
}
