pub mod cli;
pub mod derived;
pub mod utils;
pub mod visual;

use aoc_derive::inject_binaries;

extern crate self as aoc;
#[inject_binaries(path = "bin")]
pub static BINS: Vec<Bin>;
