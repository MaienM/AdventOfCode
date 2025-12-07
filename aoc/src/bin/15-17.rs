puzzle_runner::register_chapter!(book = "2015", title = "No Such Thing as Too Much");

fn parse_input(input: &str) -> Vec<u8> {
    parse!(input => {
        [nums split on '\n' as u8]
    } => nums)
}

fn find_options(containers: &[u8], left: u8) -> usize {
    if left == 0 {
        return 1;
    }
    if containers.is_empty() {
        return 0;
    }

    let with_current = if let Some(left) = left.checked_sub(containers[0]) {
        find_options(&containers[1..], left)
    } else {
        0
    };
    let without_current = find_options(&containers[1..], left);
    with_current + without_current
}

fn part1impl(input: &str, liters: u8) -> usize {
    let containers = parse_input(input);
    find_options(&containers, liters)
}

pub fn part1(input: &str) -> usize {
    part1impl(input, 150)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input]
    static EXAMPLE_INPUT: &str = "
        20
        15
        10
        5
        5
    ";

    #[test]
    fn example_input_part1() {
        assert_eq!(part1impl(&EXAMPLE_INPUT, 25), 4);
    }
}
