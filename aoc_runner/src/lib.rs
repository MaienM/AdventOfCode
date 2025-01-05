//! Scaffolding to run/test/benchmark AoC solutions.
//!
//! Each day is solved in a separate source file (in `aoc/bin`). The [`inject_binary`] and
//! [`inject_binaries`] macros find these and generate a [`Bin`](aoc_runner::derived::Bin) for each
//! of these.

#[doc(hidden)]
extern crate self as aoc_runner;

pub mod bench;
pub mod derived;
pub mod multi;
pub mod runner;
pub mod single;
mod source;
pub mod visual;

pub use aoc_runner_derive::{example_input, inject_binaries, inject_binary, visual};
