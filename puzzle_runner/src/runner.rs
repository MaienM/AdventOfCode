//! Runs solutions while collecting their results & runtimes.

use std::{
    sync::LazyLock,
    time::{Duration, Instant},
};

use ansi_term::{
    ANSIStrings,
    Colour::{Blue, Green, Purple, Red},
    unstyle,
};

use crate::derived::Part;

/// Trait to track elapsed time.
pub trait Timer {
    /// Start a new timer.
    fn start() -> Self;
    /// Get the time elapsed since the timer was [`start`](Timer::start)ed.
    fn elapsed(&self) -> Duration;
}

/// Timer based on [`std::time::Instant`].
pub struct InstantTimer(Instant);
impl Timer for InstantTimer {
    #[inline]
    fn start() -> Self {
        Self(Instant::now())
    }

    #[inline]
    fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }
}

static SYMBOL_UNKNOWN: LazyLock<String> = LazyLock::new(|| "?".to_owned());
static SYMBOL_OK: LazyLock<String> = LazyLock::new(|| Green.paint("✔").to_string());
static SYMBOL_INCORRECT: LazyLock<String> = LazyLock::new(|| Red.paint("✘").to_string());
static SYMBOL_ERROR: LazyLock<String> = LazyLock::new(|| Red.paint("⚠").to_string());

/// The result of successfully running a [`Part`].
#[derive(Clone)]
#[cfg_attr(
    feature = "wasm",
    derive(serde::Serialize, tsify::Tsify),
    tsify(into_wasm_abi, missing_as_null)
)]
pub struct RunResults {
    /// The result of the part, converted to a string.
    pub result: String,
    /// The expected result of the part, if known.
    pub solution: Option<String>,
    /// The duration of the part run.
    pub duration: Duration,
}

/// The result of running a [`Part`].
pub type PartResult = Result<RunResults, String>;

pub trait PrintPartResult {
    fn print(&self, name: &str, thresholds: &DurationThresholds, show_result: bool);
}
impl PrintPartResult for PartResult {
    fn print(&self, name: &str, thresholds: &DurationThresholds, show_result: bool) {
        let name = Purple.paint(name);
        match self {
            Ok(RunResults {
                result,
                solution,
                duration,
            }) => {
                let duration_colour = if duration < &thresholds.good {
                    Green
                } else if duration < &thresholds.acceptable {
                    Blue
                } else {
                    Red
                };
                let duration_formatted = duration_colour.paint(format!("{duration:?}"));

                if !show_result {
                    let (symbol, name) = match solution {
                        None => (SYMBOL_UNKNOWN.clone().clone(), name),
                        Some(s) => {
                            if s == result {
                                (SYMBOL_OK.clone().clone(), name)
                            } else {
                                (
                                    SYMBOL_INCORRECT.clone().clone(),
                                    Red.paint(unstyle(&ANSIStrings(&[name]))),
                                )
                            }
                        }
                    };
                    println!("{symbol} {name} [{duration_formatted}]");
                    return;
                }

                let (symbol, result) = match solution {
                    Some(expected) => {
                        if result == expected {
                            (SYMBOL_OK.clone().clone(), Green.paint(result).to_string())
                        } else if result.contains('\n') || expected.contains('\n') {
                            (
                                SYMBOL_INCORRECT.clone().clone(),
                                format!("{}\nShould be:\n{}", Red.paint(result), expected),
                            )
                        } else {
                            (
                                SYMBOL_INCORRECT.clone().clone(),
                                format!("{} (should be {})", Red.paint(result), expected),
                            )
                        }
                    }
                    None => (SYMBOL_UNKNOWN.clone().clone(), result.clone()),
                };

                if result.contains('\n') {
                    println!("{symbol} {name}: [{duration_formatted}]");
                    for line in result.split('\n') {
                        println!("  {line}");
                    }
                } else {
                    println!("{symbol} {name}: {result} [{duration_formatted}]");
                }
            }
            Err(err) => {
                let symbol = SYMBOL_ERROR.clone().clone();
                println!("{symbol} {}: {}", name, Red.paint(err));
            }
        }
    }
}

impl Part {
    /// Run and time solution.
    ///
    /// # Errors
    ///
    /// This variant never results in an error.
    pub fn run<Ti>(&self, input: &str, solution: Option<String>) -> PartResult
    where
        Ti: Timer,
    {
        let start = Ti::start();
        let result = (self.implementation)(input);
        let duration = start.elapsed();

        Ok(RunResults {
            result,
            solution,
            duration,
        })
    }
}

/// The thresholds for when a duration is considered good/acceptable.
///
/// This is used to color the times in the outputs.
pub struct DurationThresholds {
    pub good: Duration,
    pub acceptable: Duration,
}
