use crate::request::{Method, Request};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};

pub fn fetch(request: &Request, base_url: Option<&str>) -> Result<String, String> {
    let client = Client::new();
    let full_url = if (request.path.starts_with("/"))
        && let Some(base_url) = base_url
    {
        format!("{base_url}{}", request.path)
    } else {
        request.path.clone()
    };

    let mut headers = HeaderMap::new();
    for (key, value) in &request.headers {
        headers.insert(
            HeaderName::from_bytes(key.as_bytes()).unwrap(),
            HeaderValue::from_bytes(value.as_bytes()).unwrap(),
        );
    }

    match request.method {
        Method::GET => {
            let res = client.get(full_url).headers(headers).send();

            match res {
                Ok(response) => Ok(response.text().unwrap()),
                Err(e) => return Err(format!("{}", e)),
            }
        }

        Method::POST => {
            let mut req_builder = client.post(full_url).headers(headers);

            if let Some(body) = &request.body {
                req_builder = req_builder.body(body.clone());
            }

            let res = req_builder.send();

            match res {
                Ok(response) => Ok(response.text().unwrap()),
                Err(e) => return Err(format!("{}", e)),
            }
        }

        Method::PUT => {
            let mut req_builder = client.put(full_url).headers(headers);

            if let Some(body) = &request.body {
                req_builder = req_builder.body(body.clone())
            }

            let res = req_builder.send();

            match res {
                Ok(response) => Ok(response.text().unwrap()),
                Err(e) => return Err(format!("{}", e)),
            }
        }

        Method::DELETE => {
            let res = client.delete(full_url).headers(headers).send();

            match res {
                Ok(response) => Ok(response.text().unwrap()),
                Err(e) => return Err(format!("{}", e)),
            }
        }
    }
}
