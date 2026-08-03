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

    pub fn send(&self, request: HttpRequest) -> Result<Response, Error> {
        let mut builder = self
            .client
            .request(request.method, request.url)
            .headers(request.headers);

        if let Some(body) = request.body {
            builder = builder.body(body);
        }

        builder.send()
    }
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
