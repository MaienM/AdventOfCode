puzzle_lib::setup!(title = "The Ideal Stocking Stuffer");

use md5::{Digest, Md5};

pub fn part1(input: &str) -> usize {
    (0..usize::MAX)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|n| {
            let mut hasher = Md5::new();
            hasher.update(format!("{input}{n}"));
            let hash = hasher.finalize();
            hash[0] == 0 && hash[1] == 0 && hash[2] < 16
        })
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    (0..usize::MAX)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|n| {
            let mut hasher = Md5::new();
            hasher.update(format!("{input}{n}"));
            let hash = hasher.finalize();
            hash[0] == 0 && hash[1] == 0 && hash[2] == 0
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 609_043)]
    static EXAMPLE_INPUT_1: &str = "abcdef";

    #[example_input(part1 = 1_048_970)]
    static EXAMPLE_INPUT_2: &str = "pqrstuv";
}
