use std::time::Duration;

use ansi_term::Colour::{Cyan, Purple, Red};
use aoc::runner::{
    get_input_path, print_runnable_run, run_day, DurationThresholds, Runnable, RunnableRun,
    RunnableRunOk,
};
use aoc_derive::RunnableListProvider;

type RunnableList = Vec<(
    &'static str,
    Runnable<String, fn(&str) -> String>,
    Runnable<String, fn(&str) -> String>,
)>;
pub trait RunnableListProvider {
    fn get() -> RunnableList;
}

#[derive(RunnableListProvider)]
pub struct ListProvider {}

fn main() {
    let runnables = ListProvider::get();
    let mut runs: Vec<(String, Result<RunnableRun, String>)> = Vec::new();
    println!(
        "Running {} days using default inputs...",
        Cyan.paint(runnables.len().to_string())
    );
    for (name, part1, part2) in ListProvider::get() {
        let filename = get_input_path(name);
        let name = name.replace("day", "Day ");
        match run_day(&filename, &part1, &part2) {
            Ok((run_1, run_2)) => {
                for (i, run) in [(1, run_1), (2, run_2)] {
                    runs.push((format!("{name} part {i}").to_string(), Ok(run)));
                }
            }
            Err(err) => {
                runs.push((name, Err(err)));
            }
        }
    }

    let successes = runs
        .iter()
        .filter_map(|(_, r)| r.clone().ok())
        .filter_map(Result::ok)
        .collect::<Vec<RunnableRunOk>>();
    let duration_total = successes.iter().map(|r| r.duration).sum::<Duration>();
    let duration_avg = if successes.is_empty() {
        Duration::from_secs(0)
    } else {
        duration_total / successes.len() as u32
    };

    let thresholds = DurationThresholds {
        good: duration_avg / 3,
        acceptable: duration_avg * 2 / 3,
    };
    for (name, result) in runs {
        match result {
            Ok(run) => {
                print_runnable_run(name, run, &thresholds, false);
            }
            Err(err) => {
                println!("> {} failed: {}", Purple.paint(name), Red.paint(err));
            }
        }
    }
    if !successes.is_empty() {
        println!(
            "Ran {} parts in {}, averaging {} per part.",
            Cyan.paint(successes.len().to_string()),
            Purple.paint(format!("{duration_total:?}")),
            Purple.paint(format!("{duration_avg:?}",)),
        );
    }
}
