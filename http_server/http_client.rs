use std::io::{ Write, Read };
use std::net::TcpStream;
use crate::http_server::http_request::HttpRequest;

pub struct HttpClient {
    pub host: String,
    pub port: u16,
}

impl HttpClient {
    pub fn new(host: String, port: u16) -> Self {
        HttpClient { host, port }
    }

    pub fn send_request(&self, request: &HttpRequest) -> Result<String, std::io::Error> {
        // Establish a TCP connection
        let address = format!("{}:{}", self.host, self.port);
        let mut stream = TcpStream::connect(address)?;

        //Write the HTTP request to the stream
        let request_bytes = request.build_request();
        stream.write_all(&request_bytes)?;

        // Read the response into a buffer
        let mut response = String::new();
        stream.read_to_string(&mut response)?;

        Ok(response)
    }
}
