//! My solutions to the [Advent of Code](https://adventofcode.com) challenges.

mod controller;

puzzle_runner::register_series!(
    title = "Advent of Code",
    controller = controller::AoCController
);
