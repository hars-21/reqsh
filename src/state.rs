use std::{collections::HashMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::request::Request;

#[derive(Serialize, Deserialize)]
pub struct ShellState {
    base_url: Option<String>,
    headers: HashMap<String, String>,
    variables: HashMap<String, String>,
    last_request: Option<Request>,
    saved_requests: HashMap<String, Request>,
    timeout_secs: Option<u64>,
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
            timeout_secs: None,
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

    fn state_file_path() -> PathBuf {
        let home = dirs::home_dir().expect("could not determine home directory");
        home.join(".reqsh_state.json")
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::state_file_path();
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize state: {}", e))?;
        fs::write(&path, json).map_err(|e| format!("Failed to write state file: {}", e))?;
        Ok(())
    }

    pub fn load() -> Self {
        let path = Self::state_file_path();
        if path.exists() {
            let contents = fs::read_to_string(&path).unwrap_or_default();
            match serde_json::from_str(&contents) {
                Ok(state) => return state,
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to parse state file: {}. Starting fresh.",
                        e
                    );
                }
            }
        }
        Self::new()
    }

    pub fn get_request(&self, name: &str) -> Option<&Request> {
        self.saved_requests.get(name)
    }

    pub fn get_all_requests(&self) -> &HashMap<String, Request> {
        &self.saved_requests
    }

    pub fn remove_request(&mut self, name: &str) -> Result<(), String> {
        self.saved_requests
            .remove(name)
            .ok_or_else(|| format!("no saved request: {name}"))?;
        Ok(())
    }

    pub fn rename_request(&mut self, existing_name: &str, new_name: String) -> Result<(), String> {
        if let Some(value) = self.saved_requests.remove(existing_name) {
            self.saved_requests.insert(new_name, value);
            Ok(())
        } else {
            Err(format!("no saved request: {existing_name}"))
        }
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

    pub fn get_timeout(&self) -> Option<u64> {
        self.timeout_secs
    }

    pub fn set_timeout(&mut self, secs: u64) {
        self.timeout_secs = Some(secs);
    }

    pub fn clear(&mut self) {
        self.base_url = None;
        self.headers.clear();
        self.variables.clear();
        self.last_request = None;
        self.saved_requests.clear();
        self.timeout_secs = None;
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

    #[test]
    fn remove_request_deletes_it() {
        use crate::request::Method;
        let mut state = ShellState::new();
        let req = crate::request::Request::new(Method::GET, "/test".to_string());
        state.set_last_request(req);
        state.save_request("myreq".to_string()).unwrap();
        assert!(state.remove_request("myreq").is_ok());
        assert!(state.get_request("myreq").is_none());
    }

    #[test]
    fn remove_request_missing_returns_error() {
        let mut state = ShellState::new();
        assert!(state.remove_request("nonexistent").is_err());
    }

    #[test]
    fn rename_request_renames_it() {
        use crate::request::Method;
        let mut state = ShellState::new();
        let req = crate::request::Request::new(Method::GET, "/test".to_string());
        state.set_last_request(req);
        state.save_request("old-name".to_string()).unwrap();
        assert!(
            state
                .rename_request("old-name", "new-name".to_string())
                .is_ok()
        );
        assert!(state.get_request("old-name").is_none());
        assert!(state.get_request("new-name").is_some());
    }

    #[test]
    fn rename_request_missing_returns_error() {
        let mut state = ShellState::new();
        assert!(
            state
                .rename_request("nonexistent", "new".to_string())
                .is_err()
        );
    }

    #[test]
    fn save_and_load_state_roundtrip() {
        use crate::request::Method;
        use std::fs;

        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("reqsh_test_state.json");

        let _ = fs::remove_file(&temp_file);

        let mut state = ShellState::new();
        state.set_base_url("https://api.test.com");
        state.set_header("Auth".to_string(), "Token123".to_string());
        state.set_variable("user".to_string(), "admin".to_string());
        state.set_timeout(60);

        let req = crate::request::Request::new(Method::POST, "/login".to_string());
        state.set_last_request(req);
        state.save_request("login_req".to_string()).unwrap();

        let json = serde_json::to_string_pretty(&state).unwrap();
        fs::write(&temp_file, &json).unwrap();

        let loaded_json = fs::read_to_string(&temp_file).unwrap();
        let loaded_state: ShellState = serde_json::from_str(&loaded_json).unwrap();

        assert_eq!(loaded_state.get_base_url(), Some("https://api.test.com"));
        assert_eq!(
            loaded_state.get_headers().get("Auth"),
            Some(&"Token123".to_string())
        );
        assert_eq!(
            loaded_state.get_variable("user"),
            Some(&"admin".to_string())
        );
        assert_eq!(loaded_state.get_timeout(), Some(60));

        assert!(loaded_state.get_request("login_req").is_some());

        let _ = fs::remove_file(&temp_file);
    }

    #[test]
    fn load_missing_file_returns_default_state() {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("reqsh_nonexistent_state.json");
        let _ = std::fs::remove_file(&temp_file);
        let contents = "";
        let state: Result<ShellState, _> = serde_json::from_str(contents);

        assert!(state.is_err());
    }
}
