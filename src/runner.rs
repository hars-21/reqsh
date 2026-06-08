use std::collections::HashMap;

use crate::request::{Method, Request};
use reqwest::{
    blocking::{Client, RequestBuilder, Response},
    header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue},
};

pub fn fetch(
    request: &Request,
    base_url: Option<&str>,
    global_headers: &HashMap<String, String>,
) -> Result<Response, String> {
    // Client
    let client = Client::new();

    // Url Constructor
    let full_url = if request.path.starts_with("http://") || request.path.starts_with("https://")
    {
        request.path.clone()
    } else if request.path.starts_with("/") && let Some(base_url) = base_url {
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
        Method::DELETE => client.delete(full_url),
    };

    //Global Headers
    let mut headers = HeaderMap::new();
    for (key, value) in global_headers {
        headers.insert(
            HeaderName::from_bytes(key.to_ascii_lowercase().as_bytes()).unwrap(),
            HeaderValue::from_bytes(value.as_bytes()).unwrap(),
        );
    }

    // Request Headers
    for (key, value) in &request.headers {
        headers.insert(
            HeaderName::from_bytes(key.to_ascii_lowercase().as_bytes()).unwrap(),
            HeaderValue::from_bytes(value.as_bytes()).unwrap(),
        );
    }
    if !headers.contains_key(CONTENT_TYPE) {
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
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

    // Response
    let result = req_builder.send();

    match result {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("{}", e)),
    }
}
