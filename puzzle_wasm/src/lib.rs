//! A WebAssembly package bundling all puzzles for use in a browser.

use std::time::Duration;

use aoc::bins::BINS;
use puzzle_runner::{derived::Solver, runner::Timer};
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen_rayon::init_thread_pool;

mod r#extern;
use r#extern::{performance, Number};

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

    /// Convert a [`Duration`] to a [`JsValue`] as nanoseconds.
    ///
    /// bindgen doesn't support u128, so we convert it to a string and and then tell TS that it's a number. JS will end up coercing it into a number when it is used as one in most cases anyway, so this'll work out fine. Probably.
    pub(super) fn duration_to_js(duration: &Duration) -> Number {
        JsValue::from(duration.as_nanos().to_string()).unchecked_into()
    }
}

/// Timer based on [`web_sys::Performance`].
pub struct PerformanceTimer(f64);
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

/// Test the minimum resolution of timers in the current environment.
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

/// WASM wrapper for [`puzzle_runner::derived::Bin`].
#[wasm_bindgen]
pub struct Bin(&'static puzzle_runner::derived::Bin);
#[wasm_bindgen]
impl Bin {
    /// The name of the binary.
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.0.name.to_owned()
    }

    /// The title of the puzzle.
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> Option<String> {
        self.0.title.map(str::to_owned)
    }

    /// The path of the source file, relative to the root of the repository.
    #[wasm_bindgen(getter)]
    pub fn source_path(&self) -> String {
        self.0.source_path.to_owned()
    }

    /// The year that the binary is for (last 2 digits only).
    #[wasm_bindgen(getter)]
    pub fn year(&self) -> u8 {
        self.0.year
    }

    /// The day that the binary is for.
    #[wasm_bindgen(getter)]
    pub fn day(&self) -> u8 {
        self.0.day
    }

    // How many parts are implemented in this binary.
    #[wasm_bindgen(getter)]
    pub fn parts(&self) -> u8 {
        u8::from(self.0.part1.is_implemented()) + u8::from(self.0.part2.is_implemented())
    }

    /// The examples
    #[wasm_bindgen(getter)]
    pub fn examples(&self) -> Vec<Example> {
        self.0.examples.iter().map(Example).collect()
    }
}

/// WASM wrapper for [`puzzle_runner::derived::Example`].
#[wasm_bindgen]
pub struct Example(&'static puzzle_runner::derived::Example);
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
        self.0.part1.map(str::to_owned)
    }

    /// The expected result of part 2, cast to a string.
    #[wasm_bindgen(getter)]
    pub fn part2(&self) -> Option<String> {
        self.0.part2.map(str::to_owned)
    }
}

/// WASM wrapper for [`puzzle_runner::runner::SolverResult::Success`].
#[wasm_bindgen]
pub struct SolverResult {
    result: String,
    duration: Duration,
}
#[wasm_bindgen]
impl SolverResult {
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
impl TryFrom<puzzle_runner::runner::SolverResult> for SolverResult {
    type Error = String;

    fn try_from(value: puzzle_runner::runner::SolverResult) -> Result<Self, Self::Error> {
        match value {
            puzzle_runner::runner::SolverResult::Success {
                result, duration, ..
            } => Ok(SolverResult { result, duration }),
            puzzle_runner::runner::SolverResult::Error(err) => Err(err),
        }
    }
}

/// Get a list of all implemented [`Bin`]s.
#[wasm_bindgen]
pub fn list() -> Vec<Bin> {
    BINS.iter().map(Bin).collect()
}

/// Run a single part of a single [`Bin`].
///
/// This is just a wrapper around [`Solver::run`], mostly present because [`Solver`] doesn't
/// translate into a web-compatible format easily.
///
/// # Errors
///
/// Will return `Err` if there is no `Bin` wih the requested name, if the requested part is not
/// implemented, or if running the part causes a panic.
#[wasm_bindgen]
pub fn run(name: &str, part: u8, input: &str) -> Result<SolverResult, String> {
    let bin = BINS
        .iter()
        .find(|d| d.name == name)
        .ok_or(format!("Cannot find implementation for {name}."))?;
    let solver: Solver<String> = match part {
        1 => bin.part1.clone(),
        2 => bin.part2.clone(),
        _ => return Err(format!("Invalid part {part}.")),
    };

    std::panic::catch_unwind(move || solver.run::<PerformanceTimer>(input, None).try_into())
        .map_err(|_| "solution panicked".to_string())?
}

#[doc(hidden)]
pub fn main() {
    #[allow(unexpected_cfgs)]
    #[cfg(feature = "debug")]
    console_error_panic_hook::set_once();
}
