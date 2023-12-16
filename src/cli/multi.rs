use std::{collections::HashSet, time::Duration};

use ansi_term::Colour::{Cyan, Purple};
use clap::{
    builder::{PossibleValue, PossibleValuesParser, TypedValueParser},
    CommandFactory, FromArgMatches, Parser,
};

use super::source::source_path_fill_tokens;
use crate::{
    cli::{
        runner::{DurationThresholds, Solver, SolverRunResult},
        source::{Source, SourceValueParser},
    },
    derived::Day,
    utils::parse,
};

/// Create parser for --only/--skip.
fn create_target_value_parser(days: &[Day]) -> impl TypedValueParser {
    fn create_value(num: u8, suffix: &str) -> PossibleValue {
        let mut v = PossibleValue::new(format!("{num}{suffix}"));
        if num < 10 {
            v = v.alias(format!("0{num}{suffix}"));
        }
        v
    }

    let mut options = Vec::new();
    for day in days {
        options.push(create_value(day.num, ""));
        if day.part1.is_some() {
            options.push(create_value(day.num, "-1"));
        }
        if day.part2.is_some() {
            options.push(create_value(day.num, "-2"));
        }
    }
    let parser = PossibleValuesParser::new(options);

    parser.map(|s| {
        let s = s.trim_start_matches('0');
        if s.contains('-') {
            parse!(s => [day as u8] "-" [part as u8]);
            vec![(day, part)]
        } else {
            let day = s.parse().unwrap();
            vec![(day, 1), (day, 2)]
        }
    })
}

#[derive(Parser, Debug)]
pub(super) struct TargetArgs {
    /// Only run the listed days.
    ///
    /// The syntax {day}-{part} can be used to target only a single part for a day.
    #[arg(
        long,
        value_delimiter = ',',
        value_name = "1,3,8-1",
        value_parser = create_target_value_parser(&[]),
        group = "targets",
    )]
    only: Option<Vec<Vec<(u8, u8)>>>,

    /// Skip the listed days.
    ///
    /// The syntax {day}-{part} can be used to target only a single part for a day.
    #[arg(
        long,
        value_delimiter = ',',
        value_name = "1,3,8-1",
        value_parser = create_target_value_parser(&[]),
        group = "targets",
    )]
    skip: Option<Vec<Vec<(u8, u8)>>>,

    /// Pattern for paths to files containing the inputs.
    ///
    /// The following tokens will be replaced:
    /// - `{day}`: the number of the day (`1`, `13`, etc).
    /// - `{day0}`: the number of the day as two digits (`01`, `13`, etc).
    #[arg(
        long,
        default_value = "inputs/day{day0}.txt",
        value_parser = SourceValueParser,
        verbatim_doc_comment,
        conflicts_with = "use_examples",
    )]
    input_pattern: Source,

    /// Pattern for paths to files containing the expected results.
    ///
    /// The following tokens will be replaced:
    /// - `{day}`: the number of the day (`1`, `13`, etc).
    /// - `{day0}`: the number of the day as two digits (`01`, `13`, etc).
    /// - `{part}`: the number of the part (`1` or `2`).
    #[arg(
        long,
        default_value = "inputs/day{day0}.solution{part}.txt",
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
    pub(super) fn filter_days(&self, days: &[Day]) -> Vec<Day> {
        let mut days = days.to_owned();
        if let Some(only) = &self.only {
            let only: HashSet<_> = only.iter().flatten().collect();
            for day in &mut days {
                if !only.contains(&(day.num, 1)) {
                    day.part1 = None;
                }
                if !only.contains(&(day.num, 2)) {
                    day.part2 = None;
                }
            }
        } else if let Some(skip) = &self.skip {
            let skip: HashSet<_> = skip.iter().flatten().collect();
            for day in &mut days {
                if skip.contains(&(day.num, 1)) {
                    day.part1 = None;
                }
                if skip.contains(&(day.num, 2)) {
                    day.part2 = None;
                }
            }
        }
        days.into_iter()
            .filter(|day| day.part1.is_some() || day.part2.is_some())
            .collect()
    }

    pub(super) fn get_targets(&self, days: &[Day]) -> Vec<Target> {
        let mut targets = Vec::new();
        if self.use_examples {
            for day in days {
                for example in &day.examples {
                    for (i, solver, solution) in [
                        (1, &day.part1, example.part1),
                        (2, &day.part2, example.part2),
                    ] {
                        if !solver.is_some() {
                            continue;
                        }
                        let Some(solution) = solution else {
                            continue;
                        };
                        targets.push(Target {
                            day: day.name.to_owned(),
                            part: i,
                            source_name: Some(example.name.to_owned()),
                            solver: (*solver).into(),
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
            for day in days {
                let input = source_path_fill_tokens!(self.input_pattern, day = day);
                for (i, solver) in [(1, &day.part1), (2, &day.part2)] {
                    if solver.is_none() {
                        continue;
                    }
                    let solution =
                        source_path_fill_tokens!(self.result_pattern, day = day, part = i);
                    targets.push(Target {
                        day: day.name.to_owned(),
                        part: i,
                        source_name: None,
                        solver: (*solver).into(),
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
    pub(super) day: String,
    pub(super) part: u8,
    pub(super) source_name: Option<String>,
    pub(super) solver: Solver<String>,
    pub(super) input: Source,
    pub(super) solution: Source,
}

pub(super) fn parse_args_with_targets<T>(days: &[Day]) -> T
where
    T: CommandFactory + FromArgMatches,
{
    let mut command = <T as CommandFactory>::command()
        .mut_arg("only", |a| a.value_parser(create_target_value_parser(days)))
        .mut_arg("skip", |a| a.value_parser(create_target_value_parser(days)));
    <T as FromArgMatches>::from_arg_matches_mut(&mut command.clone().get_matches())
        .map_err(|err| err.format(&mut command))
        .unwrap()
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

pub fn main(days: &[Day]) {
    let args: MainArgs = parse_args_with_targets(days);

    let days = args.targets.filter_days(days);
    let targets = args.targets.get_targets(&days);
    println!(
        "Running {} runs, across {} parts, across {} days...",
        Cyan.paint(targets.len().to_string()),
        Cyan.paint(
            days.iter()
                .map(|d| u8::from(d.part1.is_some()) + u8::from(d.part2.is_some()))
                .sum::<u8>()
                .to_string()
        ),
        Cyan.paint(days.len().to_string()),
    );

    let runs: Vec<(String, SolverRunResult)> = targets
        .into_iter()
        .map(|target| {
            let mut name = format!("{} part {}", target.day.replace("day", "Day "), target.part);
            if let Some(source) = target.source_name {
                name = format!("{name} {source}");
            }

            let input = match target.input.read() {
                Ok(input) => input,
                Err(err) => {
                    return (name, SolverRunResult::Error(err));
                }
            };

            (
                name,
                match target.solution.read_maybe() {
                    Ok(solution) => target.solver.run(&input, solution),
                    Err(err) => SolverRunResult::Error(err),
                },
            )
        })
        .collect();

    let durations = runs
        .iter()
        .filter_map(|(_, r)| match r {
            SolverRunResult::Success { duration, .. } => Some(*duration),
            SolverRunResult::Error(_) => None,
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
