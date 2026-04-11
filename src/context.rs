use std::collections::HashMap;

pub struct RequestContext {
    base_url: Option<String>,
    saved_requests: HashMap<String, Request>,
}

pub struct Request {
    pub method: RequestMethod,
    pub url: String,
    pub _headers: Vec<(String, String)>,
    pub _body: Option<String>,
}

pub enum RequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl RequestMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            RequestMethod::GET => "GET",
            RequestMethod::POST => "POST",
            RequestMethod::PUT => "PUT",
            RequestMethod::DELETE => "DELETE",
        }
    }
}

impl RequestContext {
    pub fn new() -> Self {
        RequestContext {
            base_url: None,
            saved_requests: HashMap::new(),
        }
    }

    pub fn get_base_url(&self) -> Option<&str> {
        self.base_url.as_deref()
    }

    pub fn set_base_url(&mut self, url: &str) {
        self.base_url = Some(url.to_string());
    }

    pub fn save_request(&mut self, name: &str, method: RequestMethod, url: &str) {
        let request = Request {
            method,
            url: url.to_string(),
            _headers: Vec::new(),
            _body: None,
        };
        self.saved_requests.insert(name.to_string(), request);
    }

    pub fn get_saved_request(&self, name: &str) -> Option<&Request> {
        self.saved_requests.get(name)
    }

    pub fn list_saved_requests(&self) -> Vec<String> {
        self.saved_requests.keys().cloned().collect()
    }

    pub fn delete_saved_request(&mut self, name: &str) -> bool {
        self.saved_requests.remove(name).is_some()
    }
}
