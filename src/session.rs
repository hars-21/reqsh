use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    time::Duration,
};

use reqwest::{
    Url,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize};

use crate::{
    ast::{HeaderCommand, RequestSpec, SessionCommand, VariableCommand},
    http::{HttpRequest, to_reqwest_method},
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Session {
    base_url: Option<String>,
    headers: HashMap<String, String>,
    variables: HashMap<String, String>,
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
        let json =
            serde_json::to_string_pretty(self).map_err(|e| format!("failed to serialize state: {e}"))?;

        fs::write(Self::state_file_path(), json).map_err(|e| format!("failed to write state file: {e}"))
    }

    fn state_file_path() -> PathBuf {
        let home = dirs::home_dir().expect("could not determine home directory");
        home.join(".reqsh_state.json")
    }

    pub fn apply_session(&mut self, command: SessionCommand) {
        match command {
            SessionCommand::Base(url) => {
                self.base_url = Some(url);
            }

            SessionCommand::Timeout(timeout) => {
                self.timeout = Some(timeout);
            }

            SessionCommand::Header(command) => {
                self.apply_header(command);
            }
        }
    }

    pub fn apply_variable(&mut self, command: VariableCommand) {
        match command {
            VariableCommand::Set { name, value } => {
                self.variables.insert(name, value);
            }

            VariableCommand::Remove { name } => {
                self.variables.remove(&name);
            }

            VariableCommand::Clear => {
                self.variables.clear();
            }

            VariableCommand::List => {}
        }
    }

    fn apply_header(&mut self, command: HeaderCommand) {
        match command {
            HeaderCommand::Set { name, value } => {
                self.headers.insert(name, value);
            }

            HeaderCommand::Remove { name } => {
                self.headers.remove(&name);
            }

            HeaderCommand::Clear => {
                self.headers.clear();
            }

            HeaderCommand::List => {}
        }
    }

    pub fn build_request(&self, spec: RequestSpec) -> Result<HttpRequest, String> {
        let base = self.base_url.as_ref().ok_or("base URL is not configured")?;

        let url = Url::parse(base)
            .map_err(|e| e.to_string())?
            .join(&spec.path)
            .map_err(|e| e.to_string())?;

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
                HeaderValue::from_str(&header.value).map_err(|e| e.to_string())?,
            );
        }

        let method = to_reqwest_method(spec.method);

        let body = if spec.body.is_empty() {
            None
        } else {
            Some(spec.body.into_bytes())
        };

        Ok(HttpRequest {
            method,
            url,
            headers,
            body,
        })
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
