use std::collections::HashSet;

use aoc::{generate_day_main, point::Point2};

#[derive(Debug, PartialEq)]
struct Number {
    number: u32,
    points: Vec<Point2>,
}

fn parse_input(input: &str) -> (HashSet<Point2>, Vec<Number>) {
    let mut symbols = HashSet::new();
    let mut numbers = Vec::new();

    let mut number = 0;
    let mut points = vec![];
    macro_rules! finalize_number {
        () => {
            if number > 0 {
                numbers.push(Number { number, points });
                number = 0;
                points = vec![];
            }
        };
    }

    for (y, line) in input.split('\n').enumerate() {
        let mut iter = line.chars().enumerate();

        loop {
            let Some((x, char)) = iter.next() else {
                break;
            };

            match char {
                '0'..='9' => {
                    number = number * 10 + char.to_digit(10).unwrap();
                    points.push(Point2::new(x, y));
                }
                '.' => {
                    finalize_number!();
                }
                _ => {
                    symbols.insert(Point2::new(x, y));
                    finalize_number!();
                }
            }
        }
        finalize_number!();
    }
    (symbols, numbers)
}

fn udiff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

pub fn part1(input: &str) -> u32 {
    let (symbols, numbers) = parse_input(input);
    numbers
        .into_iter()
        .filter(|n| {
            for npoint in &n.points {
                for spoint in &symbols {
                    if udiff(npoint.x, spoint.x) <= 1 && udiff(npoint.y, spoint.y) <= 1 {
                        return true;
                    }
                }
            }
            false
        })
        .map(|n| n.number)
        .sum()
}

generate_day_main!(part1);

#[cfg(test)]
mod tests {
    use aoc::example;
    use macro_rules_attribute::apply;
    use pretty_assertions::assert_eq;

    use super::*;

    #[apply(example)]
    static EXAMPLE_INPUT: String = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = (
            HashSet::from([
                Point2::new(3, 1),
                Point2::new(6, 3),
                Point2::new(3, 4),
                Point2::new(5, 5),
                Point2::new(3, 8),
                Point2::new(5, 8),
            ]),
            vec![
                Number {
                    number: 467,
                    points: vec![Point2::new(0, 0), Point2::new(1, 0), Point2::new(2, 0)],
                },
                Number {
                    number: 114,
                    points: vec![Point2::new(5, 0), Point2::new(6, 0), Point2::new(7, 0)],
                },
                Number {
                    number: 35,
                    points: vec![Point2::new(2, 2), Point2::new(3, 2)],
                },
                Number {
                    number: 633,
                    points: vec![Point2::new(6, 2), Point2::new(7, 2), Point2::new(8, 2)],
                },
                Number {
                    number: 617,
                    points: vec![Point2::new(0, 4), Point2::new(1, 4), Point2::new(2, 4)],
                },
                Number {
                    number: 58,
                    points: vec![Point2::new(7, 5), Point2::new(8, 5)],
                },
                Number {
                    number: 592,
                    points: vec![Point2::new(2, 6), Point2::new(3, 6), Point2::new(4, 6)],
                },
                Number {
                    number: 755,
                    points: vec![Point2::new(6, 7), Point2::new(7, 7), Point2::new(8, 7)],
                },
                Number {
                    number: 664,
                    points: vec![Point2::new(1, 9), Point2::new(2, 9), Point2::new(3, 9)],
                },
                Number {
                    number: 598,
                    points: vec![Point2::new(5, 9), Point2::new(6, 9), Point2::new(7, 9)],
                },
            ],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(&EXAMPLE_INPUT), 4361);
    }
}
