mod http_server;

use std::collections::HashMap;
use http_server::http_method::HttpMethod;
use http_server::http_request::HttpRequest;
use http_server::http_client::HttpClient;

fn main() {
    // Define headers
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Host".to_string(), "example.com".to_string());
    headers.insert("User-Agent".to_string(), "RustHttpClient".to_string());

    // Create an HttpRequest instance
    let request = HttpRequest::new(HttpMethod::GET, "/", headers, None);

    // Create an HttpClient instance
    let client = HttpClient::new("example.com".to_string(), 80);

    // Send the HTTP request and handle the response
    match client.send_request(&request) {
        Ok(response) => println!("{}", response),
        Err(err) => eprintln!("Error: {}", err),
    }
}
