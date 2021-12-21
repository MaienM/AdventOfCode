use aoc::runner::*;

fn parse_input(input: String) -> [u32; 2] {
    let mut nums = input
        .trim()
        .splitn(2, "\n")
        .map(|line| line.chars().last().unwrap().to_digit(10).unwrap() as u32);
    return [nums.next().unwrap(), nums.next().unwrap()];
}

trait DiceRoller {
    fn roll(&self, times: usize) -> u32;
}

struct DeterministicDiceRoller<T: Iterator<Item = u32>> {
    iter: T,
}
impl<T: Iterator<Item = u32>> DeterministicDiceRoller<T> {
    fn roll(&mut self, times: usize) -> u32 {
        let iter = &mut self.iter;
        return iter.take(times).sum();
    }
}

fn part1(input: String) -> u32 {
    let mut pos = parse_input(input);
    let mut score = [0, 0];
    let mut rolls = 0;
    let mut roller = DeterministicDiceRoller {
        iter: (1..=100u32).cycle(),
    };

    for p in (0..=1).cycle() {
        let mov = roller.roll(3);
        rolls += 3;
        pos[p] = (pos[p] + mov - 1) % 10 + 1;
        score[p] += pos[p];
        if score[p] >= 1000 {
            return rolls * score[1 - p];
        }
    }

    panic!("Should not happen");
}

fn main() {
    run(part1, missing::<u64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        Player 1 starting position: 4
        Player 2 starting position: 8
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = [4, 8];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 739785);
    }
}
