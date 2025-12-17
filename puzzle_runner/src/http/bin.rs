#![cfg(not(target_arch = "wasm32"))]

use std::{
    env,
    os::unix::fs::PermissionsExt as _,
    path::PathBuf,
    process::{self, Stdio},
};

use crate::http::{HTTPClient, HTTPRequest};

/// An HTTP(s) client based on running a precompiled binary.
pub struct BinHTTPClient(PathBuf);
impl HTTPClient for BinHTTPClient {
    fn new() -> Result<Self, String> {
        let bin = env::current_exe()
            .map_err(|e| format!("failed to find current executable: {e}"))?
            .parent()
            .ok_or("failed to get directory containing current executable")?
            .join("httpclient");
        match bin.metadata() {
            Ok(metadata) => {
                if !(metadata.is_file() && metadata.permissions().mode() & 0o100 > 0) {
                    Err(format!("binary ({}) isn't an executable", bin.display()))?;
                }
            }
            Err(err) => Err(format!(
                "Unable to find controller binary ({}): {err}",
                bin.display()
            ))?,
        }
        Ok(Self(bin))
    }

    fn send(&self, request: HTTPRequest) -> Result<String, String> {
        let mut proc = process::Command::new(&self.0)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| format!("failed to run httpclient: {e}"))?;
        serde_json::to_writer(proc.stdin.as_mut().unwrap(), &request)
            .map_err(|e| format!("failed to serialize HTTP request: {e}"))?;
        let status = proc
            .wait()
            .map_err(|e| format!("failed to wait for httpclient: {e}"))?;
        if !status.success() {
            return Err(format!("failed to run httpclient: exit code {status}"));
        }
        serde_json::from_reader(proc.stdout.unwrap())
            .map_err(|e| format!("failed to deserialize httpclient response: {e}"))?
    }
}
