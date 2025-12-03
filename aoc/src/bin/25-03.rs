puzzle_lib::setup!(title = "Lobby");

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    parse!(input => {
        [banks split on '\n' with [chars as u8]]
    } => banks)
}

pub fn part1(input: &str) -> usize {
    let banks = parse_input(input);
    banks
        .into_iter()
        .map(|bank| {
            let len = bank.len();
            let idx = bank.iter().rev().skip(1).position_max().unwrap();
            let idx = len - idx - 2;
            let first = bank[idx];
            let second = bank[(idx + 1)..].iter().max().unwrap();
            (first * 10 + second) as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 357)]
    static EXAMPLE_INPUT: &str = "
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    ";
}
