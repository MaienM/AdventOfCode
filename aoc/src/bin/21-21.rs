puzzle_runner::register_chapter!(book = 2021, title = "Dirac Dice");

fn parse_input(input: &str) -> [u64; 2] {
    parse!(input => {
        "Player 1 starting position: " [p1 as u64] '\n'
        "Player 2 starting position: " [p2 as u64]
    } => [p1, p2])
}

struct DeterministicDiceRoller<T: Iterator<Item = u64>> {
    iter: T,
}
impl<T: Iterator<Item = u64>> DeterministicDiceRoller<T> {
    fn roll(&mut self, times: usize) -> u64 {
        let iter = &mut self.iter;
        iter.take(times).sum()
    }
}

const DIRAC_MAX_ROUNDS: usize = 12;
const DIRAC_DICE_WEIGHT: [(u64, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
// Tuple of wins / total.
type DiracWinrateByRound = [(u64, u64); DIRAC_MAX_ROUNDS];

fn dirac_rounds_to_victory_impl(
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
            dirac_rounds_to_victory_impl(pos, score, rounds + 1, universes, result);
        }
    }
}

fn dirac_rounds_to_victory(pos: u64) -> DiracWinrateByRound {
    let mut result = [(0, 0); DIRAC_MAX_ROUNDS];
    dirac_rounds_to_victory_impl(pos, 0, 0, 1, &mut result);
    result
}

#[register_part]
fn part1(input: &str) -> u64 {
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

    never!();
}

#[register_part]
fn part2(input: &str) -> u64 {
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
    let mut universes = 1u64;

    while universes > 0 {
        universes *= 27;
        let winrate = winrate_by_round[player][round];
        let wins = universes / winrate.1 * winrate.0;
        wins_total[player] += wins;
        universes -= wins;

        round += player;
        player = 1 - player;
    }
    wins_total.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 739_785, part2 = 444_356_092_776_315)]
    static EXAMPLE_INPUT: &str = "
        Player 1 starting position: 4
        Player 2 starting position: 8
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = [4, 8];
        assert_eq!(actual, expected);
    }
}
