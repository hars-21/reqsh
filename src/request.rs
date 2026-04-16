use std::process::Command;

pub struct Request {
    pub method: RequestMethod,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
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

impl Request {
    pub fn new(method: RequestMethod, url: String, body: Option<String>) -> Self {
        Self {
            method,
            url,
            headers: Vec::new(),
            body,
        }
    }

    pub fn fetch(&self, base_url: Option<&str>) -> String {
        let full_url = match base_url {
            Some(base) => format!("{}{}", base, self.url),
            None => format!("{}", self.url),
        };

        let body = match &self.body {
            Some(data) => format!("{}", data),
            None => format!(""),
        };

        let response = Command::new("curl")
            .args(["-X", &self.method.as_str(), &full_url, "-d", &body])
            .output()
            .map_err(|err| err.to_string())
            .unwrap();

        if response.status.success() {
            String::from_utf8(response.stdout).unwrap().to_string()
        } else {
            String::from_utf8(response.stderr).unwrap().to_string()
        }
    }
}
