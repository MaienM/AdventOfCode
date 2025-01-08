//! Scaffolding to run/test/benchmark solutions for programming puzzles/challenges.
//!
//! Each day is solved in a separate source file (in `aoc/bin`). The [`register`] macro generates a
//! [`Bin`](puzzle_runner::derived::Bin) for each of these, and the [`register_crate`] macro generates
//! a static & the entrypoints needed to run multiple at the same time.

#[doc(hidden)]
extern crate self as puzzle_runner;

pub mod bench;
pub mod derived;
pub mod multi;
pub mod runner;
pub mod single;
mod source;
pub mod visual;

pub use puzzle_runner_derive::{example_input, register, register_crate, visual};
