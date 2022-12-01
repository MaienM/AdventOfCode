use std::time::Duration;

use ansi_term::Colour::*;
use aoc::runner::*;
use aoc_derive::RunnableListProvider;

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
        let filename = get_input_path(name.to_string());
        let name = name.replace("day", "Day ");
        match run_day(&filename, part1, part2) {
            Ok((run1, run2)) => {
                for (i, run) in [(1, run1), (2, run2)] {
                    runs.push((format!("{} part {}", name, i).to_string(), Ok(run)));
                }
            }
            Err(err) => {
                runs.push((name, Err(err)));
            }
        }
    }

    let successes = runs
        .iter()
        .map(|(_, r)| r.clone())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect::<Vec<RunnableRunOk>>();
    let duration_total = successes.iter().map(|r| r.duration).sum::<Duration>();
    let duration_avg = if !successes.is_empty() {
        duration_total / successes.len() as u32
    } else {
        Duration::from_secs(0)
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
            Purple.paint(format!("{:?}", duration_total)),
            Purple.paint(format!("{:?}", duration_avg,)),
        );
    }
}
