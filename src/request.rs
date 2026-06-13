use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    pub fn new(method: Method, path: String) -> Self {
        Self {
            method,
            path,
            headers: HashMap::new(),
            params: HashMap::new(),
            body: None,
        }
    }

    pub fn set_body(&mut self, content: String) {
        self.body = Some(content);
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn set_param(&mut self, key: String, value: String) {
        self.params.insert(key, value);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_request_starts_empty() {
        let req = Request::new(Method::GET, "/users".to_string());

        assert_eq!(req.method, Method::GET);
        assert_eq!(req.path, "/users");
        assert!(req.headers.is_empty());
        assert!(req.body.is_none());
    }

    #[test]
    fn set_body_updates_body() {
        let mut req = Request::new(Method::POST, "/users".to_string());

        req.set_body("{\"name\":\"john\"}".to_string());

        assert_eq!(req.body, Some("{\"name\":\"john\"}".to_string()));
    }

    #[test]
    fn set_header_adds_header() {
        let mut req = Request::new(Method::GET, "/users".to_string());

        req.set_header("Content-Type".to_string(), "application/json".to_string());

        assert_eq!(
            req.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn set_param_adds_param() {
        let mut req = Request::new(Method::GET, "/users".to_string());
        req.set_param("page".to_string(), "1".to_string());
        assert_eq!(req.params.get("page"), Some(&"1".to_string()));
    }
}
