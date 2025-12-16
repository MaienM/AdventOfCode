//! A WebAssembly package bundling all puzzles for use in a browser.

use std::{collections::HashMap, sync::LazyLock, time::Duration};

use puzzle_runner::{
    derived::{Chapter, Part, Series},
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
    let series = [&*aoc::SERIES];

    let mut map = HashMap::new();
    for series in series {
        map.insert(series.name, series);
    }
    map
});

fn get_series(name: &str) -> Result<&&Series, String> {
    SERIES
        .get(name)
        .ok_or_else(|| format!("Cannot find series {name}."))
}

fn get_chapter<'a>(series: &'a Series, name: &str) -> Result<&'a Chapter, String> {
    series
        .chapters
        .iter()
        .find(|c| c.name == name)
        .ok_or_else(|| format!("Cannot find chapter {name} in series {}", series.name))
}

fn get_part<'a>(series: &Series, chapter: &'a Chapter, num: u8) -> Result<&'a Part, String> {
    chapter.parts.iter().find(|p| p.num == num).ok_or_else(|| {
        format!(
            "Cannot find part {num} in chapter {} in series {}",
            chapter.name, series.name
        )
    })
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
/// Will return `Err` if the [`Series`], [`Chapter`] or [`Part`] cannot be found, or if running it
/// causes a panic.
#[wasm_bindgen]
pub fn run(
    series: &str,
    chapter: &str,
    part: u8,
    input: &str,
    solution: Option<String>,
) -> PartResult {
    let series = get_series(series)?;
    let chapter = get_chapter(series, chapter)?;
    let part = get_part(series, chapter, part)?;
    part.run::<PerformanceTimer>(input, solution)
}

/// Get the URL for a chapter.
///
/// This is just a wrapper around
/// [`Controller::chapter_url`](puzzle_runner::controller::Controller::chapter_url).
///
/// # Errors
///
/// Will return `Err` if the [`Series`] cannot be found, or if the chapter name is in a form that
/// is not valid for the series.
#[wasm_bindgen]
pub fn chapter_url(series: &str, chapter: &str) -> Result<String, String> {
    let series = get_series(series)?;
    Ok(series.controller.chapter_url(chapter)?)
}

/// Setup the panic handler.
#[wasm_bindgen]
pub fn init_panic_handler() {
    console_error_panic_hook::set_once();
}
