//! Helpers to run solutions & collect the results & runtimes.

use std::time::{Duration, Instant};

use ansi_term::{
    unstyle, ANSIStrings,
    Colour::{Blue, Green, Purple, Red},
};
use once_cell::sync::Lazy;

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

/// A function that takes the input file as a string and returns the solution to one of the assignments.
#[derive(Clone)]
pub enum Solver<T> {
    Implemented(fn(&str) -> T),
    NotImplemented,
}
impl<T> From<Option<fn(&str) -> T>> for Solver<T> {
    fn from(value: Option<fn(&str) -> T>) -> Self {
        match value {
            Some(f) => Solver::Implemented(f),
            None => Solver::NotImplemented,
        }
    }
}
impl<T> Solver<T>
where
    T: ToString,
{
    pub fn run(&self, input: &str, solution: Option<String>) -> SolverRunResult {
        self.run_with_timer::<InstantTimer>(input, solution)
    }

    pub fn run_with_timer<Ti>(&self, input: &str, solution: Option<String>) -> SolverRunResult
    where
        Ti: Timer,
    {
        let Solver::Implemented(runnable) = self else {
            return SolverRunResult::Error("Not implemented.".to_string());
        };

        let start = Ti::start();
        let result = runnable(input);
        let duration = start.elapsed();

        SolverRunResult::Success {
            result: result.to_string(),
            solution,
            duration,
        }
    }
}

static SYMBOL_UNKNOWN: Lazy<String> = Lazy::new(|| "?".to_owned());
static SYMBOL_OK: Lazy<String> = Lazy::new(|| Green.paint("✔").to_string());
static SYMBOL_INCORRECT: Lazy<String> = Lazy::new(|| Red.paint("✘").to_string());
static SYMBOL_ERROR: Lazy<String> = Lazy::new(|| Red.paint("⚠").to_string());

/// The result of running a [`Solver`].
#[derive(Clone)]
pub enum SolverRunResult {
    /// A successful run.
    Success {
        /// The result of the solver, converted to a string.
        result: String,
        /// The expected result of the solver, if known.
        solution: Option<String>,
        /// The duration of the solver run.
        duration: Duration,
    },
    /// An attempted run that was aborted for some reason.
    Error(String),
}
impl SolverRunResult {
    pub fn print(&self, name: &str, thresholds: &DurationThresholds, show_result: bool) {
        let name = Purple.paint(name);
        match self {
            SolverRunResult::Success {
                result,
                solution,
                duration,
            } => {
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
            SolverRunResult::Error(err) => {
                let symbol = SYMBOL_ERROR.clone().clone();
                println!("{symbol} {}: {}", name, Red.paint(err));
            }
        }
    }
}

/// The thresholds for when a duration is considered good/acceptable.
///
/// This is used to color the times in the outputs.
pub struct DurationThresholds {
    pub good: Duration,
    pub acceptable: Duration,
}
