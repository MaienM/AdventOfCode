use std::{collections::HashMap, mem, ops::Range};

use aoc::utils::parse;

#[derive(Debug, PartialEq)]
struct Input<'a> {
    seeds: Vec<usize>,
    maps: HashMap<&'a str, Map<'a>>,
}

#[derive(Debug, PartialEq)]
struct Translation {
    source: Range<usize>,
    offset: isize,
}

#[derive(Debug, PartialEq)]
struct Map<'a> {
    target: &'a str,
    translations: Vec<Translation>,
}

fn parse_input(input: &str) -> Input {
    parse!(input => {
        "seeds: " [seeds split as usize]
        "\n\n"
        [maps split on "\n\n" into (HashMap<_, _>) with
            {
                source "-to-" target " map:\n" 
                [translations split on '\n' with
                    { [dest_start as usize] " " [source_start as usize] " " [len as usize] }
                    => Translation {
                        source: source_start..(source_start + len),
                        offset: (dest_start as isize - source_start as isize),
                    }
                ]
            } => (
                source,
                Map { target, translations },
            )
        ]
    } => Input { seeds, maps })
}

fn find_lowest_location(input: Input) -> usize {
    let mut current = "seed";
    let mut items = input.seeds;

    while current != "location" {
        let map = input.maps.get(&current).unwrap();
        current = map.target;
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

fn offset_range(range: Range<usize>, offset: isize) -> Range<usize> {
    let start = (range.start as isize + offset) as usize;
    let end = (range.end as isize + offset) as usize;
    start..end
}

fn find_lowest_location_ranges(seeds: Vec<Range<usize>>, maps: &HashMap<&str, Map>) -> usize {
    let mut current = "seed";
    let mut items = seeds;

    while current != "location" {
        let map = maps.get(current).unwrap();
        current = map.target;
        items = items
            .into_iter()
            .flat_map(|range| {
                let mut unmapped = vec![range];
                let mut mapped = Vec::new();
                for translation in &map.translations {
                    for range in mem::take(&mut unmapped) {
                        if translation.source.contains(&range.start)
                            && translation.source.contains(&(range.end - 1))
                        {
                            mapped.push(offset_range(range.clone(), translation.offset));
                        } else if translation.source.contains(&range.start) {
                            mapped.push(offset_range(
                                range.start..translation.source.end,
                                translation.offset,
                            ));
                            unmapped.push(translation.source.end..range.end);
                        } else if translation.source.contains(&(range.end - 1)) {
                            mapped.push(offset_range(
                                translation.source.start..range.end,
                                translation.offset,
                            ));
                            unmapped.push(range.start..translation.source.start);
                        } else if range.contains(&translation.source.start) {
                            mapped
                                .push(offset_range(translation.source.clone(), translation.offset));
                            unmapped.push(range.start..translation.source.start);
                            unmapped.push(translation.source.end..range.end);
                        } else {
                            unmapped.push(range);
                        }
                    }
                }
                mapped.append(&mut unmapped);
                mapped
            })
            .collect();
    }

    items.into_iter().map(|r| r.start).min().unwrap()
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    #[allow(clippy::range_plus_one)]
    find_lowest_location(input)
}

pub fn part2(input: &str) -> usize {
    let input = parse_input(input);
    find_lowest_location_ranges(
        input
            .seeds
            .chunks_exact(2)
            .map(|pair| {
                let start = pair[0];
                let len = pair[1];
                start..(start + len)
            })
            .collect(),
        &input.maps,
    )
}

aoc_runner::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_runner::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 35, part2 = 46)]
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
                    "seed",
                    Map {
                        target: "soil",
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
                    "soil",
                    Map {
                        target: "fertilizer",
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
                    "fertilizer",
                    Map {
                        target: "water",
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
                    "water",
                    Map {
                        target: "light",
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
                    "light",
                    Map {
                        target: "temperature",
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
                    "temperature",
                    Map {
                        target: "humidity",
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
                    "humidity",
                    Map {
                        target: "location",
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
