use std::error::Error as StdError;
use std::fmt;
use std::io::{Error as IoError, ErrorKind};
use std::time::{Duration, Instant};

use reqwest::{
    Error, Method, Url,
    blocking::{Client as ReqwestClient, Response},
    header::HeaderMap,
};

use crate::ast::Method as AstMethod;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub url: Url,
    pub headers: HeaderMap,
    pub body: Option<Vec<u8>>,
    pub timeout: Option<Duration>,
}

pub struct Client {
    client: ReqwestClient,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: ReqwestClient::new(),
        }
    }

    pub fn send(&self, request: HttpRequest) -> Result<HttpResponse, Error> {
        let mut builder = self
            .client
            .request(request.method, request.url)
            .headers(request.headers);

        if let Some(timeout) = request.timeout {
            builder = builder.timeout(timeout);
        }

        if let Some(body) = request.body {
            builder = builder.body(body);
        }

        let start = Instant::now();

        let response = builder.send()?;

        HttpResponse::from_reqwest(response, start.elapsed())
    }
}

pub fn describe_request_error(error: &Error) -> String {
    if error.is_timeout() {
        "request timed out".to_string()
    } else if error.is_connect() {
        describe_connect_error(error)
    } else {
        error.to_string()
    }
}

fn describe_connect_error(error: &Error) -> String {
    let mut source = StdError::source(error);
    while let Some(cause) = source {
        if let Some(io) = cause.downcast_ref::<IoError>() {
            if io.kind() == ErrorKind::ConnectionRefused {
                return "connection refused".to_string();
            }

            let message = io.to_string().to_lowercase();
            if message.contains("failed to lookup") || message.contains("name or service not known")
            {
                return "could not resolve host".to_string();
            }
        }
        source = StdError::source(cause);
    }

    error.to_string()
}

pub fn to_reqwest_method(method: AstMethod) -> Method {
    match method {
        AstMethod::Get => Method::GET,
        AstMethod::Post => Method::POST,
        AstMethod::Put => Method::PUT,
        AstMethod::Patch => Method::PATCH,
        AstMethod::Delete => Method::DELETE,
        AstMethod::Head => Method::HEAD,
        AstMethod::Options => Method::OPTIONS,
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub version: String,
    pub status: u16,
    pub reason: String,
    pub headers: Vec<Header>,
    pub body: String,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl HttpResponse {
    pub fn from_reqwest(response: Response, duration: Duration) -> Result<Self, Error> {
        let status = response.status();

        let headers = response
            .headers()
            .iter()
            .map(|(name, value)| Header {
                name: name.to_string(),
                value: value.to_str().unwrap_or_default().to_string(),
            })
            .collect();

        Ok(Self {
            version: format!("{:?}", response.version()),
            status: status.as_u16(),
            reason: status.canonical_reason().unwrap_or_default().to_string(),
            headers,
            body: response.text()?,
            duration,
        })
    }
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {} {}", self.version, self.status, self.reason)?;

        for header in &self.headers {
            writeln!(f, "{}: {}", header.name, header.value)?;
        }

        writeln!(f)?;

        write!(f, "{}", self.body)?;

        Ok(())
    }
}
