//! The single-day CLI entrypoints.

use std::time::Duration;

use ansi_term::Colour::{Cyan, Purple, Red};
use clap::{CommandFactory, FromArgMatches, Parser, ValueHint};
use rayon::ThreadPoolBuilder;

use crate::{
    derived::{Chapter, Part},
    runner::{DurationThresholds, InstantTimer, PrintPartResult as _},
    source::{ChapterSources, ChapterSourcesValueParser, Source, source_path_fill_tokens},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(super) struct SingleArgs {
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
    pub(super) folder: ChapterSources,
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

/// A trait for a way to run the parts for a single run.
pub(super) trait SingleRunner {
    type Args: Parser;

    // Get the sources from the arguments.
    fn get_sources_arg(args: &mut Self::Args) -> &mut ChapterSources;

    /// Setup based on arguments.
    fn setup(args: &Self::Args, series: String, chapter: &Chapter) -> Self;

    /// Print the header line at the top for this action.
    fn print_header(&self, description: String);

    /// Run a single part.
    fn run(&mut self, part: &Part, input: &str, solution: Result<Source, String>);

    /// Run after all parts have finished.
    fn finish(&mut self) {
    }
}

struct SingleRunnerImpl;
impl SingleRunner for SingleRunnerImpl {
    type Args = SingleArgs;

    fn get_sources_arg(args: &mut Self::Args) -> &mut ChapterSources {
        &mut args.folder
    }

    fn setup(_args: &Self::Args, _series: String, _chapter: &Chapter) -> Self {
        SingleRunnerImpl
    }

    fn print_header(&self, description: String) {
        println!("Running {description}...");
    }

    fn run(&mut self, part: &Part, input: &str, solution: Result<Source, String>) {
        let result = solution
            .clone()
            .and_then(|s| part.run::<InstantTimer>(input, s.read_maybe()?));
        result.print(&format!("Part {}", part.num), &THRESHOLDS, true);

        if let Ok(result) = result
            && result.solution.is_none()
            && let Ok(solution) = solution
        {
            let solution = solution.mutate_path(|p| format!("{p}.pending"));
            if let Ok(true) = solution.write_maybe(&result.result) {
                println!("Saved preliminary result, run `make confirm` if it is correct.");
            }
        }
    }
}

#[doc(hidden)]
pub(super) fn run_single<T: SingleRunner>(chapter: &Chapter) {
    let series = std::env::var("CARGO_PKG_NAME").unwrap();

    // Replace the placeholders in the default values.
    let mut cmd = T::Args::command_for_update();
    cmd = arg_default_value_fill_tokens!(cmd, "folder", series = series, chapter = chapter.name);

    // Parse & replace the placeholders in the actual values.
    let mut matches = cmd.get_matches();
    let mut args = T::Args::from_arg_matches_mut(&mut matches).unwrap();
    let folder = T::get_sources_arg(&mut args);
    *folder = source_path_fill_tokens!(folder, series = series, chapter = chapter.name);
    let folder = folder.clone();

    let mut runner = T::setup(&args, series, chapter);
    let input_path = folder.input().unwrap();

    runner.print_header(format!(
        "{}{} using input {}",
        Cyan.paint(chapter.name),
        chapter
            .title
            .map_or(String::new(), |t| format!(": {}", Purple.paint(t))),
        Cyan.paint(input_path.source().unwrap()),
    ));

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
        runner.run(part, &input, folder.part(part.num));
    }

    runner.finish();
}

#[doc(hidden)]
pub fn main(chapter: &Chapter) {
    run_single::<SingleRunnerImpl>(chapter);
}
