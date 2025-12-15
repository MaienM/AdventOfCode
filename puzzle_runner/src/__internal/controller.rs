//! The controller CLI entrypoints.

use std::{
    env,
    fmt::Debug,
    fs, io,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::{self, Stdio},
};

use ansi_term::Colour::{Cyan, Green, Red};
use clap::{Args, Parser, Subcommand, ValueHint};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    controller::{Controller, ControllerResult},
    derived::Series,
    source::{ChapterSources, source_path_fill_tokens},
};

trait Handler {
    /// The output type for this handler.
    type Output: Serialize;

    /// Perform the action.
    ///
    /// This should be as minimal as possible a wrapper around the controller function, and it
    /// should not produce any direct output (i.e., no output to stdout/stderr, no writing anything
    /// to the filesystem).
    fn execute(&self, series: &Series) -> ControllerResult<Self::Output>;

    /// Format & output the results.
    fn output(&self, series: &Series, result: Self::Output) -> ControllerResult<()>;
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct GlobalArgs {
    #[command(subcommand)]
    command: Command,

    /// Output in a machine-readable format.
    ///
    /// This is intended for internal use only and no promises are made regarding the format or
    /// compatibility with all combinations of options.
    #[arg(long)]
    machine: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Show the series' definition/metadata.
    Info(Info),
    /// Download the input for one of the chapters.
    GetInput(GetInput),
    /// Validate the result of one of the parts.
    ValidateResult(ValidateResult),
}

#[derive(Args, Debug)]
struct ChapterArgs {
    /// The name of the chapter.
    ///
    /// This must be in the same format as used for the binaries, but the binary does not
    /// actaully have to exist.
    chapter: String,
}

#[derive(Parser, Debug)]
struct Info;
impl Handler for Info {
    type Output = ();

    fn execute(&self, _series: &Series) -> ControllerResult<Self::Output> {
        Ok(())
    }

    fn output(&self, series: &Series, _result: Self::Output) -> ControllerResult<()> {
        println!("Name: {}", series.name);
        println!("Title: {}", series.title);
        Ok(())
    }
}

#[derive(Parser, Debug)]
struct GetInput {
    #[command(flatten)]
    chapter: ChapterArgs,

    /// Write the result to a file.
    ///
    /// If an argument is provided that will be used as a path, otherwise the default location
    /// (inputs/{series}/{chapter}/input.txt) will be used.
    #[arg(
        short, long,
        value_hint = ValueHint::FilePath,
    )]
    #[allow(clippy::option_option)]
    write: Option<Option<String>>,
}
impl Handler for GetInput {
    type Output = String;

    fn execute(&self, series: &Series) -> ControllerResult<Self::Output> {
        series.controller.get_input(&self.chapter.chapter)
    }

    fn output(&self, series: &Series, result: Self::Output) -> ControllerResult<()> {
        if let Some(path) = &self.write {
            let path = path.clone().unwrap_or(format!(
                "inputs/{}/{}/input.txt",
                series.name, self.chapter.chapter
            ));
            fs::write(&path, result)?;
            println!("Saved input to {}.", Cyan.paint(path));
        } else {
            print!("{result}");
        }
        Ok(())
    }
}

#[derive(Parser, Debug)]
struct ValidateResult {
    #[command(flatten)]
    chapter: ChapterArgs,

    /// The part to validate.
    part: u8,

    /// The result to validate.
    result: String,

    /// Path to a folder containing the expected outputs (`partN.txt`) & associated files.
    ///
    /// The following tokens will be replaced:
    /// - `{series}`: the name of the crate for the series (e.g., `aoc`).
    /// - `{chapter}`: the name of the chapter (e.g., `21-01`).
    #[arg(
        value_hint = ValueHint::DirPath,
        default_value = "inputs/{series}/{chapter}",
    )]
    pub(super) folder: String,
}
impl Handler for ValidateResult {
    type Output = (bool, String);

    fn execute(&self, series: &Series) -> ControllerResult<Self::Output> {
        let folder = ChapterSources::Path(self.folder.clone());
        let folder =
            source_path_fill_tokens!(folder, series = series.name, chapter = self.chapter.chapter);
        series
            .controller
            .validate_result(&self.chapter.chapter, self.part, &self.result, &folder)
    }

    fn output(&self, _series: &Series, (valid, message): Self::Output) -> ControllerResult<()> {
        if valid {
            println!("Result is valid:\n{}", Green.paint(message));
        } else {
            println!("Result is not valid:\n{}", Red.paint(message));
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn main(series: &Series) {
    let gargs = GlobalArgs::parse();
    match gargs.command {
        Command::Info(ref args) => run(series, &gargs, args),
        Command::GetInput(ref args) => run(series, &gargs, args),
        Command::ValidateResult(ref args) => run(series, &gargs, args),
    }
}

fn run<H: Handler>(series: &Series, args: &GlobalArgs, handler: &H) {
    let result = handler.execute(series);
    if args.machine {
        serde_json::to_writer(io::stdout(), &result).unwrap();
    } else {
        handler.output(series, result.unwrap()).unwrap();
    }
}

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
