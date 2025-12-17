//! A wrapper around an HTTP(s) client that supports deferring to a precompiled executable to
//! avoid having to bundle these dependencies in every binary.

mod bin;
mod ehttp;

use std::{collections::HashMap, fmt::Debug};

use cfg_if::cfg_if;
pub use ehttp::EHTTPClient;
use serde::{Deserialize, Serialize};

/// An HTTP(s) client.
pub trait HTTPClient: Send + Sync {
    /// Create a new instance of the controller.
    ///
    /// # Errors
    ///
    /// Returns an error if the client cannot function in the current environment.
    fn new() -> Result<Self, String>
    where
        Self: Sized;

    /// Perform a request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request could not be completed successfully, or the request
    /// returned a non-2xx status code.
    fn send(&self, request: HTTPRequest) -> Result<String, String>;
}

/// An HTTP(s) request.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HTTPRequest {
    /// The request method.
    pub method: String,
    /// The request URL.
    pub url: String,
    /// The request body, if any.
    pub body: Vec<u8>,
    /// The request headers.
    pub headers: HashMap<String, String>,
}
impl HTTPRequest {
    /// Create a new [`HTTPRequest`] with the given method & url.
    pub fn new(method: &str, url: &str) -> Self {
        Self {
            method: method.to_owned(),
            url: url.to_owned(),
            ..Self::default()
        }
    }

    /// Set the body to a string.
    pub fn set_body_str(&mut self, data: &str) {
        self.body = data.to_owned().into_bytes();
    }

    /// Set the body to form data.
    ///
    /// This adds the following headers (unless they are already set):
    ///
    /// ```
    /// Content-Type: application/x-www-form-urlencoded
    /// ```
    pub fn set_body_form<K: ToString, V: ToString, I: IntoIterator<Item = (K, V)>>(
        &mut self,
        data: I,
    ) {
        let body: Vec<_> = data
            .into_iter()
            .map(|(k, v)| format!("{}={}", k.to_string(), v.to_string()))
            .collect();
        self.body = body.join("&").into_bytes();

        self.set_default_header("Content-Type", "application/x-www-form-urlencoded");
    }

    /// Set the body to a JSON object.
    ///
    /// This adds the following headers (unless they are already set):
    ///
    /// ```
    /// Accept: application/json
    /// Content-Type: application/json
    /// ```
    pub fn set_body_json<T: Serialize>(&mut self, data: T) -> Result<(), String> {
        self.body = serde_json::to_vec(&data).map_err(|e| e.to_string())?;

        self.set_default_header("Accept", "application/json");
        self.set_default_header("Content-Type", "application/json");

        Ok(())
    }

    /// Set a header, but only if it's not yet set.
    fn set_default_header(&mut self, key: &str, value: &str) {
        if !self.headers.contains_key(key) {
            self.headers.insert(key.to_owned(), value.to_owned());
        }
    }

    /// Perform a request using the default HTTP client for the current environment.
    ///
    /// # Errors
    ///
    /// Returns an error if the client could not be created, the request could not be completed
    /// successfully, or the request returned a non-2xx status code.
    pub fn send(self) -> Result<String, String> {
        cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let client = ehttp::EHTTPClient::new()?;
            } else {
                let client = bin::BinHTTPClient::new()?;
            }
        }
        client.send(self)
    }
}
