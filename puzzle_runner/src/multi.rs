//! Helpers for the multi-day CLI entrypoint (`aoc`).

use std::{collections::HashSet, sync::OnceLock, time::Duration};

use ansi_term::Colour::{Cyan, Purple};
use clap::{
    Parser,
    builder::{PossibleValue, PossibleValuesParser, TypedValueParser},
};
use rayon::ThreadPoolBuilder;

use super::source::source_path_fill_tokens;
use crate::{
    derived::{Bin, Solver},
    runner::{DurationThresholds, InstantTimer, SolverResult},
    source::{Source, SourceValueParser},
};

pub static BINS: OnceLock<Vec<Bin>> = OnceLock::new();

/// Create parser for --only/--skip.
fn create_target_value_parser() -> impl TypedValueParser {
    fn create_value(name: &str, suffix: &str) -> PossibleValue {
        PossibleValue::new(format!("{name}{suffix}"))
    }

    let mut options = Vec::new();
    for bin in BINS.get().unwrap() {
        options.push(PossibleValue::new(bin.year.to_string()));
        options.push(create_value(bin.name, ""));
        if bin.part1.is_implemented() {
            options.push(create_value(bin.name, "-1"));
        }
        if bin.part2.is_implemented() {
            options.push(create_value(bin.name, "-2"));
        }
    }
    let parser = PossibleValuesParser::new(options);

    parser.map(|item| {
        let parts: Vec<u8> = item
            .split('-')
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        match parts.len() {
            3 => vec![(parts[0], parts[1], parts[2])],
            2 => vec![(parts[0], parts[1], 1), (parts[0], parts[1], 2)],
            1 => (1..=25)
                .flat_map(|d| vec![(parts[0], d, 1), (parts[0], d, 2)])
                .collect(),
            _ => panic!("Invalid filter item {item:?}."),
        }
    })
}

#[derive(Parser, Debug)]
pub(super) struct TargetArgs {
    /// Only run the listed binaries.
    ///
    /// This can be either {year} for all binaries for a year, {year}-{day} for a single day, or {year}-{day}-{part} for a single part of a single day.
    #[arg(
        long,
        value_delimiter = ',',
        value_name = "21,22-01,22-08-1",
        value_parser = create_target_value_parser(),
        group = "targets",
    )]
    only: Option<Vec<Vec<(u8, u8, u8)>>>,

    /// Skip the listed binaries.
    ///
    /// This can be either {year} for all binaries for a year, {year}-{day} for a single day, or {year}-{day}-{part} for a single part of a single day.
    #[arg(
        long,
        value_delimiter = ',',
        value_name = "21,22-01,22-08-1",
        value_parser = create_target_value_parser(),
        group = "targets",
    )]
    skip: Option<Vec<Vec<(u8, u8, u8)>>>,

    /// Pattern for paths to files containing the inputs.
    ///
    /// The following tokens will be replaced:
    /// - `{name}`: the name of the binary (`21-01`, etc).
    /// - `{year}`: the name of the binary (`21`, etc).
    /// - `{day}`: the day of the binary (`1`, etc).
    /// - `{day0}`: the zero padded day of the binary (`01`, etc).
    #[arg(
        long,
        default_value = "inputs/{name}.txt",
        value_parser = SourceValueParser,
        verbatim_doc_comment,
        conflicts_with = "use_examples",
    )]
    input_pattern: Source,

    /// Pattern for paths to files containing the expected results.
    ///
    /// The following tokens will be replaced:
    /// - `{name}`: the name of the binary (`21-01`, etc).
    /// - `{year}`: the name of the binary (`21`, etc).
    /// - `{day}`: the day of the binary (`1`, etc).
    /// - `{day0}`: the zero padded day of the binary (`01`, etc).
    /// - `{part}`: the number of the part (`1` or `2`).
    #[arg(
        long,
        default_value = "inputs/{name}-{part}.txt",
        value_parser = SourceValueParser,
        verbatim_doc_comment,
        conflicts_with = "use_examples",
    )]
    result_pattern: Source,

    /// Run using examples instead of real inputs/results.
    #[arg(long)]
    use_examples: bool,
}
impl TargetArgs {
    pub(super) fn filtered_binaries(&self) -> Vec<Bin> {
        let mut bins = BINS.get().unwrap().clone();
        if let Some(only) = &self.only {
            let only: HashSet<_> = only.iter().flatten().collect();
            for bin in &mut bins {
                if !only.contains(&(bin.year, bin.day, 1)) {
                    bin.part1 = Solver::NotImplemented;
                }
                if !only.contains(&(bin.year, bin.day, 2)) {
                    bin.part2 = Solver::NotImplemented;
                }
            }
        } else if let Some(skip) = &self.skip {
            let skip: HashSet<_> = skip.iter().flatten().collect();
            for bin in &mut bins {
                if skip.contains(&(bin.year, bin.day, 1)) {
                    bin.part1 = Solver::NotImplemented;
                }
                if skip.contains(&(bin.year, bin.day, 2)) {
                    bin.part2 = Solver::NotImplemented;
                }
            }
        }
        bins.into_iter()
            .filter(|bin| bin.part1.is_implemented() || bin.part2.is_implemented())
            .collect()
    }

