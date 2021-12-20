use std::env;
use std::fs;
use std::time::Duration;
use std::time::Instant;

use ansi_term::Colour::*;

type Runnable<T> = fn(String) -> T;

// Used with a proc-macro to provide a listing of all Runnables of all days.
pub type RunnableList = Vec<(&'static str, Runnable<String>, Runnable<String>)>;
pub trait RunnableListProvider {
    fn get() -> RunnableList;
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

pub fn print_runnable_run(name: String, run: RunnableRun, thresholds: &DurationThresholds) {
    let name = Purple.paint(name);
    match run {
        Err(err) => {
            println!("> {}: {}", name, Red.paint(err));
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

            let result_formatted = match run.solution {
                Some(expected) => {
                    if run.result == expected {
                        Green.paint(&run.result).to_string()
                    } else {
                        if run.result.contains("\n") || expected.contains("\n") {
                            format!("{}\nShould be:\n{}", Red.paint(&run.result), expected)
                        } else {
                            format!("{} (should be {})", Red.paint(&run.result), expected)
                        }
                    }
                }
                None => run.result.clone(),
            };

            if result_formatted.contains("\n") {
                println!("> {}: [{}]", name, duration_formatted);
                for line in result_formatted.split("\n") {
                    println!("  {}", line);
                }
            } else {
                println!("> {}: {} [{}]", name, result_formatted, duration_formatted);
            }
        }
    }
}

fn run_runnable<T: ToString>(
    runnable: Runnable<T>,
    input: &String,
    solution: Option<String>,
) -> RunnableRun {
    if runnable == missing {
        return Err("Not implemented.".to_string());
    }

    let start = Instant::now();
    let result = runnable(input.to_owned());
    let duration = start.elapsed();

    let result = result.to_string();

    return Ok(RunnableRunOk {
        result,
        duration,
        solution,
    });
}

pub fn get_input_path(name: String) -> String {
    return format!("inputs/{}.txt", name);
}

pub fn get_output_path(input_path: &String, part: i8) -> String {
    if input_path.contains(".") {
        let [tail, head]: [&str; 2] = input_path
            .rsplitn(2, ".")
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        return format!("{}.solution{}.{}", head, part, tail);
    } else {
        return format!("{}.solution{}", input_path, part);
    }
}

pub fn run_day<T1: ToString, T2: ToString>(
    filename: &String,
    part1: Runnable<T1>,
    part2: Runnable<T2>,
) -> Result<(RunnableRun, RunnableRun), String> {
    return match fs::read_to_string(filename) {
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
        Err(err) => Err(format!(
            "Unable to read input file '{}': {}.",
            filename, err
        )),
    };
}

pub fn run<T1: ToString, T2: ToString>(part1: Runnable<T1>, part2: Runnable<T2>) {
    let args: Vec<String> = env::args().collect();

    let name = args[0]
        .split("/")
        .last()
        .expect("Unable to determine binary name.");

    let filenames: Vec<String> = if args.len() > 1 {
        args.iter().skip(1).cloned().collect()
    } else {
        vec![get_input_path(name.to_string())]
    };

    for filename in &filenames {
        println!(
            "Running {} using input {}...",
            Cyan.paint(name),
            Cyan.paint(filename)
        );
        let (run1, run2) = run_day(filename, part1, part2).unwrap();
        print_runnable_run("Part 1".to_string(), run1, &THRESHOLDS_DEFAULT);
        print_runnable_run("Part 2".to_string(), run2, &THRESHOLDS_DEFAULT);
    }
}

pub fn missing<T: ToString>(_data: String) -> T {
    panic!("Should never actually be called.");
}
