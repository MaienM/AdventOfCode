use ansi_term::Colour::*;
use std::env;
use std::fs;
use std::time::Duration;
use std::time::Instant;

type Runnable<T> = fn(String) -> T;

// Used with a proc-macro to provide a listing of all Runnables of all days.
pub type RunnableList = Vec<(&'static str, Runnable<String>, Runnable<String>)>;
pub trait RunnableListProvider {
    fn get() -> RunnableList;
}

#[derive(Clone)]
pub struct RunnableRunOk {
    pub result: String,
    pub duration: Duration,
}

type RunnableRun = Result<RunnableRunOk, String>;

pub fn print_runnable_run(name: String, run: RunnableRun) {
    let name = Purple.paint(name);
    match run {
        Err(err) => {
            println!("> {}: {}", name, Red.paint(err));
        }
        Ok(run) => {
            let duration_colour = if run.duration.as_secs() > 0 {
                Red
            } else if run.duration.as_millis() > 0 {
                Blue
            } else {
                Green
            };
            let duration_formatted = duration_colour.paint(format!("{:?}", run.duration));

            if run.result.contains("\n") {
                println!("> {}: [{}]", name, duration_formatted);
                for line in run.result.split("\n") {
                    println!("  {}", line);
                }
            } else {
                println!("> {}: {} [{}]", name, run.result, duration_formatted);
            }
        }
    }
}

fn run_runnable<T: ToString>(runnable: Runnable<T>, input: &String) -> RunnableRun {
    if runnable == missing {
        return Err("Not implemented.".to_string());
    }

    let start = Instant::now();
    let result = runnable(input.to_owned());
    let duration = start.elapsed();

    let result = result.to_string();

    return Ok(RunnableRunOk { result, duration });
}

pub fn get_input_path(name: String) -> String {
    return format!("inputs/{}.txt", name);
}

pub fn run_day<T1: ToString, T2: ToString>(
    filename: &String,
    part1: Runnable<T1>,
    part2: Runnable<T2>,
) -> Result<(RunnableRun, RunnableRun), String> {
    return match fs::read_to_string(filename) {
        Ok(input) => Ok((run_runnable(part1, &input), run_runnable(part2, &input))),
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

    let filename = get_input_path(name.to_string());
    let filename = args.get(1).unwrap_or(&filename);

    println!(
        "Running {} using input {}...",
        Cyan.paint(name),
        Cyan.paint(filename)
    );
    let (run1, run2) = run_day(filename, part1, part2).unwrap();
    print_runnable_run("Part 1".to_string(), run1);
    print_runnable_run("Part 2".to_string(), run2);
}

pub fn missing<T: ToString>(_data: String) -> T {
    panic!("Should never actually be called.");
}
