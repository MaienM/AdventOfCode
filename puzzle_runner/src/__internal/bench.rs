//! The benchmarking CLI features.
#![cfg(feature = "bench")]

use std::{
    env,
    fs::{File, create_dir_all},
    io::Write,
    path::Path,
    time::Duration,
};

use clap::{Parser, value_parser};
use criterion::{Criterion, profiler::Profiler as CProfiler};
use pprof::{ProfilerGuard, protos::Message};

use crate::{
    __internal::single::{SingleArgs, SingleRunner, run_single},
    derived::{Chapter, Part, Series},
    source::ChapterSources,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(super) struct BenchArgs {
    #[command(flatten)]
    pub(super) single: SingleArgs,

    /// For compatibility with criterion CLI.
    #[arg(long, num_args = 0, hide = true)]
    bench: (),

    /// For compatibility with criterion CLI.
    #[arg(long, num_args = 1, hide = true)]
    profile_time: Option<f64>,

    /// Save results under a named baseline.
    #[arg(short = 's', long)]
    save_baseline: Option<String>,

    /// Set the number of samples to collect.
    #[arg(long, default_value = "100", value_parser = value_parser![u64].range(10..))]
    samples: u64,

    /// Save profile results with this name.
    #[arg(long)]
    profile_name: Option<String>,
}
impl BenchArgs {
    pub(super) fn build_criterion(&self) -> Criterion {
        let mut criterion = Criterion::default();
        criterion = criterion.sample_size(self.samples as usize);

        if let Some(name) = &self.save_baseline {
            criterion = criterion.save_baseline(name.clone());
        }

        if let Some(time) = self.profile_time {
            criterion = criterion
                .with_profiler(Profiler {
                    name: self.profile_name.clone().unwrap_or("default".to_owned()),
                    ..Profiler::default()
                })
                .profile_time(Some(Duration::from_secs_f64(time)));
        }

        criterion
    }
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

pub(super) struct BenchRunner(String, Criterion);
impl SingleRunner for BenchRunner {
    type Args = BenchArgs;

    fn get_sources_arg(args: &Self::Args) -> &String {
        &args.single.folder
    }

    fn setup(args: &Self::Args, series: &Series, chapter: &Chapter) -> Self {
        BenchRunner(
            format!("{}/{}", series.name, chapter.name),
            args.build_criterion(),
        )
    }

    fn print_header(&self, description: String) {
        println!("Benchmarking {description}...");
        println!();
    }

    fn run(&mut self, part: &Part, input: &str, _sources: &ChapterSources) {
        let name = format!("{}/{}", self.0, part.num);
        self.1.bench_function(&name, |b| {
            b.iter(|| (part.implementation)(input));
        });
    }

    fn finish(&mut self) {
        self.1.final_summary();
    }
}

#[doc(hidden)]
pub fn main(series: &Series, chapter: &Chapter) {
    // Benchmarks are ran with the directory set to the crate root instead of repository root. This
    // breaks loading inputs (which uses relative paths by default), so we change back to the
    // repository root here.
    env::set_current_dir(env::current_dir().unwrap().parent().unwrap()).unwrap();

    run_single::<BenchRunner>(series, chapter);
}
