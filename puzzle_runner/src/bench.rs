//! The benchmarking CLI features.

#![cfg(feature = "bench")]

use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::Path,
    time::Duration,
};

use clap::{Parser, builder::ArgPredicate, value_parser};
use criterion::{Criterion, profiler::Profiler as CProfiler};
use pprof::{ProfilerGuard, protos::Message};

use super::{derived::Solver, multi::TargetArgs};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct BenchArgs {
    #[command(flatten)]
    targets: TargetArgs,

    /// For compatibility with criterion CLI.
    #[arg(long, num_args = 0, hide = true)]
    bench: (),

    /// For compatibility with criterion CLI.
    #[arg(long, num_args = 1, hide = true)]
    profile_time: Option<f64>,

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

    /// Save profile results with this name.
    #[arg(long)]
    profile_name: Option<String>,
}

#[derive(Default)]
struct Profiler<'a> {
    name: String,
    profiler: Option<ProfilerGuard<'a>>,
}
impl CProfiler for Profiler<'_> {
    fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {
        self.profiler = Some(ProfilerGuard::new(100).unwrap());
    }

    fn stop_profiling(&mut self, _benchmark_id: &str, benchmark_dir: &Path) {
        create_dir_all(benchmark_dir).unwrap();

        let path = benchmark_dir.join(&self.name).with_extension("pb");
        let mut file = File::create(path).unwrap();
        let mut content = Vec::new();
        self.profiler
            .take()
            .unwrap()
            .report()
            .build()
            .unwrap()
            .pprof()
            .unwrap()
            .write_to_vec(&mut content)
            .unwrap();
        file.write_all(&content).unwrap();
    }
}

pub fn main() {
    let args = BenchArgs::parse();

    let mut criterion = Criterion::default();
    criterion = criterion.sample_size(args.samples as usize);

    if let Some(name) = args.save_baseline {
        criterion = criterion.save_baseline(name);
    } else if let Some(name) = args.baseline {
        criterion = criterion.retain_baseline(name, true);
    }

    if let Some(time) = args.profile_time {
        criterion = criterion
            .with_profiler(Profiler {
                name: args.profile_name.unwrap_or("default".to_owned()),
                ..Profiler::default()
            })
            .profile_time(Some(Duration::from_secs_f64(time)));
    }

    let bins = args.targets.filtered_binaries();
    for target in args.targets.get_targets(&bins) {
        let Solver::Implemented(runnable) = target.solver else {
            continue;
        };

        let mut name = format!("{}/part{}", target.bin, target.part);
        if let Some(source) = target.source_name {
            name = format!("{name}/{source}");
        }

        // For some reason this entrypoint is run from inside the crate dir instead of from the
        // root (like the others are), so we need to adjust for that.
        let input = target
            .input
            .mutate_path(|p| Path::new("..").join(p).to_str().unwrap().to_owned())
            .read()
            .unwrap();

        criterion.bench_function(&name, |b| {
            b.iter(|| runnable(&input));
        });
    }

    criterion.final_summary();
}
