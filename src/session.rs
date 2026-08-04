use std::{collections::HashMap, fs, path::PathBuf, time::Duration};

use reqwest::{
    Url,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize};

use crate::{
    ast::RequestSpec,
    http::{HttpRequest, to_reqwest_method},
};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Session {
    base_url: Option<String>,
    headers: HashMap<String, String>,
    variables: HashMap<String, String>,
    last_request: Option<RequestSpec>,
    saved_requests: HashMap<String, RequestSpec>,
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
            let base = self.base_url.as_ref().ok_or("base URL is not configured")?;

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
