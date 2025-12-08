//! The single-day CLI entrypoints.

use std::time::Duration;

use ansi_term::Colour::{Cyan, Green, Purple, Red};
use clap::{CommandFactory, FromArgMatches, Parser, ValueHint};
use rayon::ThreadPoolBuilder;

use crate::{
    derived::{Chapter, Part, Series},
    runner::{DurationThresholds, InstantTimer, PrintPartResult as _},
    source::{ChapterSources, IOResult, Source, source_path_fill_tokens},
};

/// Check whether a value should be submitted.
///
/// This rejects any unlikely values (empty string, 0, and 1, as well as any values containing a
/// newline).
fn is_plausible_result(result: &str) -> bool {
    !(["", "0", "1"].contains(&result) || result.contains('\n'))
}

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
    folder: String,

    /// Whether to auto-submit answers.
    #[arg(action)]
    submit: bool,
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

    // Get the [`SingleArgs`] from the arguments.
    fn get_single_args(args: &Self::Args) -> &SingleArgs;

    /// Setup based on arguments.
    fn setup(args: &Self::Args, series: &'static Series, chapter: &'static Chapter) -> Self;

    /// Print the header line at the top for this action.
    fn print_header(&self, description: String);

    /// Run a single part.
    fn run(&mut self, part: &Part, input: &str, solution: Result<Source, String>);

    /// Run after all parts have finished.
    fn finish(&mut self) {
    }
}

struct SingleRunnerImpl {
    submit: bool,
    series: &'static Series,
    chapter: &'static Chapter,
}
impl SingleRunner for SingleRunnerImpl {
    type Args = SingleArgs;

    fn get_single_args(args: &Self::Args) -> &SingleArgs {
        args
    }

    fn setup(
        args: &Self::Args,
        series: &'static Series,
        chapter: &'static Chapter,
    ) -> SingleRunnerImpl {
        SingleRunnerImpl {
            submit: args.submit,
            series,
            chapter,
        }
    }

    fn print_header(&self, description: String) {
        println!("Running {description}...");
    }

    fn run(&mut self, part: &Part, input: &str, solution: Result<Source, String>) {
        let result = solution
            .clone()
            .and_then(|s| part.run::<InstantTimer>(input, s.read().to_option()?));
        result.print(&format!("Part {}", part.num), &THRESHOLDS, true);

        if let Ok(result) = result
            && result.solution.is_none()
            && let Ok(solution) = solution
            && is_plausible_result(&result.result)
        {
            // The result is not yet confirmed, so attempt to submit & save it.
            if self.submit && is_plausible_result(&result.result) {
                match self
                    .series
                    .controller
                    .validate_result(self.chapter, part, &result.result)
                {
                    Ok(Ok(())) => {
                        println!("{}", Green.paint("Result is correct!"));
                        match solution.write(&result.result) {
                            IOResult::Ok(_) => {
                                println!("Saved result to {}.", Cyan.paint(solution.source()));
                            }
                            IOResult::NotFound(_) => {
                                println!("Failed to save to {}.", Cyan.paint(solution.source()));
                            }
                            IOResult::Err(err) => {
                                println!(
                                    "Failed to save to {}: {}.",
                                    Cyan.paint(solution.source()),
                                    Red.paint(err)
                                );
                            }
                        }
                    }
                    Ok(Err(err)) => {
                        println!("{}", Red.paint("Result is incorrect:"));
                        println!("{err}");
                    }
                    Err(err) => {
                        println!(
                            "Result could not be validated automatically: {}.",
                            Red.paint(String::from(err))
                        );
                    }
                }
            }

            let solution = solution.transform_path(|p| format!("{p}.pending"));
            if solution.write(&result.result).to_value().is_ok() {
                println!(
                    "Saved preliminary result, validate manually and run `make confirm` if it is correct."
                );
            }
        }
    }
}

#[doc(hidden)]
pub(super) fn run_single<T: SingleRunner>(series: &'static Series, chapter: &'static Chapter) {
    // Replace the placeholders in the default values.
    let mut cmd = T::Args::command_for_update();
    cmd =
        arg_default_value_fill_tokens!(cmd, "folder", series = series.name, chapter = chapter.name);

    // Parse & replace the placeholders in the actual values.
    let mut matches = cmd.get_matches();
    let args = T::Args::from_arg_matches_mut(&mut matches).unwrap();
    let folder = ChapterSources::Path(T::get_single_args(&args).folder.clone());
    let folder = source_path_fill_tokens!(folder, series = series.name, chapter = chapter.name);
    let input_path = folder.input().to_value().unwrap();

    let mut runner = T::setup(&args, series, chapter);
    runner.print_header(format!(
        "{} {}{} using input {}",
        Purple.paint(series.title),
        Cyan.paint(chapter.name),
        chapter
            .title
            .map_or(String::new(), |t| format!(": {}", Purple.paint(t))),
        Cyan.paint(input_path.source()),
    ));

    let input = match input_path
        .read_or_init(|| series.controller.get_input(chapter))
        .to_value()
    {
        Ok(contents) => contents,
        Err(err) => return println!("{}", Red.paint(err)),
    };

    // Initialize the thread pool now. This will happen automatically when it's first needed, but if this is inside a solution this will add to the runtime of that solution, unfairly penalizing it for being the first to use rayon while the other solutions that also do so get a free pass.
    ThreadPoolBuilder::new().build_global().unwrap();

    for part in &chapter.parts {
        runner.run(part, &input, folder.part(part.num).to_value());
    }

    runner.finish();
}

#[doc(hidden)]
pub fn main(series: &'static Series, chapter: &'static Chapter) {
    run_single::<SingleRunnerImpl>(series, chapter);
}
