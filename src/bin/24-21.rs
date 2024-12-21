use std::{cmp::Ordering, collections::HashMap, vec::Vec};

use aoc::utils::{ext::iter::IterExt, parse, point::Point2};
use common_macros::hash_map;
use itertools::repeat_n;
use memoize::memoize;
use once_cell::sync::Lazy;

fn parse_input(input: &str) -> Vec<&str> {
    parse!(input => {
        [codes split on '\n']
    } => codes)
}

#[derive(Clone)]
struct Keypad {
    positions: HashMap<char, Point2>,
    dead: Point2,
}
impl From<&Keypad> for usize {
    fn from(value: &Keypad) -> Self {
        value.dead.y
    }
}

static NUM_KEYPAD: Lazy<Keypad> = Lazy::new(|| Keypad {
    positions: hash_map![
        '7' => Point2::new(0, 0),
        '8' => Point2::new(1, 0),
        '9' => Point2::new(2, 0),
        '4' => Point2::new(0, 1),
        '5' => Point2::new(1, 1),
        '6' => Point2::new(2, 1),
        '1' => Point2::new(0, 2),
        '2' => Point2::new(1, 2),
        '3' => Point2::new(2, 2),
        '0' => Point2::new(1, 3),
        'A' => Point2::new(2, 3),
    ],
    dead: Point2::new(0, 3),
});

static DIR_KEYPAD: Lazy<Keypad> = Lazy::new(|| Keypad {
    positions: hash_map![
        '^' => Point2::new(1, 0),
        'A' => Point2::new(2, 0),
        '<' => Point2::new(0, 1),
        'v' => Point2::new(1, 1),
        '>' => Point2::new(2, 1),
    ],
    dead: Point2::new(0, 0),
});

#[memoize(Map: keypad -> usize)]
fn get_optimal_step(keypad: &Keypad, from: Point2, to: Point2) -> String {
    if from == to {
        return "A".to_owned();
    }

    // Given that changing direction is pretty expensive (especially with the multiple layers of keypads) it will never be optimal, so we only consider moving in two straight lines. There's (sometimes) two ways to complete a move in such a way: either horizontal first or vertical first. If both are valid (i.e. neither passes over the dead space) the optimal path starts with horizontal if moving left and starts with vertical otherwise.

    let horiz = match from.x.cmp(&to.x) {
        Ordering::Less => repeat_n(">", to.x - from.x).collect(),
        Ordering::Equal => String::new(),
        Ordering::Greater => repeat_n("<", from.x - to.x).collect(),
    };
    let vert = match from.y.cmp(&to.y) {
        Ordering::Less => repeat_n("v", to.y - from.y).collect(),
        Ordering::Equal => String::new(),
        Ordering::Greater => repeat_n("^", from.y - to.y).collect(),
    };

    if keypad.dead.x == from.x && keypad.dead.y == to.y {
        format!("{horiz}{vert}A")
    } else if keypad.dead.x == to.x && keypad.dead.y == from.y {
        format!("{vert}{horiz}A")
    } else if horiz.contains('<') {
        format!("{horiz}{vert}A")
    } else {
        format!("{vert}{horiz}A")
    }
}

#[memoize(Map: keypad -> usize, Map: sequence -> String)]
fn get_optimal_path(keypad: &Keypad, sequence: &str) -> Vec<String> {
    assert!(sequence.find('A') == Some(sequence.len() - 1));
    let mut path = Vec::new();
    let mut pos = keypad.positions[&'A'];
    for chr in sequence.chars() {
        let nextpos = keypad.positions[&chr];
        path.push(get_optimal_step(keypad, pos, nextpos));
        pos = nextpos;
    }
    path
}

fn solve(sequence: &str, robots: usize) -> usize {
    let mut paths = get_optimal_path(&NUM_KEYPAD, sequence)
        .into_iter()
        .count_occurences();

    for _ in 1..robots {
        let mut next_paths = HashMap::new();
        for (sequence, count) in paths {
            for path in get_optimal_path(&DIR_KEYPAD, &sequence) {
                *(next_paths.entry(path).or_default()) += count;
            }
        }
        paths = next_paths;
    }

    paths
        .into_iter()
        .map(|(path, count)| path.len() * count)
        .sum()
}

fn calc(codes: &[&str], robots: usize) -> usize {
    codes
        .iter()
        .map(|code| {
            let presses = solve(code, robots);
            let num: usize = code.trim_end_matches('A').parse().unwrap();
            presses * num
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    let codes = parse_input(input);
    calc(&codes, 3)
}

pub fn part2(input: &str) -> usize {
    let codes = parse_input(input);
    calc(&codes, 26)
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 126_384)]
    static EXAMPLE_INPUT: &str = "
        029A
        980A
        179A
        456A
        379A
    ";

    macro_rules! test_solve {
        ($input:literal, $robots:literal, $len:literal) => {
            ::paste::paste! {
                #[allow(non_snake_case)]
                #[test]
                fn [< example_solve_ $input _ $robots >]() {
                    let actual = solve($input, $robots);
                    assert_eq!(actual, $len);
                }
            }
        };
    }

    test_solve!("029A", 1, 12);
    test_solve!("029A", 2, 28);
    test_solve!("029A", 3, 68);
    test_solve!("029A", 26, 82_050_061_710);

    test_solve!("980A", 3, 60);
    test_solve!("980A", 26, 72_242_026_390);

    test_solve!("179A", 3, 68);
    test_solve!("179A", 26, 81_251_039_228);

    test_solve!("456A", 3, 64);
    test_solve!("456A", 26, 80_786_362_258);

    test_solve!("379A", 3, 64);
    test_solve!("379A", 26, 77_985_628_636);
}
