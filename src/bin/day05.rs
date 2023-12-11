use std::{collections::HashMap, ops::Range};

use aoc::utils::parse::splitn;

#[derive(Debug, PartialEq)]
struct Input {
    seeds: Vec<usize>,
    maps: HashMap<String, Map>,
}

#[derive(Debug, PartialEq)]
struct Translation {
    source: Range<usize>,
    offset: isize,
}

#[derive(Debug, PartialEq)]
struct Map {
    target: String,
    translations: Vec<Translation>,
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
                    Translation {
                        source: source_start..(source_start + len),
                        offset: (dest_start as isize - source_start as isize),
                    }
                })
                .collect();
            (
                from.to_owned(),
                Map {
                    target: to.to_owned(),
                    translations: ranges,
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
                    .translations
                    .iter()
                    .find(|r| r.source.contains(&n))
                    .map_or(0, |r| r.offset);
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
    let mut iter = input.seeds.chunks_exact(2);
    while let Some([start, len]) = iter.next() {
        for i in 0..*len {
            seeds.push(start + i);
        }
    }
    input.seeds = seeds;

    find_lowest_location(input)
}

aoc::cli::single::generate_main!();

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
    #[allow(clippy::too_many_lines)]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let range = |start, length| start..(start + length);
        let expected = Input {
            seeds: vec![79, 14, 55, 13],
            maps: HashMap::from([
                (
                    "seed".to_owned(),
                    Map {
                        target: "soil".to_owned(),
                        translations: vec![
                            Translation {
                                source: range(98, 2),
                                offset: 50 - 98,
                            },
                            Translation {
                                source: range(50, 48),
                                offset: 52 - 50,
                            },
                        ],
                    },
                ),
                (
                    "soil".to_owned(),
                    Map {
                        target: "fertilizer".to_owned(),
                        translations: vec![
                            Translation {
                                source: range(15, 37),
                                offset: 0 - 15,
                            },
                            Translation {
                                source: range(52, 2),
                                offset: 37 - 52,
                            },
                            Translation {
                                source: range(0, 15),
                                offset: 39,
                            },
                        ],
                    },
                ),
                (
                    "fertilizer".to_owned(),
                    Map {
                        target: "water".to_owned(),
                        translations: vec![
                            Translation {
                                source: range(53, 8),
                                offset: 49 - 53,
                            },
                            Translation {
                                source: range(11, 42),
                                offset: 0 - 11,
                            },
                            Translation {
                                source: range(0, 7),
                                offset: 42,
                            },
                            Translation {
                                source: range(7, 4),
                                offset: 57 - 7,
                            },
                        ],
                    },
                ),
                (
                    "water".to_owned(),
                    Map {
                        target: "light".to_owned(),
                        translations: vec![
                            Translation {
                                source: range(18, 7),
                                offset: 88 - 18,
                            },
                            Translation {
                                source: range(25, 70),
                                offset: 18 - 25,
                            },
                        ],
                    },
                ),
                (
                    "light".to_owned(),
                    Map {
                        target: "temperature".to_owned(),
                        translations: vec![
                            Translation {
                                source: range(77, 23),
                                offset: 45 - 77,
                            },
                            Translation {
                                source: range(45, 19),
                                offset: 81 - 45,
                            },
                            Translation {
                                source: range(64, 13),
                                offset: 68 - 64,
                            },
                        ],
                    },
                ),
                (
                    "temperature".to_owned(),
                    Map {
                        target: "humidity".to_owned(),
                        translations: vec![
                            Translation {
                                source: range(69, 1),
                                offset: 0 - 69,
                            },
                            Translation {
                                source: range(0, 69),
                                offset: 1,
                            },
                        ],
                    },
                ),
                (
                    "humidity".to_owned(),
                    Map {
                        target: "location".to_owned(),
                        translations: vec![
                            Translation {
                                source: range(56, 37),
                                offset: 60 - 56,
                            },
                            Translation {
                                source: range(93, 4),
                                offset: 56 - 93,
                            },
                        ],
                    },
                ),
            ]),
        };
        assert_eq!(actual, expected);
    }
}
