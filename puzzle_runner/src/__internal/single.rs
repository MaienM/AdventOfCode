//! The single-day CLI entrypoints.

use std::time::Duration;

use ansi_term::Colour::{Cyan, Purple, Red};
use clap::{CommandFactory, FromArgMatches, Parser, ValueHint};
use rayon::ThreadPoolBuilder;

use crate::{
    derived::{Chapter, Part, Series},
    runner::{DurationThresholds, InstantTimer, PrintPartResult as _},
    source::{ChapterSources, IOResult, PartFileType, source_path_fill_tokens},
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
    )]
    pub(super) folder: String,
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
    fn get_sources_arg(args: &Self::Args) -> &String;

    /// Setup based on arguments.
    fn setup(args: &Self::Args, series: &Series, chapter: &Chapter) -> Self;

    /// Print the header line at the top for this action.
    fn print_header(&self, description: String);

    /// Run a single part.
    fn run(&mut self, part: &Part, input: &str, sources: &ChapterSources);

    /// Run after all parts have finished.
    fn finish(&mut self) {
    }
}

struct SingleRunnerImpl;
impl SingleRunner for SingleRunnerImpl {
    type Args = SingleArgs;

    fn get_sources_arg(args: &Self::Args) -> &String {
        &args.folder
    }

    fn setup(_args: &Self::Args, _series: &Series, _chapter: &Chapter) -> Self {
        SingleRunnerImpl
    }

    fn print_header(&self, description: String) {
        println!("Running {description}...");
    }

    fn run(&mut self, part: &Part, input: &str, sources: &ChapterSources) {
        let result = sources
            .part(part.num, &PartFileType::Result)
            .to_option()
            .and_then(|s| {
                part.run::<InstantTimer>(input, s.and_then(|s| s.read().to_option().unwrap()))
            });
        result.print(&format!("Part {}", part.num), &THRESHOLDS, true);

        if let Ok(result) = result
            && result.solution.is_none()
            && let IOResult::Ok(pending) = sources.part(part.num, &PartFileType::Pending)
            && pending.write(&result.result).to_value().is_ok()
        {
            println!(
                "Saved preliminary result, run `make submit` to validate it or `make confirm` to mark it as correct."
            );
        }
    }
}

#[doc(hidden)]
pub(super) fn run_single<T: SingleRunner>(series: &Series, chapter: &Chapter) {
    // Replace the placeholders in the default values.
    let mut cmd = T::Args::command_for_update();
    cmd =
        arg_default_value_fill_tokens!(cmd, "folder", series = series.name, chapter = chapter.name);

    // Parse & replace the placeholders in the actual values.
    let mut matches = cmd.get_matches();
    let args = T::Args::from_arg_matches_mut(&mut matches).unwrap();
    let folder = ChapterSources::Path(T::get_sources_arg(&args).clone());
    let folder = source_path_fill_tokens!(folder, series = series.name, chapter = chapter.name);
    let input_path = folder.input().to_value().unwrap();

    let mut runner = T::setup(&args, series, chapter);
    runner.print_header(format!(
        "{} {}{} using input {}",
        Purple.paint(&series.title),
        Cyan.paint(chapter.name),
        chapter
            .title
            .as_ref()
            .map_or(String::new(), |t| format!(": {}", Purple.paint(t))),
        Cyan.paint(input_path.source()),
    ));

    let input = match input_path
        .read_or_init(|| {
            println!("Downloading input...");
            series.controller.get_input(chapter.name)
        })
        .to_value()
    {
        Ok(contents) => contents,
        Err(err) => return println!("{}", Red.paint(err)),
    };

    // Initialize the thread pool now. This will happen automatically when it's first needed, but if this is inside a solution this will add to the runtime of that solution, unfairly penalizing it for being the first to use rayon while the other solutions that also do so get a free pass.
    ThreadPoolBuilder::new().build_global().unwrap();

    for part in &chapter.parts {
        runner.run(part, &input, &folder);
    }

    runner.finish();
}

#[doc(hidden)]
pub fn main(series: &Series, chapter: &Chapter) {
    run_single::<SingleRunnerImpl>(series, chapter);
}
