use std::collections::HashMap;

use crate::request::{Method, Request};

pub struct RequestContext {
    base_url: Option<String>,
    saved_requests: HashMap<String, Request>,
    env_vars: HashMap<String, String>,
}

impl RequestContext {
    pub fn new() -> Self {
        RequestContext {
            base_url: None,
            saved_requests: HashMap::new(),
            env_vars: HashMap::new(),
        }
    }

    pub fn get_base_url(&self) -> Option<&str> {
        self.base_url.as_deref()
    }

    pub fn set_base_url(&mut self, url: &str) {
        self.base_url = Some(url.to_string());
    }

    pub fn save_request(&mut self, name: &str, method: Method, url: String) {
        let request = Request::new(method, url);
        self.saved_requests.insert(name.to_string(), request);
    }

    pub fn get_saved_request(&self, name: &str) -> Option<&Request> {
        self.saved_requests.get(name)
    }

    pub fn get_saved_request_mut(&mut self, name: &str) -> Option<&mut Request> {
        self.saved_requests.get_mut(name)
    }

    pub fn list_saved_requests(&self) -> Vec<String> {
        self.saved_requests.keys().cloned().collect()
    }

    pub fn delete_saved_request(&mut self, name: &str) -> bool {
        self.saved_requests.remove(name).is_some()
    }

    pub fn save_env_var(&mut self, key: String, value: String) {
        self.env_vars.insert(key, value);
    }

    pub fn get_env_var(&self, key: &str) -> Option<&String> {
        self.env_vars.get(key)
    }

    pub fn list_env_vars(&self) -> Vec<String> {
        self.env_vars.keys().cloned().collect()
    }

    pub fn delete_env_var(&mut self, key: &str) -> Option<String> {
        self.env_vars.remove(key)
    }

    pub fn clear_env_vars(&mut self) {
        self.env_vars.clear()
    }
}
