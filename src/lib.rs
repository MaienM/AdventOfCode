pub mod cli;
pub mod derived;
pub mod utils;
pub mod visual;

use aoc_derive::inject_days;

extern crate self as aoc;
#[inject_days(path = "bin")]
pub static DAYS: Vec<Day>;
