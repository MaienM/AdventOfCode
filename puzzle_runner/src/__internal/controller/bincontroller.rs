#![cfg(not(target_arch = "wasm32"))]

use std::{
    env,
    fmt::Debug,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::{self, Stdio},
};

use serde::de::DeserializeOwned;

use crate::{
    controller::{Controller, ControllerResult},
    source::ChapterSources,
};

/// A [`Controller`] which wraps the CLI exposed by this module.
pub struct BinController(PathBuf);
impl BinController {
    fn run<R>(&self, args: &[&str]) -> ControllerResult<R>
    where
        R: DeserializeOwned + Debug,
    {
        let proc = process::Command::new(&self.0)
            .arg("--machine")
            .args(args)
            .stdout(Stdio::piped())
            .spawn()?;
        serde_json::from_reader(proc.stdout.unwrap()).unwrap()
    }
}
impl Controller for BinController {
    fn new() -> ControllerResult<Self>
    where
        Self: Sized,
    {
        let bin = env::current_exe()?
            .parent()
            .ok_or("Failed to get directory containing current executable")?
            .join("controller");
        match bin.metadata() {
            Ok(metadata) => {
                if !(metadata.is_file() && metadata.permissions().mode() & 0o100 > 0) {
                    Err(format!(
                        "Controller binary ({}) isn't an executable",
                        bin.display()
                    ))?;
                }
            }
            Err(err) => Err(format!(
                "Unable to find controller binary ({}): {err}",
                bin.display()
            ))?,
        }
        Ok(Self(bin))
    }

    fn get_input(&self, chapter: &str) -> ControllerResult<String> {
        self.run(&["get-input", chapter])
    }

    fn validate_result(
        &self,
        chapter: &str,
        part: u8,
        result: &str,
        sources: &crate::source::ChapterSources,
    ) -> ControllerResult<(bool, String)> {
        let ChapterSources::Path(path) = sources else {
            Err("cannot run on non-path sources")?
        };
        self.run(&[
            "validate-result",
            chapter,
            &part.to_string(),
            result,
            "--folder",
            path,
        ])
    }

    fn validate_result_impl(
        &self,
        chapter: &str,
        part: u8,
        result: &str,
    ) -> ControllerResult<(bool, String)> {
        self.run(&["validate-result", chapter, &part.to_string(), result])
    }
}
