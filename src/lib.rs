pub mod counter;
pub mod grid;
pub mod range;
pub mod runner;

pub fn parse_number_list(input: String, sep: &str) -> Vec<i32> {
    return input
        .trim()
        .split(sep)
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
}
