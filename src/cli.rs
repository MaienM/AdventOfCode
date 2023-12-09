use std::{fs::read_to_string, io::ErrorKind};

use clap::{
    builder::{StringValueParser, TypedValueParser},
    parser::ValueSource,
};

/// The source of a solution input or expected output.
#[derive(Clone, Debug)]
pub enum Source {
    /// A path that was explicitly passed in by the user.
    ///
    /// Any error while reading this will be reported back.
    ExplicitPath(String),

    /// A path that was automatically chosen by the program.
    ///
    /// A "file does not exist" error will be treated as if there was no path, but any other IO error will be reported back to the user.
    AutomaticPath(String),

    /// An inline value.
    Inline { source: String, contents: String },

    /// No value available.
    None(
        /// A description of the purpose of this path.
        String,
    ),
}
impl Source {
    fn read_path(path: &str) -> Result<String, (ErrorKind, String)> {
        read_to_string(path)
            .map(|contents| contents.strip_suffix('\n').unwrap_or(&contents).to_owned())
            .map_err(|err| (err.kind(), format!("Failed to read {path}: {err}")))
    }

    /// Get the source of the value, if any.
    pub fn source(&self) -> Result<String, String> {
        match self {
            Source::ExplicitPath(path) | Source::AutomaticPath(path) => Ok(path.clone()),
            Source::Inline { source, .. } => Ok(source.clone()),
            Source::None(description) => Err(format!("No value for {description}.")),
        }
    }

    /// Attempt to read the file at the provided path, returning [`None`] when a non-fatal error occurs.
    pub fn read_maybe(&self) -> Result<Option<String>, String> {
        match self {
            Source::ExplicitPath(path) => Ok(Some(Self::read_path(path).map_err(|(_, e)| e)?)),
            Source::AutomaticPath(path) => match Self::read_path(path) {
                Ok(contents) => Ok(Some(contents)),
                Err((ErrorKind::NotFound, _)) => Ok(None),
                Err((_, err)) => Err(err),
            },
            Source::Inline { contents, .. } => Ok(Some(contents.clone())),
            Source::None(_) => Ok(None),
        }
    }

    /// Attempt to read the file at the provided path, returning an error if this fails for any reason.
    pub fn read(&self) -> Result<String, String> {
        match self {
            Source::ExplicitPath(path) | Source::AutomaticPath(path) => {
                Self::read_path(path).map_err(|(_, e)| e)
            }
            Source::Inline { contents, .. } => Ok(contents.clone()),
            Source::None(description) => Err(format!("No value for {description}.")),
        }
    }

    /// Mutate the contained path (if any). Does nothing for [`Source::None`].
    #[must_use]
    pub fn mutate_path<F>(&self, f: F) -> Self
    where
        F: Fn(String) -> String,
    {
        let mut result = self.clone();
        match result {
            Source::ExplicitPath(ref mut path) | Source::AutomaticPath(ref mut path) => {
                *path = f(std::mem::take(path));
            }
            _ => {}
        };
        result
    }
}

/// Parse arguments to [`Source`]s.
#[derive(Clone)]
pub struct SourceValueParser;
impl TypedValueParser for SourceValueParser {
    type Value = Source;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        _value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        panic!("Should never be called as parse_ref_ is implemented.");
    }

    fn parse_ref_(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
        source: ValueSource,
    ) -> Result<Self::Value, clap::Error> {
        let value = StringValueParser::new().parse_ref_(cmd, arg, value, source)?;

        if source == ValueSource::DefaultValue {
            Ok(Source::AutomaticPath(value))
        } else if value.is_empty() {
            Ok(Source::None(
                arg.map_or("unknown".to_owned(), |a| a.get_id().to_string()),
            ))
        } else {
            Ok(Source::ExplicitPath(value))
        }
    }
}

macro_rules! file_path_fill_tokens {
    ($path:expr, day = $day:expr) => {
        $path.mutate_path(|p| {
            p.replace("{day}", &$day.num.to_string())
                .replace("{day0}", &$day.name[3..])
        })
    };
    ($path:expr, day = $day:expr, part = $part:expr) => {
        file_path_fill_tokens!($path, day = $day)
            .mutate_path(|p| p.replace("{part}", &$part.to_string()))
    };
}

