use std::time::Duration;

use aoc::{
    cli::runner::{Solver, Timer},
    DAYS,
};
use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;
use web_sys::Performance;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = number)]
    pub type Number;
}

mod time {
    use std::time::Duration;

    use wasm_bindgen::prelude::*;

    use super::Number;

    /// Convert of a difference between [`web_sys::Performance::now`] results into a [`std::time::Duration`].
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub(super) fn elapsed_to_duration(elapsed: f64) -> Duration {
        // The result from performance.now is in milliseconds.
        let mut duration = Duration::from_secs((elapsed / 1000f64) as u64);
        duration += Duration::from_nanos((elapsed * 1_000_000f64).round() as u64 % 1_000_000_000);
        duration
    }

    /// Convert a [`std::time::Duration`] to a [`wasm_bindgen::JsValue`] as nanoseconds.
    ///
    /// bindgen doesn't support u128, so we convert it to a string and and then tell TS that it's a number. JS will end up coercing it into a number when it is used as one in most cases anyway, so this'll work out fine. Probably.
    pub(super) fn duration_to_js(duration: &Duration) -> Number {
        JsValue::from(duration.as_nanos().to_string()).unchecked_into()
    }
}

/// Timer based on [`web_sys::Performance`].
struct PerformanceTimer(Performance, f64);
impl Timer for PerformanceTimer {
    #[inline]
    fn start() -> Self {
        let performance = web_sys::window().unwrap().performance().unwrap();
        let start = performance.now();
        Self(performance, start)
    }

    #[inline]
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn elapsed(&self) -> Duration {
        let end = self.0.now();
        time::elapsed_to_duration(end - self.1)
    }
}

/// Test the minimum resolution of [`web_sys::Performance`].
///
/// This will block for the length of one resolution, the worst I've seen is `16.66ms` (1/60th of a second).
#[wasm_bindgen]
pub fn get_timer_resolution() -> Number {
    let performance = web_sys::window().unwrap().performance().unwrap();
    let start = performance.now();
    let mut end = start;
    #[allow(clippy::float_cmp)]
    while start == end {
        end = performance.now();
    }
    let duration = time::elapsed_to_duration(end - start);
    time::duration_to_js(&duration)
}

/// WASM wrapper for [`aoc::derived::Day`].
#[wasm_bindgen]
pub struct Day(&'static aoc::derived::Day);
#[wasm_bindgen]
impl Day {
    /// The number of the day.
    #[wasm_bindgen(getter)]
    pub fn num(&self) -> u8 {
        self.0.num
    }

    /// Whether part 1 has been implemented for this day.
    #[wasm_bindgen(getter)]
    pub fn has_part1(&self) -> bool {
        self.0.part1.is_some()
    }

    /// Whether part 2 has been implemented for this day.
    #[wasm_bindgen(getter)]
    pub fn has_part2(&self) -> bool {
        self.0.part2.is_some()
    }

    fn run_part(part: Solver<String>, input: &str) -> Result<SolverRunResult, String> {
        match std::panic::catch_unwind(move || {
            let result: Result<SolverRunResult, String> = part
                .run_with_timer::<PerformanceTimer>(input, None)
                .try_into();
            result
        }) {
            Ok(value) => value,
            Err(_) => Err("solution panicked".to_string()),
        }
    }

    /// Run part 1.
    pub fn part1(&self, input: &str) -> Result<SolverRunResult, String> {
        Day::run_part(self.0.part1.into(), input)
    }

    /// Run part 2.
    pub fn part2(&self, input: &str) -> Result<SolverRunResult, String> {
        Day::run_part(self.0.part2.into(), input)
    }

    /// The examples
    #[wasm_bindgen(getter)]
    pub fn examples(&self) -> Vec<Example> {
        self.0.examples.iter().map(Example).collect()
    }
}

/// WASM wrapper for [`aoc::derived::Example`].
#[wasm_bindgen]
pub struct Example(&'static aoc::derived::Example);
#[wasm_bindgen]
impl Example {
    /// The name of the example.
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.0.name.to_owned()
    }

    /// The example input.
    #[wasm_bindgen(getter)]
    pub fn input(&self) -> String {
        self.0.input.to_owned()
    }

    /// The expected result of part 1, cast to a string.
    #[wasm_bindgen(getter)]
    pub fn part1(&self) -> Option<String> {
        self.0.part1.map(ToOwned::to_owned)
    }

    /// The expected result of part 2, cast to a string.
    #[wasm_bindgen(getter)]
    pub fn part2(&self) -> Option<String> {
        self.0.part2.map(ToOwned::to_owned)
    }
}

/// WASM wrapper for [`aoc::cli::runner::SolverRunResult::Success`].
#[wasm_bindgen]
pub struct SolverRunResult {
    result: String,
    duration: Duration,
}
#[wasm_bindgen]
impl SolverRunResult {
    /// The result of the solver, converted to a string.
    #[wasm_bindgen(getter)]
    pub fn result(&self) -> String {
        self.result.clone()
    }

    /// The duration of the solver run, in nanoseconds.
    #[wasm_bindgen(getter)]
    pub fn duration(&self) -> Number {
        time::duration_to_js(&self.duration)
    }
}
impl TryFrom<aoc::cli::runner::SolverRunResult> for SolverRunResult {
    type Error = String;

    fn try_from(value: aoc::cli::runner::SolverRunResult) -> Result<Self, Self::Error> {
        match value {
            aoc::cli::runner::SolverRunResult::Success {
                result, duration, ..
            } => Ok(SolverRunResult { result, duration }),
            aoc::cli::runner::SolverRunResult::Error(err) => Err(err),
        }
    }
}

/// Get list of all days.
#[wasm_bindgen]
pub fn list() -> Vec<Day> {
    DAYS.iter().map(Day).collect()
}

pub fn main() {
    #[cfg(feature = "debug")]
    console_error_panic_hook::set_once();
}
