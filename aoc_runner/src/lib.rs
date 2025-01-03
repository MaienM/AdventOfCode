//! CLI entrypoints for my AoC solutions.

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
