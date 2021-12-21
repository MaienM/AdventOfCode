use aoc::runner::*;

fn parse_input(input: String) -> [u128; 2] {
    let mut nums = input
        .trim()
        .splitn(2, "\n")
        .map(|line| line.chars().last().unwrap().to_digit(10).unwrap() as u128);
    return [nums.next().unwrap(), nums.next().unwrap()];
}

trait DiceRoller {
    fn roll(&self, times: usize) -> u128;
}

struct DeterministicDiceRoller<T: Iterator<Item = u128>> {
    iter: T,
}
impl<T: Iterator<Item = u128>> DeterministicDiceRoller<T> {
    fn roll(&mut self, times: usize) -> u128 {
        let iter = &mut self.iter;
        return iter.take(times).sum();
    }
}

const DIRAC_DICE_WEIGHT: [(u128, u128); 7] =
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn dirac_do_round(
    player: usize,
    wins: &mut [u128; 2],
    pos: [u128; 2],
    scores: [u128; 2],
    universes: u128,
) {
    for (roll, roll_universes) in DIRAC_DICE_WEIGHT {
        let mut pos = pos.clone();
        pos[player] = (pos[player] + roll - 1) % 10 + 1;
        let mut scores = scores.clone();
        scores[player] += pos[player];
        let universes = roll_universes * universes;
        if scores[player] >= 21 {
            wins[player] += universes;
        } else {
            dirac_do_round(1 - player, wins, pos, scores, universes);
        }
    }
}

fn part1(input: String) -> u128 {
    let mut pos = parse_input(input);
    let mut score = [0, 0];
    let mut rolls = 0;
    let mut roller = DeterministicDiceRoller {
        iter: (1..=100u128).cycle(),
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

fn part2(input: String) -> u128 {
    let pos = parse_input(input);
    let mut wins = [0, 0];
    dirac_do_round(0, &mut wins, pos, [0, 0], 1);
    return wins.into_iter().max().unwrap();
}

fn main() {
    run(part1, part2);
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

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 444356092776315);
    }
}