pub mod multi {
    use std::{collections::HashSet, time::Duration};

    use ansi_term::Colour::{Cyan, Purple};
    use clap::{
        builder::{ArgPredicate, PossibleValue, PossibleValuesParser, TypedValueParser},
        CommandFactory, FromArgMatches, Parser,
    };
    use criterion::Criterion;

    use super::{Source, SourceValueParser};
    use crate::{
        derived::Day,
        parse::splitn,
        runner::{DurationThresholds, Solver, SolverRunResult},
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
            if day.part1.is_implemented() {
                options.push(create_value(day.num, "-1"));
            }
            if day.part2.is_implemented() {
                options.push(create_value(day.num, "-2"));
            }
        }
        let parser = PossibleValuesParser::new(options);

        parser.map(|s| {
            let s = s.trim_start_matches('0');
            if s.contains('-') {
                let (day, part) = splitn!(s, '-', u8, u8);
                vec![(day, part)]
            } else {
                let day = s.parse().unwrap();
                vec![(day, 1), (day, 2)]
            }
        })
    }

    #[derive(Parser, Debug)]
    struct CommonArgs {
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
    impl CommonArgs {
        fn filter_days(&self, days: &[Day]) -> Vec<Day> {
            let mut days = days.to_owned();
            if let Some(only) = &self.only {
                let only: HashSet<_> = only.iter().flatten().collect();
                for day in &mut days {
                    if !only.contains(&(day.num, 1)) {
                        day.part1 = Solver::NotImplemented;
                    }
                    if !only.contains(&(day.num, 2)) {
                        day.part2 = Solver::NotImplemented;
                    }
                }
            } else if let Some(skip) = &self.skip {
                let skip: HashSet<_> = skip.iter().flatten().collect();
                for day in &mut days {
                    if skip.contains(&(day.num, 1)) {
                        day.part1 = Solver::NotImplemented;
                    }
                    if skip.contains(&(day.num, 2)) {
                        day.part2 = Solver::NotImplemented;
                    }
                }
            }
            days.into_iter()
                .filter(|day| day.part1.is_implemented() || day.part2.is_implemented())
                .collect()
        }

        fn get_targets(&self, days: &[Day]) -> Vec<Target> {
            let mut targets = Vec::new();
            if self.use_examples {
                for day in days {
                    for example in &day.examples {
                        for (i, solver, solution) in [
                            (1, &day.part1, example.part1),
                            (2, &day.part2, example.part2),
                        ] {
                            if !solver.is_implemented() {
                                continue;
                            }
                            let Some(solution) = solution else {
                                continue;
                            };
                            targets.push(Target {
                                day: day.name.to_owned(),
                                part: i,
                                source_name: Some(example.name.to_owned()),
                                solver: solver.clone(),
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
                    let input = file_path_fill_tokens!(self.input_pattern, day = day);
                    for (i, solver) in [(1, &day.part1), (2, &day.part2)] {
                        if solver.is_implemented() {
                            let solution =
                                file_path_fill_tokens!(self.result_pattern, day = day, part = i);
                            targets.push(Target {
                                day: day.name.to_owned(),
                                part: i,
                                source_name: None,
                                solver: solver.clone(),
                                input: input.clone(),
                                solution,
                            });
                        }
                    }
                }
            }
            targets
        }
    }

    struct Target {
        day: String,
        part: u8,
        source_name: Option<String>,
        solver: Solver<String>,
        input: Source,
        solution: Source,
    }

    fn parse_args<T>(days: &[Day]) -> T
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
        common: CommonArgs,

        /// Show the results in addition to the pass/fail (which is always shown).
        #[arg(short = 'r', long)]
        show_results: bool,
    }

    pub fn main(days: &[Day]) {
        let args: MainArgs = parse_args(days);

        let days = args.common.filter_days(days);
        let targets = args.common.get_targets(&days);
        println!(
            "Running {} runs, across {} parts, across {} days...",
            Cyan.paint(targets.len().to_string()),
            Cyan.paint(
                days
                    .iter()
                    .map(|d| u8::from(d.part1.is_implemented()) + u8::from(d.part2.is_implemented()))
                    .sum::<u8>()
                    .to_string()
            ),
            Cyan.paint(targets.len().to_string()),
        );

        let runs: Vec<(String, SolverRunResult)> = targets
            .into_iter()
            .map(|target| {
                let mut name =
                    format!("{} part {}", target.day.replace("day", "Day "), target.part);
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

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct BenchArgs {
        #[command(flatten)]
        common: CommonArgs,

        /// Noop for compatibility.
        #[arg(long, num_args = 0)]
        bench: (),

        /// Save results under a named baseline.
        #[arg(
            short = 's',
            long,
            default_value_if("baseline", ArgPredicate::IsPresent, None),
            default_value = "base",
            conflicts_with = "baseline"
        )]
        save_baseline: Option<String>,

        /// Compare to a named baseline.
        ///
        /// If any benchmarks do not have the specified baseline this command fails.
        #[arg(short = 'b', long)]
        baseline: Option<String>,
    }

    pub fn bench(days: &[Day]) {
        let args: BenchArgs = parse_args(days);

        let mut criterion = Criterion::default();
        if let Some(name) = args.save_baseline {
            criterion = criterion.save_baseline(name);
        } else if let Some(name) = args.baseline {
            criterion = criterion.retain_baseline(name, true);
        }

        let days = args.common.filter_days(days);
        for target in args.common.get_targets(&days) {
            let Solver::Implemented(runnable) = target.solver else {
                continue;
            };

            let mut name = format!("{}/part{}", target.day, target.part);
            if let Some(source) = target.source_name {
                name = format!("{name}/{source}");
            }

            let input = target.input.read().unwrap();

            criterion.bench_function(&name, |b| {
                b.iter(|| runnable(&input));
            });
        }

        criterion.final_summary();
    }
}

pub mod single {
    use std::time::Duration;

    use ansi_term::Colour::{Cyan, Red};
    use clap::{Parser, ValueHint};

    use super::{Source, SourceValueParser};
    use crate::{
        derived::Day,
        runner::{DurationThresholds, SolverRunResult},
    };

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct SingleArgs {
        /// Path to a file containing the input.
        #[arg(
            value_hint = ValueHint::FilePath,
            default_value = "inputs/day{day0}.txt",
            value_parser = SourceValueParser,
        )]
        input: Source,

        /// Path to a file containing the expected result of part 1.
        #[arg(
            value_hint = ValueHint::FilePath,
            default_value = "inputs/day{day0}.solution{part}.txt",
            value_parser = SourceValueParser,
        )]
        part1: Source,

        /// Path to a file containing the expected result of part 2.
        #[arg(
            value_hint = ValueHint::FilePath,
            default_value = "inputs/day{day0}.solution{part}.txt",
            value_parser = SourceValueParser,
        )]
        part2: Source,
    }

    const THRESHOLDS: DurationThresholds = DurationThresholds {
        good: Duration::from_millis(1),
        acceptable: Duration::from_secs(1),
    };

    pub fn main(day: &Day) {
        let args = SingleArgs::parse();

        let input_path = file_path_fill_tokens!(args.input, day = day);
        let part1_path = file_path_fill_tokens!(args.part1, day = day, part = 1);
        let part2_path = file_path_fill_tokens!(args.part2, day = day, part = 2);

        println!(
            "Running {} using input {}...",
            Cyan.paint(format!("day {}", day.num)),
            Cyan.paint(input_path.source().unwrap()),
        );

        let input = match input_path.read() {
            Ok(input) => input,
            Err(err) => {
                println!("{}", Red.paint(err));
                return;
            }
        };

        for (i, part, solution_path) in [(1, &day.part1, part1_path), (2, &day.part2, part2_path)] {
            let solution = solution_path.read_maybe();
            let result = match solution {
                Ok(solution) => part.run(&input, solution),
                Err(err) => SolverRunResult::Error(err),
            };
            result.print(&format!("Part {i}"), &THRESHOLDS, true);
        }
    }

    #[macro_export]
    macro_rules! __generate_day_main__ {
        () => {
            #[::aoc_derive::inject_day]
            static DAY: Day;

            pub fn main() {
                ::aoc::cli::single::main(&*DAY);
            }
        };
    }
    pub use __generate_day_main__ as generate_main;
}
