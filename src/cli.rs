use std::{fs::read_to_string, io::ErrorKind};

use clap::{
    builder::{StringValueParser, TypedValueParser},
    parser::ValueSource,
};

/// A path to a file containing a solution input or expected output.
#[derive(Clone, Debug)]
pub enum FilePath {
    /// A path that was explicitly passed in by the user.
    ///
    /// Any error while reading this will be reported back.
    Explicit(String),

    /// A path that was automatically chosen by the program.
    ///
    /// A "file does not exist" error will be treated as if there was no path, but any other IO error will be reported back to the user.
    Automatic(String),

    /// No path available.
    None(
        /// A description of the purpose of this path.
        String,
    ),
}
impl FilePath {
    fn read_path(path: &str) -> Result<String, (ErrorKind, String)> {
        read_to_string(path)
            .map(|contents| contents.strip_suffix('\n').unwrap_or(&contents).to_owned())
            .map_err(|err| (err.kind(), format!("Failed to read {path}: {err}")))
    }

    /// Get the path, if any.
    pub fn path(&self) -> Result<String, String> {
        match self {
            FilePath::Explicit(path) | FilePath::Automatic(path) => Ok(path.clone()),
            FilePath::None(description) => Err(format!("No path for {description}.")),
        }
    }

    /// Attempt to read the file at the provided path, returning [`None`] when a non-fatal error occurs.
    pub fn read_maybe(&self) -> Result<Option<String>, String> {
        match self {
            FilePath::Explicit(path) => Ok(Some(Self::read_path(path).map_err(|(_, e)| e)?)),
            FilePath::Automatic(path) => match Self::read_path(path) {
                Ok(contents) => Ok(Some(contents)),
                Err((ErrorKind::NotFound, _)) => Ok(None),
                Err((_, err)) => Err(err),
            },
            FilePath::None(_) => Ok(None),
        }
    }

    /// Attempt to read the file at the provided path, returning an error if this fails for any reason.
    pub fn read(&self) -> Result<String, String> {
        self.path()
            .and_then(|path| Self::read_path(&path).map_err(|(_, e)| e))
    }

    /// Mutate the contained path (if any). Does nothing for [`FilePath::None`].
    #[must_use]
    pub fn mutate_path<F>(&self, f: F) -> Self
    where
        F: Fn(String) -> String,
    {
        let mut result = self.clone();
        match result {
            FilePath::Explicit(ref mut path) | FilePath::Automatic(ref mut path) => {
                *path = f(std::mem::take(path));
            }
            FilePath::None(_) => {}
        };
        result
    }
}

/// Parse arguments to [`FilePath`]s.
#[derive(Clone)]
pub struct FilePathParser;
impl TypedValueParser for FilePathParser {
    type Value = FilePath;

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
            Ok(FilePath::Automatic(value))
        } else if value.is_empty() {
            Ok(FilePath::None(
                arg.map_or("unknown".to_owned(), |a| a.get_id().to_string()),
            ))
        } else {
            Ok(FilePath::Explicit(value))
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

    use super::{FilePath, FilePathParser};
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
    }
    impl CommonArgs {
        fn get_targets(&self, days: &[Day]) -> Vec<Day> {
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

        /// Pattern for paths to files containing the inputs.
        ///
        /// The following tokens will be replaced:
        /// - `{day}`: the number of the day (`1`, `13`, etc).
        /// - `{day0}`: the number of the day as two digits (`01`, `13`, etc).
        #[arg(
            long,
            default_value = "inputs/day{day0}.txt",
            value_parser = FilePathParser,
            verbatim_doc_comment,
        )]
        input_pattern: FilePath,

        /// Pattern for paths to files containing the expected results.
        ///
        /// The following tokens will be replaced:
        /// - `{day}`: the number of the day (`1`, `13`, etc).
        /// - `{day0}`: the number of the day as two digits (`01`, `13`, etc).
        /// - `{part}`: the number of the part (`1` or `2`).
        #[arg(
            long,
            default_value = "inputs/day{day0}.solution{part}.txt",
            value_parser = FilePathParser,
            verbatim_doc_comment
        )]
        result_pattern: FilePath,

        /// Show the results in addition to the pass/fail (which is always shown).
        #[arg(short = 'r', long)]
        show_results: bool,
    }

    pub fn main(days: &[Day]) {
        let args: MainArgs = parse_args(days);

        let targets = args.common.get_targets(days);
        println!(
            "Running {} parts over {} days...",
            Cyan.paint(
                targets
                    .iter()
                    .map(|d| u8::from(d.part1.is_implemented()) + u8::from(d.part2.is_implemented()))
                    .sum::<u8>()
                    .to_string()
            ),
            Cyan.paint(targets.len().to_string()),
        );

        let mut runs: Vec<(String, SolverRunResult)> = Vec::new();
        for day in targets {
            let name = day.name.replace("day", "Day ");

            let input_path = file_path_fill_tokens!(args.input_pattern, day = day);
            let input = match input_path.read() {
                Ok(input) => input,
                Err(err) => {
                    runs.push((name, SolverRunResult::Error(err)));
                    continue;
                }
            };

            for (i, part) in [(1, day.part1), (2, day.part2)] {
                if part.is_implemented() {
                    let solution = file_path_fill_tokens!(args.result_pattern, day = day, part = i)
                        .read_maybe();
                    runs.push((
                        format!("{name} part {i}"),
                        match solution {
                            Ok(solution) => part.run(&input, solution),
                            Err(err) => SolverRunResult::Error(err),
                        },
                    ));
                }
            }
        }

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
                "Finished {} parts in {}, averaging {} per part.",
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

        for day in args.common.get_targets(days) {
            for example in day.examples {
                for (i, part, solution) in [
                    (1, &day.part1, example.part1),
                    (2, &day.part2, example.part2),
                ] {
                    if solution.is_none() {
                        continue;
                    }
                    let Solver::Implemented(runnable) = part else {
                        continue;
                    };
                    criterion.bench_function(
                        &format!("{}/part{}/{}", day.name, i, example.name),
                        |b| {
                            b.iter(|| runnable(example.input));
                        },
                    );
                }
            }
        }

        criterion.final_summary();
    }
}

pub mod single {
    use std::time::Duration;

    use ansi_term::Colour::{Cyan, Red};
    use clap::{Parser, ValueHint};

    use super::{FilePath, FilePathParser};
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
            value_parser = FilePathParser,
        )]
        input: FilePath,

        /// Path to a file containing the expected result of part 1.
        #[arg(
            value_hint = ValueHint::FilePath,
            default_value = "inputs/day{day0}.solution{part}.txt",
            value_parser = FilePathParser,
        )]
        part1: FilePath,

        /// Path to a file containing the expected result of part 2.
        #[arg(
            value_hint = ValueHint::FilePath,
            default_value = "inputs/day{day0}.solution{part}.txt",
            value_parser = FilePathParser,
        )]
        part2: FilePath,
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
            Cyan.paint(input_path.path().unwrap()),
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
