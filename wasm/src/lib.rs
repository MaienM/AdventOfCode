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
    #[no_mangle]
    static performance: Performance;

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
struct PerformanceTimer(f64);
impl Timer for PerformanceTimer {
    #[inline]
    fn start() -> Self {
        Self(performance.now())
    }

    #[inline]
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn elapsed(&self) -> Duration {
        let end = performance.now();
        time::elapsed_to_duration(end - self.0)
    }
}

/// Test the minimum resolution of [`web_sys::Performance`].
///
/// This will block for the length of one resolution, the worst I've seen is `16.66ms` (1/60th of a second).
#[wasm_bindgen]
pub fn get_timer_resolution() -> Number {
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

/// Run a single solution.
#[wasm_bindgen]
pub fn run(day: u8, part: u8, input: &str) -> Result<SolverRunResult, String> {
    let day = DAYS
        .iter()
        .find(|d| d.num == day)
        .ok_or(format!("Cannot find implementation for day {day}."))?;
    let solver: Solver<String> = match part {
        1 => day.part1,
        2 => day.part2,
        _ => return Err(format!("Invalid part {part}.")),
    }
    .into();

    std::panic::catch_unwind(move || {
        solver
            .run_with_timer::<PerformanceTimer>(input, None)
            .try_into()
    })
    .map_err(|_| "solution panicked".to_string())?
}

pub fn main() {
    #[cfg(feature = "debug")]
    console_error_panic_hook::set_once();
}
