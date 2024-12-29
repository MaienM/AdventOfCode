//! My solutions to the [Advent of Code](https://adventofcode.com) challenges.

pub mod cli;
pub mod derived;
pub mod utils;
pub mod visual;

use aoc_derive::inject_binaries;

#[doc(hidden)]
extern crate self as aoc;

#[doc(hidden)]
#[inject_binaries(path = "bin")]
pub static BINS: Vec<Bin>;
