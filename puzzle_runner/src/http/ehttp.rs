use ehttp::{Request, fetch_async};
use pollster::FutureExt as _;

use crate::http::{HTTPClient, HTTPRequest};

/// An HTTP(s) client based on [`ehttp`].
pub struct EHTTPClient;
impl HTTPClient for EHTTPClient {
    fn new() -> Result<Self, String> {
        Ok(Self)
    }

    fn send(&self, request: HTTPRequest) -> Result<String, String> {
        let mut erequest = Request {
            method: request.method,
            url: request.url,
            body: request.body,
            ..Request::get("")
        };
        for (k, v) in request.headers {
            erequest.headers.insert(k, v);
        }

        let response = fetch_async(erequest).block_on()?;
        let text = response.text().ok_or("empty response")?;
        Ok(text.to_owned())
    }
}
