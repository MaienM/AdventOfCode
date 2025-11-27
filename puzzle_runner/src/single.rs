//! The single-day CLI entrypoints.

use std::time::Duration;

use ansi_term::Colour::{Cyan, Purple, Red};
use clap::{CommandFactory, FromArgMatches, Parser, ValueHint};
use rayon::ThreadPoolBuilder;

use crate::{
    derived::Bin,
    runner::{DurationThresholds, InstantTimer, SolverResult},
    source::{Source, SourceValueParser, source_path_fill_tokens},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct SingleArgs {
    /// Path to a file containing the input.
    #[arg(
        value_hint = ValueHint::FilePath,
        default_value = "inputs/{name}.txt",
        value_parser = SourceValueParser,
    )]
    input: Source,

    /// Path to a file containing the expected result of part 1.
    #[arg(
        value_hint = ValueHint::FilePath,
        default_value = "inputs/{name}-{part}.txt",
        value_parser = SourceValueParser,
    )]
    part1: Source,

    /// Path to a file containing the expected result of part 2.
    #[arg(
        value_hint = ValueHint::FilePath,
        default_value = "inputs/{name}-{part}.txt",
        value_parser = SourceValueParser,
    )]
    part2: Source,
}

const THRESHOLDS: DurationThresholds = DurationThresholds {
    good: Duration::from_millis(1),
    acceptable: Duration::from_secs(1),
};

macro_rules! arg_default_value_fill_tokens {
    (replace; $chain:expr, $(,)?) => {
        $chain
    };
    (replace; $chain:expr, $name:ident = $value:expr $(, $($restname:ident = $restvalue:expr),*)?) => {
        arg_default_value_fill_tokens!(replace; $chain.replace(&format!("{{{}}}", stringify!($name)), &$value.to_string()), $($($restname = $restvalue),*)?)
    };
    ($cmd:ident, $arg:expr, $($name:ident = $value:expr),+ $(,)?) => {
        $cmd.mut_arg($arg, |a| {
            let value = a.get_default_values()[0].to_str().unwrap();
            let value = arg_default_value_fill_tokens!(replace; value, $($name = $value),+);
            a.default_value(value)
        })
    }
}

#[doc(hidden)]
pub fn main(bin: &Bin) {
    // Replace the placeholders in the default values.
    let mut cmd = SingleArgs::command_for_update();
    cmd = arg_default_value_fill_tokens!(cmd, "input", name = bin.name);
    cmd = arg_default_value_fill_tokens!(cmd, "part1", name = bin.name, part = 1);
    cmd = arg_default_value_fill_tokens!(cmd, "part2", name = bin.name, part = 2);

    let mut matches = cmd.get_matches();
    let args = SingleArgs::from_arg_matches_mut(&mut matches).unwrap();

    let input_path = source_path_fill_tokens!(args.input, bin = bin);
    let part1_path = source_path_fill_tokens!(args.part1, bin = bin, part = 1);
    let part2_path = source_path_fill_tokens!(args.part2, bin = bin, part = 2);

    println!(
        "Running {}{} using input {}...",
        Cyan.paint(bin.name),
        bin.title
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

    for (i, part, solution_path) in [
        (1, &bin.part1, part1_path),
        (2, &bin.part2, part2_path),
    ] {
        let solution = solution_path.read_maybe();
        let result = match solution {
            Ok(solution) => {
                part.run::<InstantTimer>(&input, solution)
            }
            Err(err) => SolverResult::Error(err),
        };
        result.print(&format!("Part {i}"), &THRESHOLDS, true);
    }
}
