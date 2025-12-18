puzzle_runner::register_chapter!(title = "Perfectly Spherical Houses in a Vacuum");

use std::collections::HashSet;

use puzzle_lib::point::{Direction2, Point2};

fn parse_input(input: &str) -> Vec<Direction2> {
    parse!(input => {
        [directions chars match {
            '^' => Direction2::North,
            '>' => Direction2::East,
            'v' => Direction2::South,
            '<' => Direction2::West,
        }]
    } => directions)
}

#[register_part]
fn part1(input: &str) -> usize {
    let directions = parse_input(input);
    let mut visited = HashSet::new();
    let mut current = Point2::new(0, 0);
    visited.insert(current);
    for direction in directions {
        current += direction;
        visited.insert(current);
    }
    visited.len()
}

#[register_part]
fn part2(input: &str) -> usize {
    let directions = parse_input(input);
    let mut visited = HashSet::new();
    let mut current = [Point2::new(0, 0), Point2::new(0, 0)];
    visited.insert(current[0]);
    for chunk in &directions.into_iter().chunks(2) {
        for (i, direction) in chunk.enumerate() {
            current[i] += direction;
            visited.insert(current[i]);
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 2)]
    static EXAMPLE_INPUT_1: &str = ">";

    #[example_input(part1 = 4, part2 = 3)]
    static EXAMPLE_INPUT_2: &str = "^>v<";

    #[example_input(part1 = 2, part2 = 11)]
    static EXAMPLE_INPUT_3: &str = "^v^v^v^v^v";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT_2);
        let expected = vec![
            Direction2::North,
            Direction2::East,
            Direction2::South,
            Direction2::West,
        ];
        assert_eq!(actual, expected);
    }
}
