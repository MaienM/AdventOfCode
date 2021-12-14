use ansi_term::Colour;
use ansi_term::Colour::*;
use std::env;
use std::fs;
use std::time::Instant;

type Runnable<T> = fn(String) -> T;

fn do_runnable<T: ToString>(name: &'static str, runnable: Runnable<T>, input: &String) {
    if runnable == missing {
        println!("> {} is not implemented.", name);
        return;
    }

    let start = Instant::now();
    let result = runnable(input.to_owned());
    let duration = start.elapsed();

    let duration_colour: Colour = if duration.as_secs() > 0 {
        Red
    } else if duration.as_millis() > 0 {
        Blue
    } else {
        Green
    };
    let duration_formatted = duration_colour.paint(format!("{:?}", duration));

    let name = Purple.paint(name);
    let result = result.to_string();
    if result.contains("\n") {
        println!("> {}: [{}]", name, duration_formatted);
        for line in result.split("\n") {
            println!("  {}", line);
        }
    } else {
        println!("> {}: {} [{}]", name, result, duration_formatted);
    }
}

pub fn run<T1: ToString, T2: ToString>(part1: Runnable<T1>, part2: Runnable<T2>) {
    let args: Vec<String> = env::args().collect();

    let name = args[0]
        .split("/")
        .last()
        .expect("Unable to determine binary name.");

    let filename = format!("inputs/{}.txt", name);
    let filename = args.get(1).unwrap_or(&filename);

    println!(
        "Running {} with input '{}'...",
        Cyan.paint(name),
        Cyan.paint(filename)
    );

    let input = fs::read_to_string(filename).expect("Unable to read input file.");
    do_runnable("Part 1", part1, &input);
    do_runnable("Part 2", part2, &input);
}

pub fn missing<T: ToString>(_data: String) -> T {
    panic!("Should never actually be called.");
}
