use ansi_term::Colour::*;
use std::time::Duration;

use aoc::runner::*;
use aoc_derive::RunnableListProvider;

#[derive(RunnableListProvider)]
pub struct ListProvider {}

fn main() {
    let runnables = ListProvider::get();
    let mut durations: Vec<Duration> = Vec::new();
    println!(
        "Running {} days using default inputs...",
        Cyan.paint(runnables.len().to_string())
    );
    for (name, part1, part2) in ListProvider::get() {
        let filename = get_input_path(name.to_string());
        let name = name.replace("day", "Day ");
        match run_day(&filename, part1, part2) {
            Ok((run1, run2)) => {
                for (i, run) in [(1, run1), (2, run2)] {
                    if run.is_ok() {
                        durations.push(run.clone().unwrap().duration);
                    }
                    print_runnable_run(format!("{} part {}", name, i).to_string(), run);
                }
            }
            Err(err) => {
                println!("> {} failed: {}", Purple.paint(name), Red.paint(err));
            }
        }
    }
    if !durations.is_empty() {
        println!(
            "Ran {} parts in {}, averaging {} per part.",
            Cyan.paint(durations.len().to_string()),
            Purple.paint(format!("{:?}", durations.iter().sum::<Duration>())),
            Purple.paint(format!(
                "{:?}",
                durations.iter().sum::<Duration>() / durations.len() as u32
            )),
        );
    }
}
