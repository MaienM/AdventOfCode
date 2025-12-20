puzzle_runner::register_chapter!(title = "Infinite Elves and Infinite Houses");

#[register_part(arg = 2*2*3*3*5*7)]
fn part1(input: &str, step: usize) -> usize {
    let threshold: usize = input.parse().unwrap();
    let threshold = threshold.div_ceil(10);
    (step..threshold)
        .into_par_iter()
        .step_by(step)
        .find_first(|num| {
            (2..=(num / 2))
                .filter(|i| num.is_multiple_of(*i))
                .sum::<usize>()
                + num
                + 1
                >= threshold
        })
        .unwrap()
}

#[register_part(arg = 2*2*3*3*5*7)]
fn part2(input: &str, step: usize) -> usize {
    let threshold: usize = input.parse().unwrap();
    let threshold = threshold.div_ceil(11);
    (step..threshold)
        .into_par_iter()
        .step_by(step)
        .find_first(|num| {
            let sum = (usize::max(1, num / 50)..=(num / 2))
                .filter(|i| num % i == 0 && i * 50 >= *num)
                .sum::<usize>()
                + num;
            sum >= threshold
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 6, part1::arg = 2)]
    static EXAMPLE_INPUT_1: &str = "100";

    #[example_input(part1 = 8, part1::arg = 2)]
    static EXAMPLE_INPUT_2: &str = "150";
}
