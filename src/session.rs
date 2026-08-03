use std::{collections::HashMap, time::Duration};

use reqwest::{
    Url,
    header::{HeaderMap, HeaderName, HeaderValue},
};

use crate::{
    ast::{HeaderCommand, RequestSpec, SessionCommand, VariableCommand},
    http::{HttpRequest, to_reqwest_method},
};

#[derive(Debug, Default)]
pub struct Session {
    base_url: Option<String>,
    headers: HashMap<String, String>,
    variables: HashMap<String, String>,
    timeout: Option<Duration>,
}

impl Session {
    pub fn new() -> Self {
        Self::default()
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
