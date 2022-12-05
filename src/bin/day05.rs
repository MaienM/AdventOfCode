#![feature(iter_array_chunks)]

use std::collections::VecDeque;

use aoc::runner::*;
use derive_new::new;

#[derive(Debug, Eq, PartialEq, new)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}
type Moves = Vec<Move>;

type Stack = VecDeque<char>;
type Stacks = Vec<Stack>;

fn parse_input(input: String) -> (Stacks, Moves) {
    let [input_state, input_moves]: [&str; 2] = input
        .splitn(2, "\n\n")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();

    let mut stacks = Stacks::new();
    for line in input_state.split("\n") {
        if line.trim().chars().next().unwrap_or('1') == '1' {
            continue;
        }
        for (i, part) in (line.to_owned() + " ")
            .chars()
            .array_chunks::<4>()
            .enumerate()
        {
            if part[1] != ' ' {
                assert!(part[0] == '[');
                assert!(part[2] == ']');

                while i >= stacks.len() {
                    stacks.push(Stack::new());
                }

                stacks[i].push_back(part[1]);
            }
        }
    }

    let moves: Moves = input_moves
        .trim()
        .split("\n")
        .map(str::trim)
        .map(|line| {
            let mut parts = line.split(" ");
            parts.next(); // move
            let count = parts.next().unwrap().parse().unwrap();
            parts.next(); // from
            let from: usize = parts.next().unwrap().parse().unwrap();
            parts.next(); // to
            let to: usize = parts.next().unwrap().parse().unwrap();
            return Move::new(count, from - 1, to - 1);
        })
        .collect();

    return (stacks, moves);
}

fn do_moves(mut stacks: Stacks, moves: Moves) -> Stacks {
    for move_ in moves {
        for _ in 0..move_.count {
            let crate_ = stacks[move_.from].pop_front().unwrap();
            stacks[move_.to].push_front(crate_);
        }
    }
    return stacks;
}

pub fn part1(input: String) -> String {
    let (mut stacks, moves) = parse_input(input);
    stacks = do_moves(stacks, moves);
    return stacks
        .iter()
        .map(Stack::front)
        .map(Option::unwrap)
        .collect();
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string().replace("\n        ", "\n"));
        let expected = (
            vec![
                vec!['N', 'Z'].into(),
                vec!['D', 'C', 'M'].into(),
                vec!['P'].into(),
            ],
            vec![
                Move::new(1, 1, 0),
                Move::new(3, 0, 2),
                Move::new(2, 1, 0),
                Move::new(1, 0, 1),
            ],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(
            part1(EXAMPLE_INPUT.to_string().replace("\n        ", "\n")),
            "CMZ"
        );
    }
}
