use std::collections::HashMap;
use crate::http_server::http_method::HttpMethod;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn new(
        method: HttpMethod,
        url: &str,
        headers: HashMap<String, String>,
        body: Option<String>
    ) -> Self {
        HttpRequest {
            method,
            url: url.to_string(),
            headers,
            body,
        }
    }

    pub fn build_request(&self) -> Vec<u8> {
        let mut request = format!("{} {} HTTP/1.1\r\n", self.method.as_str(), self.url);

        for (key, value) in &self.headers {
            request.push_str(&format!("{}: {}\r\n", key, value));
        }

        request.push_str("\r\n");

        if let Some(ref body) = self.body {
            request.push_str(body);
        }

        request.into_bytes()
    }
}
