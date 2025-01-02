//! CLI entrypoints for my AoC solutions.

pub mod bench;
pub mod derived;
pub mod multi;
pub mod runner;
pub mod single;
mod source;

#[doc(hidden)]
extern crate self as aoc_runner;

pub use aoc_runner_derive::*;
