//! My solutions to the [Advent of Code](https://adventofcode.com) challenges.

mod controller;

puzzle_runner::register_series!(
    title = "Advent of Code",
    description = "Advent of code is an advent calendar of programming puzzles made by Eric Wastl.",
    url = "https://adventofcode.com",
    controller = controller::AoCController
);
