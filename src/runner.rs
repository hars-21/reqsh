use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::request::{Method, Request};
use reqwest::{
    blocking::{Client, RequestBuilder, Response},
    header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue},
};

pub fn fetch(
    request: &Request,
    base_url: Option<&str>,
    global_headers: &HashMap<String, String>,
) -> Result<(Response, Duration), String> {
    // Client
    let client = Client::new();

    // Url Constructor
    let full_url = if request.path.starts_with("http://") || request.path.starts_with("https://") {
        request.path.clone()
    } else if request.path.starts_with("/")
        && let Some(base_url) = base_url
    {
        let base = base_url.trim_end_matches('/');
        format!("{base}{}", request.path)
    } else {
        return Err(String::from(
            "Base URL not found. Use base <url> to add base url",
        ));
    };

    // Request Builder
    let mut req_builder: RequestBuilder;

    // Method
    req_builder = match request.method {
        Method::GET => client.get(full_url),
        Method::POST => client.post(full_url),
        Method::PUT => client.put(full_url),
        Method::PATCH => client.patch(full_url),
        Method::DELETE => client.delete(full_url),
    };

    //Global Headers
    let mut headers = HeaderMap::new();
    for (key, value) in global_headers {
        let name = HeaderName::from_bytes(key.to_ascii_lowercase().as_bytes())
            .map_err(|_| format!("Invalid header name: {key}"))?;
        let val = HeaderValue::from_bytes(value.as_bytes())
            .map_err(|_| format!("Invalid header value for '{key}': {value}"))?;
        headers.insert(name, val);
    }

    // Request Headers
    for (key, value) in &request.headers {
        let name = HeaderName::from_bytes(key.to_ascii_lowercase().as_bytes())
            .map_err(|_| format!("Invalid header name: {key}"))?;
        let val = HeaderValue::from_bytes(value.as_bytes())
            .map_err(|_| format!("Invalid header value for '{key}': {value}"))?;
        headers.insert(name, val);
    }
    if !headers.contains_key(CONTENT_TYPE) {
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    }
    req_builder = req_builder.headers(headers);

    // Query Params
    if !request.params.is_empty() {
        req_builder = req_builder.query(&request.params);
    }

    // Body
    if let Some(body) = &request.body {
        req_builder = req_builder.body(body.clone());
    }

    // Timer
    let now = Instant::now();

    // Response
    let result = req_builder.send();
    let response_time = now.elapsed();

    match result {
        Ok(response) => Ok((response, response_time)),
        Err(e) => Err(format!("{}", e)),
    }
}
