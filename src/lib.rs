pub mod cli;
pub mod derived;
pub mod utils;

use aoc_derive::inject_days;

extern crate self as aoc;
#[inject_days(path = "bin")]
pub static DAYS: Vec<Day>;
