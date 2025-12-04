//! The single-day CLI entrypoints.

use std::time::Duration;

use ansi_term::Colour::{Cyan, Purple, Red};
use clap::{CommandFactory, FromArgMatches, Parser, ValueHint};
use rayon::ThreadPoolBuilder;

use crate::{
    derived::Chapter,
    runner::{DurationThresholds, InstantTimer, PrintPartResult as _},
    source::{ChapterSources, ChapterSourcesValueParser, source_path_fill_tokens},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct SingleArgs {
    /// Path to a folder containing the input (`input.txt`) and expected outputs (`partN.txt`).
    ///
    /// The following tokens will be replaced:
    /// - `{series}`: the name of the crate for the series (e.g., `aoc`).
    /// - `{chapter}`: the name of the chapter (e.g., `21-01`).
    #[arg(
        value_hint = ValueHint::DirPath,
        default_value = "inputs/{series}/{chapter}",
        value_parser = ChapterSourcesValueParser,
    )]
    folder: ChapterSources,
}

const THRESHOLDS: DurationThresholds = DurationThresholds {
    good: Duration::from_millis(1),
    acceptable: Duration::from_secs(1),
};

macro_rules! arg_default_value_fill_tokens {
    ($cmd:ident, $arg:expr, $($name:ident = $value:expr),+ $(,)?) => {
        $cmd.mut_arg($arg, |a| {
            let value = a.get_default_values()[0].to_str().unwrap();
            let value = arg_default_value_fill_tokens!(@replace; value, $($name = $value),+);
            a.default_value(value)
        })
    };
    (@replace; $chain:expr, $name:ident = $value:expr $(, $($restname:ident = $restvalue:expr),*)?) => {
        arg_default_value_fill_tokens!(@replace; $chain.replace(&format!("{{{}}}", stringify!($name)), &$value.to_string()), $($($restname = $restvalue),*)?)
    };
    (@replace; $chain:expr, $(,)?) => {
        $chain
    };
}

#[doc(hidden)]
pub fn main(chapter: &Chapter) {
    let series = std::env::var("CARGO_PKG_NAME").unwrap();

    // Replace the placeholders in the default values.
    let mut cmd = SingleArgs::command_for_update();
    cmd = arg_default_value_fill_tokens!(cmd, "folder", series = series, name = chapter.name);

    // Parse & replace the placeholders in the actual values.
    let mut matches = cmd.get_matches();
    let args = SingleArgs::from_arg_matches_mut(&mut matches).unwrap();

    let folder_path =
        source_path_fill_tokens!(args.folder, series = series, chapter = chapter.name);
    let input_path = folder_path.input().unwrap();

    println!(
        "Running {}{} using input {}...",
        Cyan.paint(chapter.name),
        chapter
            .title
            .map_or(String::new(), |t| format!(": {}", Purple.paint(t))),
        Cyan.paint(input_path.source().unwrap()),
    );

    let input = match input_path.read() {
        Ok(input) => input,
        Err(err) => {
            println!("{}", Red.paint(err));
            return;
        }
    };

    // Initialize the thread pool now. This will happen automatically when it's first needed, but if this is inside a solution this will add to the runtime of that solution, unfairly penalizing it for being the first to use rayon while the other solutions that also do so get a free pass.
    ThreadPoolBuilder::new().build_global().unwrap();

    for part in &chapter.parts {
        let solution = folder_path.part(part.num).and_then(|s| s.read_maybe());
        let result = solution.and_then(|solution| part.run::<InstantTimer>(&input, solution));
        result.print(&format!("Part {}", part.num), &THRESHOLDS, true);
    }
}
