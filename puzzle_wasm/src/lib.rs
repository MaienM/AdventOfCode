//! A WebAssembly package bundling all puzzles for use in a browser.

use std::{collections::HashMap, sync::LazyLock, time::Duration};

use puzzle_runner::{
    derived::Series,
    runner::{PartResult, Timer},
};
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen_rayon::init_thread_pool;

mod r#extern;
use r#extern::performance;
use web_sys::Performance;

use crate::r#extern::Number;

static SERIES: LazyLock<HashMap<&'static str, &Series>> = LazyLock::new(|| {
    let series = [&*aoc::generated::SERIES];

    let mut map = HashMap::new();
    for series in series {
        map.insert(series.name, series);
    }
    map
});

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
        Self(performance.with(Performance::now))
    }

    #[inline]
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn elapsed(&self) -> Duration {
        let end = performance.with(Performance::now);
        time::elapsed_to_duration(end - self.0)
    }
}

/// Test the minimum resolution of timers in the current environment.
///
/// This will block for the length of one resolution, the worst I've seen is `16.66ms` (1/60th of a second).
#[wasm_bindgen]
pub fn get_timer_resolution() -> Number {
    let start = performance.with(Performance::now);
    let mut end = start;
    #[allow(clippy::float_cmp)]
    while start == end {
        end = performance.with(Performance::now);
    }
    let duration = time::elapsed_to_duration(end - start);
    time::duration_to_js(&duration)
}

/// Get all implemented [`Series`]s.
#[wasm_bindgen(unchecked_return_type = "Map<string, Series>")]
pub fn all() -> JsValue {
    serde_wasm_bindgen::to_value(&*SERIES).unwrap()
}

/// Run a single [`Part`](puzzle_runner::derived::Part).
///
/// This is just a wrapper around [`Part::run`](puzzle_runner::derived::Part::run).
///
/// # Errors
///
/// Will return `Err` if the [`Series`], [`Chapter`](puzzle_runner::derived::Chapter) or
/// [`Part`](puzzle_runner::derived::Part) cannot be found, or if running it causes a panic.
#[wasm_bindgen]
pub fn run(
    series: &str,
    chapter: &str,
    part: u8,
    input: &str,
    solution: Option<String>,
) -> PartResult {
    let part = if let Some(series) = SERIES.get(series)
        && let Some(chapter) = series.chapters.iter().find(|c| c.name == chapter)
        && let Some(part) = chapter.parts.iter().find(|p| p.num == part)
    {
        part
    } else {
        return Err(format!(
            "Cannot find implementation for {series}/{chapter}/part{part}."
        ));
    };
    part.run::<PerformanceTimer>(input, solution)
}

/// Setup the panic handler.
#[wasm_bindgen]
pub fn init_panic_handler() {
    console_error_panic_hook::set_once();
}
