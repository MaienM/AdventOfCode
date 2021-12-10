use std::env;
use std::fs;
use std::time::Instant;

type Runnable = fn(String) -> i32;

fn do_runnable(name: &'static str, runnable: Runnable, input: &String) {
    if runnable == missing {
        println!("> {} is not implemented.", name);
        return;
    }

    let start = Instant::now();
    let result = runnable(input.to_owned());
    let duration = start.elapsed();
    println!("> {}: {} [{:?}]", name, result, duration);
}

pub fn run(part1: Runnable, part2: Runnable) {
    let args: Vec<String> = env::args().collect();

    let name = args[0]
        .split("/")
        .last()
        .expect("Unable to determine binary name.");

    let filename = format!("inputs/{}.txt", name);
    let filename = args.get(1).unwrap_or(&filename);

    println!("Running {} with input {}...", name, filename);

    let input = fs::read_to_string(filename).expect("Unable to read input file.");
    do_runnable("Part 1", part1, &input);
    do_runnable("Part 2", part2, &input);
}

pub fn missing(_data: String) -> i32 {
    return -1;
}
