use std::{collections::HashMap, fs, path::PathBuf, time::Duration};

use reqwest::{
    Url,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize};

use crate::{
    ast::RequestSpec,
    http::{HttpRequest, HttpResponse, to_reqwest_method},
};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Session {
    base_url: Option<String>,
    headers: HashMap<String, String>,
    variables: HashMap<String, String>,
    last_request: Option<RequestSpec>,
    saved_requests: HashMap<String, RequestSpec>,
    #[serde(skip)]
    last_response: Option<HttpResponse>,
    #[serde(with = "duration_secs")]
    timeout: Option<Duration>,
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load() -> Self {
        let contents = match fs::read_to_string(Self::state_file_path()) {
            Ok(contents) => contents,
            Err(_) => return Self::new(),
        };

        match serde_json::from_str(&contents) {
            Ok(session) => session,
            Err(e) => {
                eprintln!("warning: failed to parse state file, starting fresh: {e}");
                Self::new()
            }
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("failed to serialize state: {e}"))?;

        fs::write(Self::state_file_path(), json)
            .map_err(|e| format!("failed to write state file: {e}"))
    }

    fn state_file_path() -> PathBuf {
        let home = dirs::home_dir().expect("could not determine home directory");
        home.join(".reqsh_state.json")
    }

    pub fn base_url(&mut self) -> &mut Option<String> {
        &mut self.base_url
    }

    pub fn headers(&mut self) -> &mut HashMap<String, String> {
        &mut self.headers
    }

    pub fn variables(&mut self) -> &mut HashMap<String, String> {
        &mut self.variables
    }

    pub fn last_request(&mut self) -> &mut Option<RequestSpec> {
        &mut self.last_request
    }

    pub fn last_response(&mut self) -> &mut Option<HttpResponse> {
        &mut self.last_response
    }

    pub fn saved_requests(&mut self) -> &mut HashMap<String, RequestSpec> {
        &mut self.saved_requests
    }

    pub fn timeout(&mut self) -> &mut Option<Duration> {
        &mut self.timeout
    }

    pub fn save_request(&mut self, name: String) -> Result<(), String> {
        let request = self.last_request.clone().ok_or("no request to save")?;

        self.saved_requests.insert(name, request);
        Ok(())
    }

    pub fn remove_request(&mut self, name: &str) -> Result<(), String> {
        self.saved_requests
            .remove(name)
            .map(|_| ())
            .ok_or_else(|| format!("no saved request: {name}"))
    }

    pub fn rename_request(&mut self, old_name: &str, new_name: String) -> Result<(), String> {
        let request = self
            .saved_requests
            .remove(old_name)
            .ok_or_else(|| format!("no saved request: {old_name}"))?;

        self.saved_requests.insert(new_name, request);
        Ok(())
    }

    pub fn clear_requests(&mut self) {
        self.saved_requests.clear();
    }

    pub fn build_request(&self, spec: RequestSpec) -> Result<HttpRequest, String> {
        let path = self.interpolate(&spec.path)?;

        let mut url = if path.starts_with("http://") || path.starts_with("https://") {
            Url::parse(&path).map_err(|e| e.to_string())?
        } else {
            let base = self.base_url.as_ref().ok_or(
                "base URL is not configured\nset one with `base <url>` or use an absolute URL",
            )?;

            Url::parse(base)
                .map_err(|e| e.to_string())?
                .join(&path)
                .map_err(|e| e.to_string())?
        };

        if !spec.queries.is_empty() {
            let mut pairs = url.query_pairs_mut();
            for query in &spec.queries {
                pairs.append_pair(&query.name, &self.interpolate(&query.value)?);
            }
        }

        let mut headers = HeaderMap::new();

        // Session headers
        for (name, value) in &self.headers {
            headers.insert(
                HeaderName::from_bytes(name.as_bytes()).map_err(|e| e.to_string())?,
                HeaderValue::from_str(value).map_err(|e| e.to_string())?,
            );
        }

        // Request headers override session headers
        for header in spec.headers {
            headers.insert(
                HeaderName::from_bytes(header.name.as_bytes()).map_err(|e| e.to_string())?,
                HeaderValue::from_str(&self.interpolate(&header.value)?)
                    .map_err(|e| e.to_string())?,
            );
        }

        let method = to_reqwest_method(spec.method);

        let body = if spec.body.is_empty() {
            None
        } else {
            Some(self.interpolate(&spec.body)?.into_bytes())
        };

        Ok(HttpRequest {
            method,
            url,
            headers,
            body,
            timeout: self.timeout,
        })
    }

    fn interpolate(&self, input: &str) -> Result<String, String> {
        let mut output = String::new();
        let mut rest = input;

        while let Some(start) = rest.find("{{") {
            output.push_str(&rest[..start]);
            let tail = &rest[start + 2..];

            match tail.find("}}") {
                Some(end) => {
                    let name = tail[..end].trim();
                    let value = self
                        .variables
                        .get(name)
                        .ok_or_else(|| format!("undefined variable: {name}"))?;

                    output.push_str(value);
                    rest = &tail[end + 2..];
                }

                None => return Err(format!("unclosed `{{{{` in: {input}")),
            }
        }

        output.push_str(rest);
        Ok(output)
    }
}

mod duration_secs {
    use std::time::Duration;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.map(|d| d.as_secs()).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = Option::<u64>::deserialize(deserializer)?;
        Ok(secs.map(Duration::from_secs))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::ast::{Header, Method, Query, RequestSpec};

    fn request(method: Method, path: &str) -> RequestSpec {
        RequestSpec {
            method,
            path: path.to_string(),
            headers: Vec::new(),
            queries: Vec::new(),
            body: String::new(),
        }
    }

