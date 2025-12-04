puzzle_runner::register_chapter!(book = "2025", title = "Lobby");

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

pub fn part2(input: &str) -> usize {
    let banks = parse_input(input);
    banks
        .into_iter()
        .map(|mut bank| {
            let len = bank.len();
            // Figure out what the largest + earliest (in that order of importance) possible starting number is.
            let idx = bank.iter().rev().skip(11).position_max().unwrap();
            let idx = len - idx - 12;
            for _ in 0..idx {
                bank.remove(0);
            }
            // Shrink to the target size by removing either the first number that's smaller than
            // the one that follows it, or the last number.
            'top: while bank.len() > 12 {
                let mut last = 9;
                for (idx, num) in bank.iter().enumerate() {
                    if *num > last {
                        bank.remove(idx - 1);
                        continue 'top;
                    }
                    last = *num;
                }
                bank.pop();
            }
            bank.into_iter()
                .map_into::<usize>()
                .reduce(|a, b| a * 10 + b)
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 357, part2 = 3_121_910_778_619)]
    static EXAMPLE_INPUT: &str = "
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    ";
}
