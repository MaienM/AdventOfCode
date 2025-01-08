aoc::setup!(title = "Supply Stacks");

use std::collections::VecDeque;

use derive_new::new;

#[derive(Debug, Eq, PartialEq, new)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

type Stack = VecDeque<char>;
type Stacks = Vec<Stack>;

fn parse_input(input: &str) -> (Stacks, Vec<Move>) {
    parse!(input => input_state "\n\n" input_moves);

    let mut stacks = Stacks::new();
    for line in input_state.split('\n') {
        if line.trim().chars().next().unwrap_or('1') == '1' {
            continue;
        }
        for i in 0..=(line.len() / 4) {
            let crate_ = line.chars().nth(i * 4 + 1).unwrap();
            if crate_ != ' ' {
                while i >= stacks.len() {
                    stacks.push(Stack::new());
                }
                stacks[i].push_back(crate_);
            }
        }
    }

    parse!(input_moves => [
        moves split on '\n' with
        { "move " [count as usize] " from " [from as usize] " to " [to as usize] }
        => Move::new(count, from - 1, to - 1)
    ]);

    (stacks, moves)
}

fn do_move_9000(stacks: &mut Stacks, move_: &Move) {
    for _ in 0..move_.count {
        let crate_ = stacks[move_.from].pop_front().unwrap();
        stacks[move_.to].push_front(crate_);
    }
}

fn do_move_9001(stacks: &mut Stacks, move_: &Move) {
    let mut stack = Stack::new();
    for _ in 0..move_.count {
        let crate_ = stacks[move_.from].pop_front().unwrap();
        stack.push_front(crate_);
    }
    for crate_ in stack {
        stacks[move_.to].push_front(crate_);
    }
}

fn get_stack_tops(stacks: &Stacks) -> String {
    stacks
        .iter()
        .map(Stack::front)
        .map(Option::unwrap)
        .collect()
}

pub fn part1(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    for move_ in moves {
        do_move_9000(&mut stacks, &move_);
    }
    get_stack_tops(&stacks)
}

pub fn part2(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    for move_ in moves {
        do_move_9001(&mut stacks, &move_);
    }
    get_stack_tops(&stacks)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = "CMZ", part2 = "MCD")]
    static EXAMPLE_INPUT: &str = "
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    ";

    macro_rules! vec_deque {
        [$($item:expr),* $(,)?] => {
            VecDeque::from(vec![$($item),*])
        };
    }

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = (
            vec![
                vec_deque!['N', 'Z'],
                vec_deque!['D', 'C', 'M'],
                vec_deque!['P'],
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
    fn test_do_move_9000() {
        let mut stacks = vec![
            vec_deque!['N', 'Z'],
            vec_deque!['D', 'C', 'M'],
            vec_deque!['P'],
        ];
        do_move_9000(&mut stacks, &Move::new(1, 1, 0));
        assert_eq!(
            stacks,
            vec![
                vec_deque!['D', 'N', 'Z'],
                vec_deque!['C', 'M'],
                vec_deque!['P'],
            ]
        );
        do_move_9000(&mut stacks, &Move::new(3, 0, 2));
        assert_eq!(
            stacks,
            vec![
                vec_deque![],
                vec_deque!['C', 'M'],
                vec_deque!['Z', 'N', 'D', 'P'],
            ]
        );
        do_move_9000(&mut stacks, &Move::new(2, 1, 0));
        assert_eq!(
            stacks,
            vec![
                vec_deque!['M', 'C'],
                vec_deque![],
                vec_deque!['Z', 'N', 'D', 'P'],
            ]
        );
        do_move_9000(&mut stacks, &Move::new(1, 0, 1));
        assert_eq!(
            stacks,
            vec![
                vec_deque!['C'],
                vec_deque!['M'],
                vec_deque!['Z', 'N', 'D', 'P'],
            ]
        );
    }

    #[test]
    fn test_do_move_9001() {
        let mut stacks = vec![
            vec_deque!['N', 'Z'],
            vec_deque!['D', 'C', 'M'],
            vec_deque!['P'],
        ];
        do_move_9001(&mut stacks, &Move::new(1, 1, 0));
        assert_eq!(
            stacks,
            vec![
                vec_deque!['D', 'N', 'Z'],
                vec_deque!['C', 'M'],
                vec_deque!['P'],
            ]
        );
        do_move_9001(&mut stacks, &Move::new(3, 0, 2));
        assert_eq!(
            stacks,
            vec![
                vec_deque![],
                vec_deque!['C', 'M'],
                vec_deque!['D', 'N', 'Z', 'P'],
            ]
        );
        do_move_9001(&mut stacks, &Move::new(2, 1, 0));
        assert_eq!(
            stacks,
            vec![
                vec_deque!['C', 'M'],
                vec_deque![],
                vec_deque!['D', 'N', 'Z', 'P'],
            ]
        );
        do_move_9001(&mut stacks, &Move::new(1, 0, 1));
        assert_eq!(
            stacks,
            vec![
                vec_deque!['M'],
                vec_deque!['C'],
                vec_deque!['D', 'N', 'Z', 'P'],
            ]
        );
    }
}
