use std::{collections::HashMap, ops::Range};

use aoc::{generate_day_main, splitn};

#[derive(Debug, PartialEq)]
struct Input {
    seeds: Vec<usize>,
    maps: HashMap<String, Mapping>,
}

#[derive(Debug, PartialEq)]
struct Mapping {
    target: String,
    ranges: Vec<(Range<usize>, isize)>,
}

fn parse_input(input: &str) -> Input {
    let (seeds, maps) = splitn!(input, "\n\n", str, str);

    let (_, seeds) = splitn!(seeds, ": ", str, str);
    let seeds = seeds.split(' ').map(|s| s.parse().unwrap()).collect();

    let maps = maps
        .split("\n\n")
        .map(|lines| {
            let (header, ranges) = splitn!(lines, '\n', str, str);
            let (header, _) = splitn!(header, ' ', str, str);
            let (from, _, to) = splitn!(header, '-', str, str, str);
            let ranges = ranges
                .split('\n')
                .map(|line| {
                    let (dest_start, source_start, len) = splitn!(line, ' ', usize, usize, usize);
                    (
                        source_start..(source_start + len),
                        (dest_start as isize - source_start as isize),
                    )
                })
                .collect();
            (
                from.to_owned(),
                Mapping {
                    target: to.to_owned(),
                    ranges,
                },
            )
        })
        .collect();

    Input { seeds, maps }
}

fn find_lowest_location(input: Input) -> usize {
    let mut current = "seed".to_owned();
    let mut items = input.seeds;

    while current != "location" {
        let map = input.maps.get(&current).unwrap();
        current = map.target.clone();
        items = items
            .into_iter()
            .map(|n| {
                let offset = map
                    .ranges
                    .iter()
                    .find(|r| r.0.contains(&n))
                    .map_or(0, |r| r.1);
                (n as isize + offset) as usize
            })
            .collect();
    }

    items.into_iter().min().unwrap()
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    find_lowest_location(input)
}

pub fn part2(input: &str) -> usize {
    let mut input = parse_input(input);

    let mut seeds = Vec::new();
    while !input.seeds.is_empty() {
        let len = input.seeds.pop().unwrap();
        let start = input.seeds.pop().unwrap();
        for i in 0..len {
            seeds.push(start + i);
        }
    }
    input.seeds = seeds;

    find_lowest_location(input)
}

generate_day_main!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 35, part2 = 46, test)]
    static EXAMPLE_INPUT: &str = "
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = Input {
            seeds: vec![79, 14, 55, 13],
            maps: HashMap::from([
                (
                    "seed".to_owned(),
                    Mapping {
                        target: "soil".to_owned(),
                        ranges: vec![
                            (98..(98 + 2), (50isize - 98isize)),
                            (50..(50 + 48), (52isize - 50isize)),
                        ],
                    },
                ),
                (
                    "soil".to_owned(),
                    Mapping {
                        target: "fertilizer".to_owned(),
                        ranges: vec![
                            (15..(15 + 37), (0isize - 15isize)),
                            (52..(52 + 2), (37isize - 52isize)),
                            (0..(0 + 15), (39isize - 0isize)),
                        ],
                    },
                ),
                (
                    "fertilizer".to_owned(),
                    Mapping {
                        target: "water".to_owned(),
                        ranges: vec![
                            (53..(53 + 8), (49isize - 53isize)),
                            (11..(11 + 42), (0isize - 11isize)),
                            (0..(0 + 7), (42isize - 0isize)),
                            (7..(7 + 4), (57isize - 7isize)),
                        ],
                    },
                ),
                (
                    "water".to_owned(),
                    Mapping {
                        target: "light".to_owned(),
                        ranges: vec![
                            (18..(18 + 7), (88isize - 18isize)),
                            (25..(25 + 70), (18isize - 25isize)),
                        ],
                    },
                ),
                (
                    "light".to_owned(),
                    Mapping {
                        target: "temperature".to_owned(),
                        ranges: vec![
                            (77..(77 + 23), (45isize - 77isize)),
                            (45..(45 + 19), (81isize - 45isize)),
                            (64..(64 + 13), (68isize - 64isize)),
                        ],
                    },
                ),
                (
                    "temperature".to_owned(),
                    Mapping {
                        target: "humidity".to_owned(),
                        ranges: vec![
                            (69..(69 + 1), (0isize - 69isize)),
                            (0..(0 + 69), (1isize - 0isize)),
                        ],
                    },
                ),
                (
                    "humidity".to_owned(),
                    Mapping {
                        target: "location".to_owned(),
                        ranges: vec![
                            (56..(56 + 37), (60isize - 56isize)),
                            (93..(93 + 4), (56isize - 93isize)),
                        ],
                    },
                ),
            ]),
        };
        assert_eq!(actual, expected);
    }
}