    fn session_with_base(base: &str) -> Session {
        let mut session = Session::new();
        *session.base_url() = Some(base.to_string());
        session
    }

    #[test]
    fn interpolate_passthrough() {
        let session = Session::new();
        assert_eq!(session.interpolate("hello").unwrap(), "hello");
    }

    #[test]
    fn interpolate_replaces_variable() {
        let mut session = Session::new();
        session
            .variables()
            .insert("name".to_string(), "world".to_string());
        assert_eq!(
            session.interpolate("hello {{name}}").unwrap(),
            "hello world"
        );
    }

    #[test]
    fn interpolate_undefined_variable_errors() {
        let session = Session::new();
        assert!(session.interpolate("{{missing}}").is_err());
    }

    #[test]
    fn interpolate_unclosed_errors() {
        let session = Session::new();
        assert!(session.interpolate("{{unclosed").is_err());
    }

    #[test]
    fn interpolate_multiple_variables() {
        let mut session = Session::new();
        session.variables().insert("a".to_string(), "1".to_string());
        session.variables().insert("b".to_string(), "2".to_string());
        assert_eq!(session.interpolate("{{a}}-{{b}}").unwrap(), "1-2");
    }

    #[test]
    fn build_request_fails_without_base_url() {
        let session = Session::new();
        assert!(
            session
                .build_request(request(Method::Get, "/users"))
                .is_err()
        );
    }

    #[test]
    fn build_request_resolves_relative_path() {
        let session = session_with_base("http://localhost:8123");
        let http = session
            .build_request(request(Method::Get, "/users"))
            .unwrap();
        assert_eq!(http.url.as_str(), "http://localhost:8123/users");
    }

    #[test]
    fn build_request_uses_absolute_url_without_base() {
        let session = Session::new();
        let http = session
            .build_request(request(Method::Get, "http://example.com/foo"))
            .unwrap();
        assert_eq!(http.url.as_str(), "http://example.com/foo");
    }

    #[test]
    fn build_request_appends_queries() {
        let session = session_with_base("http://localhost:8123");
        let mut spec = request(Method::Get, "/users");
        spec.queries = vec![Query {
            name: "page".to_string(),
            value: "1".to_string(),
        }];
        let http = session.build_request(spec).unwrap();
        assert_eq!(http.url.as_str(), "http://localhost:8123/users?page=1");
    }

    #[test]
    fn build_request_interpolates_path() {
        let mut session = session_with_base("http://localhost:8123");
        session
            .variables()
            .insert("id".to_string(), "42".to_string());
        let http = session
            .build_request(request(Method::Get, "/users/{{id}}"))
            .unwrap();
        assert_eq!(http.url.as_str(), "http://localhost:8123/users/42");
    }

    #[test]
    fn build_request_interpolates_header_value() {
        let mut session = session_with_base("http://localhost:8123");
        session
            .variables()
            .insert("token".to_string(), "abc".to_string());
        let mut spec = request(Method::Get, "/users");
        spec.headers = vec![Header {
            name: "Authorization".to_string(),
            value: "Bearer {{token}}".to_string(),
        }];
        let http = session.build_request(spec).unwrap();
        assert_eq!(http.headers.get("authorization").unwrap(), "Bearer abc");
    }

    #[test]
    fn build_request_sets_timeout() {
        let mut session = session_with_base("http://localhost:8123");
        session.timeout().replace(Duration::from_secs(5));
        let http = session
            .build_request(request(Method::Get, "/users"))
            .unwrap();
        assert_eq!(http.timeout, Some(Duration::from_secs(5)));
    }

    #[test]
    fn save_request_needs_last_request() {
        let mut session = Session::new();
        assert!(session.save_request("foo".to_string()).is_err());
    }

    #[test]
    fn save_and_get_request() {
        let mut session = Session::new();
        *session.last_request() = Some(request(Method::Get, "/users"));
        session.save_request("foo".to_string()).unwrap();
        assert_eq!(session.saved_requests().get("foo").unwrap().path, "/users");
    }

    #[test]
    fn remove_request_missing_errors() {
        let mut session = Session::new();
        assert!(session.remove_request("nonexistent").is_err());
    }

    #[test]
    fn rename_request_renames_it() {
        let mut session = Session::new();
        *session.last_request() = Some(request(Method::Get, "/users"));
        session.save_request("old".to_string()).unwrap();
        session.rename_request("old", "new".to_string()).unwrap();
        assert!(session.saved_requests().get("old").is_none());
        assert!(session.saved_requests().get("new").is_some());
    }

    #[test]
    fn rename_request_missing_errors() {
        let mut session = Session::new();
        assert!(
            session
                .rename_request("nonexistent", "new".to_string())
                .is_err()
        );
    }

    #[test]
    fn state_serde_roundtrip() {
        let mut session = session_with_base("http://localhost:8123");
        session
            .headers()
            .insert("Auth".to_string(), "Token123".to_string());
        session
            .variables()
            .insert("user".to_string(), "admin".to_string());
        session.timeout().replace(Duration::from_secs(60));
        *session.last_request() = Some(request(Method::Post, "/login"));
        session.save_request("login".to_string()).unwrap();

        let json = serde_json::to_string(&session).unwrap();
        let mut loaded: Session = serde_json::from_str(&json).unwrap();

        assert_eq!(
            *loaded.base_url(),
            Some("http://localhost:8123".to_string())
        );
        assert_eq!(loaded.headers().get("Auth"), Some(&"Token123".to_string()));
        assert_eq!(loaded.variables().get("user"), Some(&"admin".to_string()));
        assert_eq!(*loaded.timeout(), Some(Duration::from_secs(60)));
        assert!(loaded.saved_requests().contains_key("login"));
    }
}
