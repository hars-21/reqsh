use std::collections::HashMap;

pub struct ShellState {
    base_url: Option<String>,
    headers: HashMap<String, String>,
}

impl ShellState {
    pub fn new() -> Self {
        ShellState {
            base_url: None,
            headers: HashMap::new(),
        }
    }

    pub fn get_base_url(&self) -> Option<&str> {
        self.base_url.as_deref()
    }

    pub fn set_base_url(&mut self, url: &str) {
        self.base_url = Some(url.to_string());
    }

    pub fn set_header(&mut self, key: String, value: String) -> Option<String> {
        self.headers.insert(key, value)
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
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
}
