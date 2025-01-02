aoc::setup!();

use std::collections::HashMap;

use aoc::point::Point2;

type Point = Point2<usize>;

#[derive(Debug, PartialEq)]
struct Number {
    number: u32,
    points: Vec<Point>,
}

#[derive(Debug, PartialEq)]
struct Symbol {
    symbol: char,
    point: Point,
}

fn parse_input(input: &str) -> (Vec<Symbol>, Vec<Number>) {
    let mut symbols = Vec::new();
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
                    points.push(Point::new(x, y));
                }
                '.' => {
                    finalize_number!();
                }
                _ => {
                    symbols.push(Symbol {
                        symbol: char,
                        point: Point::new(x, y),
                    });
                    finalize_number!();
                }
            }
        }
        finalize_number!();
    }
    (symbols, numbers)
}

pub fn part1(input: &str) -> u32 {
    let (symbols, numbers) = parse_input(input);
    numbers
        .into_iter()
        .filter_map(|n| {
            for npoint in &n.points {
                for symbol in &symbols {
                    if usize::abs_diff(npoint.x, symbol.point.x) <= 1
                        && usize::abs_diff(npoint.y, symbol.point.y) <= 1
                    {
                        return Some(n.number);
                    }
                }
            }
            None
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let (symbols, numbers) = parse_input(input);
    let mut gears: HashMap<Point, Vec<u32>> =
        symbols.iter().map(|s| (s.point, Vec::new())).collect();
    'number: for number in numbers {
        for npoint in &number.points {
            for symbol in &symbols {
                if usize::abs_diff(npoint.x, symbol.point.x) <= 1
                    && usize::abs_diff(npoint.y, symbol.point.y) <= 1
                {
                    gears.get_mut(&symbol.point).unwrap().push(number.number);
                    continue 'number;
                }
            }
        }
    }
    gears
        .into_iter()
        .filter_map(|(point, gear)| match gear.len() {
            2 => Some(gear[0] * gear[1]),
            0 | 1 => None,
            _ => panic!("Excessive gear at {point:?}: {gear:?}."),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use aoc_runner::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 4361, part2 = 467_835)]
    static EXAMPLE_INPUT: &str = "
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
            vec![
                Symbol {
                    symbol: '*',
                    point: Point::new(3, 1),
                },
                Symbol {
                    symbol: '#',
                    point: Point::new(6, 3),
                },
                Symbol {
                    symbol: '*',
                    point: Point::new(3, 4),
                },
                Symbol {
                    symbol: '+',
                    point: Point::new(5, 5),
                },
                Symbol {
                    symbol: '$',
                    point: Point::new(3, 8),
                },
                Symbol {
                    symbol: '*',
                    point: Point::new(5, 8),
                },
            ],
            vec![
                Number {
                    number: 467,
                    points: vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0)],
                },
                Number {
                    number: 114,
                    points: vec![Point::new(5, 0), Point::new(6, 0), Point::new(7, 0)],
                },
                Number {
                    number: 35,
                    points: vec![Point::new(2, 2), Point::new(3, 2)],
                },
                Number {
                    number: 633,
                    points: vec![Point::new(6, 2), Point::new(7, 2), Point::new(8, 2)],
                },
                Number {
                    number: 617,
                    points: vec![Point::new(0, 4), Point::new(1, 4), Point::new(2, 4)],
                },
                Number {
                    number: 58,
                    points: vec![Point::new(7, 5), Point::new(8, 5)],
                },
                Number {
                    number: 592,
                    points: vec![Point::new(2, 6), Point::new(3, 6), Point::new(4, 6)],
                },
                Number {
                    number: 755,
                    points: vec![Point::new(6, 7), Point::new(7, 7), Point::new(8, 7)],
                },
                Number {
                    number: 664,
                    points: vec![Point::new(1, 9), Point::new(2, 9), Point::new(3, 9)],
                },
                Number {
                    number: 598,
                    points: vec![Point::new(5, 9), Point::new(6, 9), Point::new(7, 9)],
                },
            ],
        );
        assert_eq!(actual, expected);
    }
}
