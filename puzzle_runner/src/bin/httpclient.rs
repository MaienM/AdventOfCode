use std::io::{stdin, stdout};

use puzzle_runner::http::{EHTTPClient, HTTPClient as _, HTTPRequest};

pub fn main() {
    let client = EHTTPClient::new().unwrap();
    let request: HTTPRequest = serde_json::from_reader(stdin()).unwrap();
    let response = client.send(request);
    serde_json::to_writer(stdout(), &response).unwrap();
}
