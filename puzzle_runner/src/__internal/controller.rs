//! The controller CLI entrypoints.

use std::fs;

use ansi_term::Colour::Cyan;
use clap::{Parser, Subcommand, ValueHint};

use crate::{controller::ControllerResult, derived::Series};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Show the series' definition/metadata.
    Info,
    /// Download the input for one of the chapters.
    GetInput(GetInput),
}

#[derive(Parser, Debug)]
struct GetInput {
    /// The name of the chapter.
    ///
    /// This must be in the same format as used for the binaries, but the binary does not
    /// actaully have to exist.
    chapter: String,

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

#[doc(hidden)]
pub fn main(series: &Series) {
    let args = Args::parse();
    match args.command {
        Command::Info => info(series),
        Command::GetInput(args) => get_input(series, args).unwrap(),
    }
}

fn info(series: &Series) {
    println!("Name: {}", series.name);
    println!("Title: {}", series.title);
}

fn get_input(series: &Series, args: GetInput) -> ControllerResult<()> {
    let input = series.controller.get_input(&args.chapter)?;
    if let Some(path) = args.write {
        let path = path.unwrap_or(format!("inputs/{}/{}/input.txt", series.name, args.chapter));
        fs::write(&path, input)?;
        println!("Saved input in {}.", Cyan.paint(path));
    } else {
        print!("{input}");
    }
    Ok(())
}
