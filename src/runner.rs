use std::{
    env, fs,
    time::{Duration, Instant},
};

use ansi_term::{
    unstyle, ANSIStrings,
    Colour::{Blue, Cyan, Green, Purple, Red},
};

pub enum Runnable<T, F>
where
    T: ToString,
    F: Fn(&str) -> T,
{
    Implemented(F),
    Missing,
}
impl<F, T> From<F> for Runnable<T, F>
where
    T: ToString,
    F: Fn(&str) -> T,
{
    fn from(value: F) -> Self {
        Runnable::Implemented(value)
    }
}
impl From<()> for Runnable<String, fn(&str) -> String> {
    fn from(_value: ()) -> Self {
        Runnable::Missing
    }
}

#[derive(Clone)]
pub struct RunnableRunOk {
    pub result: String,
    pub solution: Option<String>,
    pub duration: Duration,
}

pub type RunnableRun = Result<RunnableRunOk, String>;

pub struct DurationThresholds {
    pub good: Duration,
    pub acceptable: Duration,
}
const THRESHOLDS_DEFAULT: DurationThresholds = DurationThresholds {
    good: Duration::from_millis(1),
    acceptable: Duration::from_secs(1),
};

pub fn print_runnable_run(
    name: String,
    run: RunnableRun,
    thresholds: &DurationThresholds,
    show_result: bool,
) {
    let name = Purple.paint(name);
    match run {
        Err(err) => {
            println!("> {name}: {}", Red.paint(err));
        }
        Ok(run) => {
            let duration_colour = if run.duration < thresholds.good {
                Green
            } else if run.duration < thresholds.acceptable {
                Blue
            } else {
                Red
            };
            let duration_formatted = duration_colour.paint(format!("{:?}", run.duration));

            if !show_result {
                let name = if run.solution.map_or(true, |s| s == run.result) {
                    name
                } else {
                    Red.paint(unstyle(&ANSIStrings(&[name])))
                };
                println!("> {name} [{duration_formatted}]");
                return;
            }

            let result_formatted = match run.solution {
                Some(expected) => {
                    if run.result == expected {
                        Green.paint(&run.result).to_string()
                    } else if run.result.contains('\n') || expected.contains('\n') {
                        format!("{}\nShould be:\n{expected}", Red.paint(&run.result))
                    } else {
                        format!("{} (should be {expected})", Red.paint(&run.result))
                    }
                }
                None => run.result.clone(),
            };

            if result_formatted.contains('\n') {
                println!("> {name}: [{duration_formatted}]");
                for line in result_formatted.split('\n') {
                    println!("  {line}");
                }
            } else {
                println!("> {name}: {result_formatted} [{duration_formatted}]");
            }
        }
    }
}

fn run_runnable<T, F>(
    runnable: &Runnable<T, F>,
    input: &str,
    solution: Option<String>,
) -> RunnableRun
where
    T: ToString,
    F: Fn(&str) -> T,
{
    let Runnable::Implemented(runnable) = runnable else {
        return Err("Not implemented.".to_string());
    };

    let start = Instant::now();
    let result = ToString::to_string(&runnable(input));
    let duration = start.elapsed();

    Ok(RunnableRunOk {
        result,
        solution,
        duration,
    })
}

#[allow(clippy::must_use_candidate)]
pub fn get_input_path(name: &str) -> String {
    format!("inputs/{name}.txt")
}

#[allow(clippy::must_use_candidate)]
pub fn get_output_path(input_path: &str, part: i8) -> String {
    if input_path.contains('.') {
        let [tail, head]: [&str; 2] = input_path
            .rsplitn(2, '.')
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        format!("{head}.solution{part}.{tail}")
    } else {
        format!("{input_path}.solution{part}")
    }
}

pub fn run_day<T1, F1, T2, F2>(
    filename: &String,
    part1: &Runnable<T1, F1>,
    part2: &Runnable<T2, F2>,
) -> Result<(RunnableRun, RunnableRun), String>
where
    T1: ToString,
    F1: Fn(&str) -> T1,
    T2: ToString,
    F2: Fn(&str) -> T2,
{
    match fs::read_to_string(filename) {
        Ok(input) => Ok((
            run_runnable(
                part1,
                &input,
                fs::read_to_string(get_output_path(filename, 1)).ok(),
            ),
            run_runnable(
                part2,
                &input,
                fs::read_to_string(get_output_path(filename, 2)).ok(),
            ),
        )),
        Err(err) => Err(format!("Unable to read input file '{filename}': {err}.")),
    }
}

pub fn run<T1, F1, T2, F2>(part1: impl Into<Runnable<T1, F1>>, part2: impl Into<Runnable<T2, F2>>)
where
    T1: ToString,
    F1: Fn(&str) -> T1,
    T2: ToString,
    F2: Fn(&str) -> T2,
{
    let args: Vec<String> = env::args().collect();

    let name = args[0]
        .split('/')
        .last()
        .expect("Unable to determine binary name.");

    let filenames: Vec<String> = if args.len() > 1 {
        args.iter().skip(1).cloned().collect()
    } else {
        vec![get_input_path(name)]
    };

    let part1 = part1.into();
    let part2 = part2.into();
    for filename in &filenames {
        println!(
            "Running {} using input {}...",
            Cyan.paint(name),
            Cyan.paint(filename)
        );
        let (run1, run2) = run_day(filename, &part1, &part2).unwrap();
        print_runnable_run("Part 1".to_string(), run1, &THRESHOLDS_DEFAULT, true);
        print_runnable_run("Part 2".to_string(), run2, &THRESHOLDS_DEFAULT, true);
    }
}