    pub(super) fn get_targets(&self, bins: &[Bin]) -> Vec<Target> {
        let mut targets = Vec::new();
        if self.use_examples {
            for bin in bins {
                for example in &bin.examples {
                    for (i, solver, solution) in [
                        (1, &bin.part1, example.part1),
                        (2, &bin.part2, example.part2),
                    ] {
                        if !solver.is_implemented() {
                            continue;
                        }
                        let Some(solution) = solution else {
                            continue;
                        };
                        targets.push(Target {
                            bin: bin.name.to_owned(),
                            part: i,
                            source_name: Some(example.name.to_owned()),
                            solver: solver.to_owned(),
                            input: Source::Inline {
                                source: example.name.to_owned(),
                                contents: example.input.to_owned(),
                            },
                            solution: Source::Inline {
                                source: example.name.to_owned(),
                                contents: solution.to_owned(),
                            },
                        });
                    }
                }
            }
        } else {
            for bin in bins {
                let input = source_path_fill_tokens!(self.input_pattern, bin = bin);
                for (i, solver) in [(1, &bin.part1), (2, &bin.part2)] {
                    if !solver.is_implemented() {
                        continue;
                    }
                    let solution =
                        source_path_fill_tokens!(self.result_pattern, bin = bin, part = i);
                    targets.push(Target {
                        bin: bin.name.to_owned(),
                        part: i,
                        source_name: None,
                        solver: solver.to_owned(),
                        input: input.clone(),
                        solution,
                    });
                }
            }
        }
        targets
    }
}

pub(super) struct Target {
    pub(super) bin: String,
    pub(super) part: u8,
    pub(super) source_name: Option<String>,
    pub(super) solver: Solver<String>,
    pub(super) input: Source,
    pub(super) solution: Source,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct MainArgs {
    #[command(flatten)]
    targets: TargetArgs,

    /// Show the results in addition to the pass/fail (which is always shown).
    #[arg(short = 'r', long)]
    show_results: bool,
}

/// The entrypoint for the multi-day CLI.
pub fn main() {
    const {
        assert!(
            !cfg!(feature = "visual"),
            "this entrypoint doesn't support feature 'visual'."
        );
    }

    let args = MainArgs::parse();

    let bins = args.targets.filtered_binaries();
    let targets = args.targets.get_targets(&bins);
    println!(
        "Running {} solves across {} solutions across {} binaries...",
        Cyan.paint(targets.len().to_string()),
        Cyan.paint(
            bins.iter()
                .map(|d| u8::from(d.part1.is_implemented()) + u8::from(d.part2.is_implemented()))
                .sum::<u8>()
                .to_string()
        ),
        Cyan.paint(bins.len().to_string()),
    );

    // Initialize the thread pool now. This will happen automatically when it's first needed, but if this is inside a solution this will add to the runtime of that solution, unfairly penalizing it for being the first to use rayon while the other solutions that also do so get a free pass.
    ThreadPoolBuilder::new().build_global().unwrap();

    let runs: Vec<(String, SolverResult)> = targets
        .into_iter()
        .map(|target| {
            let mut name = format!("{} part {}", target.bin, target.part);
            if let Some(source) = target.source_name {
                name = format!("{name} {source}");
            }

            let input = match target.input.read() {
                Ok(input) => input,
                Err(err) => {
                    return (name, SolverResult::Error(err));
                }
            };

            (
                name,
                match target.solution.read_maybe() {
                    Ok(solution) => target.solver.run::<InstantTimer>(&input, solution),
                    Err(err) => SolverResult::Error(err),
                },
            )
        })
        .collect();

    let durations = runs
        .iter()
        .filter_map(|(_, r)| match r {
            SolverResult::Success { duration, .. } => Some(*duration),
            SolverResult::Error(_) => None,
        })
        .collect::<Vec<_>>();
    let duration_total = durations.iter().sum::<Duration>();
    let duration_avg = if durations.is_empty() {
        Duration::from_secs(0)
    } else {
        duration_total / durations.len() as u32
    };
    let thresholds = DurationThresholds {
        good: duration_avg / 3,
        acceptable: duration_avg * 2 / 3,
    };
    for (name, result) in runs {
        result.print(&name, &thresholds, args.show_results);
    }
    if !durations.is_empty() {
        println!(
            "Finished {} runs in {}, averaging {} per run.",
            Cyan.paint(durations.len().to_string()),
            Purple.paint(format!("{duration_total:?}")),
            Purple.paint(format!("{duration_avg:?}",)),
        );
    }
}
