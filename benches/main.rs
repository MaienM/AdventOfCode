use aoc_derive::inject_days;

#[inject_days(path = "../src/bin")]
static DAYS: Vec<Day>;

fn main() {
    aoc::cli::multi::bench(&DAYS);
}
