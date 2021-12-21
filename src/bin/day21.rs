use aoc::runner::*;

fn parse_input(input: String) -> [u64; 2] {
    let mut nums = input
        .trim()
        .splitn(2, "\n")
        .map(|line| line.chars().last().unwrap().to_digit(10).unwrap() as u64);
    return [nums.next().unwrap(), nums.next().unwrap()];
}

trait DiceRoller {
    fn roll(&self, times: usize) -> u64;
}

struct DeterministicDiceRoller<T: Iterator<Item = u64>> {
    iter: T,
}
impl<T: Iterator<Item = u64>> DeterministicDiceRoller<T> {
    fn roll(&mut self, times: usize) -> u64 {
        let iter = &mut self.iter;
        return iter.take(times).sum();
    }
}

const DIRAC_MAX_ROUNDS: usize = 12;
const DIRAC_DICE_WEIGHT: [(u64, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
// Tuple of wins / total.
type DiracWinrateByRound = [(u64, u64); DIRAC_MAX_ROUNDS];

fn _dirac_rounds_to_victory(
    pos: u64,
    score: u64,
    rounds: usize,
    universes: u64,
    result: &mut DiracWinrateByRound,
) {
    for (roll, roll_universes) in DIRAC_DICE_WEIGHT {
        let pos = (pos + roll - 1) % 10 + 1;
        let score = score + pos;
        let universes = universes * roll_universes;
        if score >= 21 {
            let old = result[rounds];
            result[rounds] = (old.0 + universes, old.1 + universes);
        } else {
            let old = result[rounds];
            result[rounds] = (old.0, old.1 + universes);
            _dirac_rounds_to_victory(pos, score, rounds + 1, universes, result);
        }
    }
}

fn dirac_rounds_to_victory(pos: u64) -> DiracWinrateByRound {
    let mut result = [(0, 0); DIRAC_MAX_ROUNDS];
    _dirac_rounds_to_victory(pos, 0, 0, 1, &mut result);
    return result;
}

pub fn part1(input: String) -> u64 {
    let mut pos = parse_input(input);
    let mut score = [0, 0];
    let mut rolls = 0;
    let mut roller = DeterministicDiceRoller {
        iter: (1..=100u64).cycle(),
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

pub fn part2(input: String) -> u64 {
    let pos = parse_input(input);
    let winrate_by_round: [DiracWinrateByRound; 2] = pos
        .into_iter()
        .map(dirac_rounds_to_victory)
        .collect::<Vec<DiracWinrateByRound>>()
        .try_into()
        .unwrap();
    let mut wins_total = [0, 0];
    let mut round = 0_usize;
    let mut player = 0_usize;
    let mut universes = 1_u64;

    while universes > 0 {
        universes *= 27;
        let winrate = winrate_by_round[player][round];
        let wins = universes / winrate.1 * winrate.0;
        wins_total[player] += wins;
        universes -= wins;

        round += player;
        player = 1 - player;
    }
    return wins_total.into_iter().max().unwrap();
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
