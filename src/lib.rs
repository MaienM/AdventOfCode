pub mod counter;
pub mod grid;
pub mod runner;

#[macro_use]
extern crate derive_new;

pub fn parse_number_list(input: String, sep: &str) -> Vec<i32> {
    return input
        .trim()
        .split(sep)
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
}
