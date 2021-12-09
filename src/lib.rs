use std::env;
use std::fs;

type Run = fn(String) -> String;

pub fn dorun(run: Run) {
    let args: Vec<String> = env::args().collect();

    let name = args[0]
        .split("/")
        .last()
        .expect("Unable to determine binary name.");
    let day = name.split("-").next().expect("Unable to get day name.");
    let filename = format!("inputs/{}.txt", day);
    let filename = args.get(1).unwrap_or(&filename);

    println!("Running {} with input {}...", name, filename);

    let input = fs::read_to_string(filename).expect("Unable to read input file.");

    let result = run(input);
    println!("Result: {}", result);
}

