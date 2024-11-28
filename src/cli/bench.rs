#![cfg(feature = "bench")]

use clap::{builder::ArgPredicate, value_parser, Parser};
use criterion::Criterion;

use super::{multi::TargetArgs, runner::Solver};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct BenchArgs {
    #[command(flatten)]
    targets: TargetArgs,

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

    /// Set the number of samples to collect.
    #[arg(long, default_value = "100", value_parser = value_parser![u64].range(10..))]
    samples: u64,
}

pub fn main() {
    assert!(
        !cfg!(feature = "visual"),
        "this entrypoint doesn't support feature 'visual'."
    );

    let args = BenchArgs::parse();

    let mut criterion = Criterion::default();
    if let Some(name) = args.save_baseline {
        criterion = criterion.save_baseline(name);
    } else if let Some(name) = args.baseline {
        criterion = criterion.retain_baseline(name, true);
    }
    criterion = criterion.sample_size(args.samples as usize);

    let days = args.targets.filtered_days();
    for target in args.targets.get_targets(&days) {
        let Solver::Implemented(runnable) = target.solver else {
            continue;
        };

        let mut name = format!("{}/part{}", target.day, target.part);
        if let Some(source) = target.source_name {
            name = format!("{name}/{source}");
        }

        let input = target.input.read().unwrap();

        criterion.bench_function(&name, |b| {
            b.iter(|| runnable(&input));
        });
    }

    criterion.final_summary();
}
