use aoc_derive::inject_days;

#[inject_days]
static DAYS: Vec<Day>;

fn main() {
    aoc::cli::multi::main(&DAYS);
}
