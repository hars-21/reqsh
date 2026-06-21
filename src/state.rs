use std::collections::HashMap;

use crate::request::Request;

pub struct ShellState {
    base_url: Option<String>,
    headers: HashMap<String, String>,
    variables: HashMap<String, String>,
    last_request: Option<Request>,
    saved_requests: HashMap<String, Request>,
}

impl Default for ShellState {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellState {
    pub fn new() -> Self {
        ShellState {
            base_url: None,
            headers: HashMap::new(),
            variables: HashMap::new(),
            last_request: None,
            saved_requests: HashMap::new(),
        }
    }

    pub fn set_last_request(&mut self, req: Request) {
        self.last_request = Some(req);
    }

    pub fn save_request(&mut self, name: String) -> Result<(), String> {
        let req = self
            .last_request
            .as_ref()
            .ok_or_else(|| "no request to save - execute a request first".to_string())?
            .clone();
        self.saved_requests.insert(name, req);
        Ok(())
    }

    pub fn get_request(&self, name: &str) -> Option<&Request> {
        self.saved_requests.get(name)
    }

    pub fn get_all_requests(&self) -> &HashMap<String, Request> {
        &self.saved_requests
    }

    pub fn get_base_url(&self) -> Option<&str> {
        self.base_url.as_deref()
    }

    pub fn set_base_url(&mut self, url: &str) {
        self.base_url = Some(url.to_string());
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn remove_header(&mut self, key: &str) {
        self.headers.remove(key);
    }

    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }

    pub fn get_variables(&self) -> &HashMap<String, String> {
        &self.variables
    }

    pub fn set_variable(&mut self, name: String, value: String) {
        self.variables.insert(name, value);
    }

    pub fn remove_variable(&mut self, name: &str) {
        self.variables.remove(name);
    }

    pub fn clear(&mut self) {
        self.base_url = None;
        self.headers.clear();
        self.variables.clear();
        self.last_request = None;
        self.saved_requests.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_state_starts_empty() {
        let ctx = ShellState::new();
        assert!(ctx.base_url.is_none());
        assert!(ctx.headers.is_empty());
        assert!(ctx.variables.is_empty());
    }

    #[test]
    fn set_base_url_updates_state() {
        let mut state = ShellState::new();

        state.set_base_url("https://example.com");

        assert_eq!(state.get_base_url(), Some("https://example.com"));
    }

    #[test]
    fn set_header_adds_header() {
        let mut state = ShellState::new();

        state.set_header("Content-Type".to_string(), "application/json".to_string());

        assert_eq!(
            state.get_headers().get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn set_variables_add_variable() {
        let mut state = ShellState::new();

        state.set_variable("Token".to_string(), "random-auth-token".to_string());

        assert_eq!(
            state.get_variable("Token"),
            Some(&"random-auth-token".to_string())
        );
    }

    #[test]
    fn remove_variable_removes_it() {
        let mut state = ShellState::new();
        state.set_variable("key".to_string(), "val".to_string());
        state.remove_variable("key");
        assert!(state.get_variable("key").is_none());
    }

    #[test]
    fn remove_header_removes_it() {
        let mut state = ShellState::new();
        state.set_header("X-Foo".to_string(), "bar".to_string());
        state.remove_header("X-Foo");
        assert!(state.get_headers().get("X-Foo").is_none());
    }

    #[test]
    fn save_request_needs_last_request() {
        let mut state = ShellState::new();
        assert!(state.save_request("name".to_string()).is_err());
    }

    #[test]
    fn save_and_get_request() {
        use crate::request::Method;
        let mut state = ShellState::new();
        let req = crate::request::Request::new(Method::GET, "/test".to_string());
        state.set_last_request(req);
        state.save_request("myreq".to_string()).unwrap();
        let saved = state.get_request("myreq");
        assert!(saved.is_some());
        assert_eq!(saved.unwrap().path, "/test");
    }
}
