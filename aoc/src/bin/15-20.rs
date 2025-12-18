puzzle_runner::register_chapter!(book = 2015, title = "Infinite Elves and Infinite Houses");

#[register_part]
fn part1(input: &str) -> usize {
    let threshold: usize = input.parse().unwrap();
    let elfsum = threshold / 10;
    (0..usize::MAX)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|num| (1..=(num / 2)).filter(|i| num % i == 0).sum::<usize>() + num >= elfsum)
        .unwrap()
}

#[register_part]
fn part2(input: &str) -> usize {
    let threshold: usize = input.parse().unwrap();
    (0..usize::MAX)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|num| {
            let sum = (usize::max(1, num / 50)..=(num / 2))
                .filter(|i| num % i == 0 && i * 50 >= *num)
                .sum::<usize>()
                + num;
            sum * 11 > threshold
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 6)]
    static EXAMPLE_INPUT_1: &str = "100";

    #[example_input(part1 = 8)]
    static EXAMPLE_INPUT_2: &str = "150";
}
