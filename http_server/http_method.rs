#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    GET,
    POST,
}

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
        }
    }
}
