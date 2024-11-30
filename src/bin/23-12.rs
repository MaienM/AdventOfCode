use std::collections::HashMap;

use aoc::utils::parse;
use rayon::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}
impl From<char> for Condition {
    fn from(value: char) -> Self {
        match value {
            '.' => Condition::Operational,
            '#' => Condition::Damaged,
            '?' => Condition::Unknown,
            _ => panic!("Unknown condition {value:?}."),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Record {
    conditions: Vec<Condition>,
    damaged_groups: Vec<usize>,
}

fn parse_record(line: &str) -> Record {
    parse!(line => {
        [conditions chars as Condition] " " [damaged_groups split on ',' as usize]
    } => Record { conditions, damaged_groups })
}

fn parse_input(input: &str) -> Vec<Record> {
    parse!(input => [records split on '\n' with (parse_record)]);
    records
}

fn find_valid_options(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    record: &mut Record,
    index: usize,
    groups_index: usize,
    current_group_size: usize,
) -> usize {
    let cache_key = (index, groups_index, current_group_size);
    if let Some(result) = cache.get(&cache_key) {
        return *result;
    }

    let condition = match record.conditions.get(index) {
        Some(Condition::Unknown) if current_group_size > 0 => {
            if &current_group_size < record.damaged_groups.get(groups_index).unwrap_or(&0) {
                Some(&Condition::Damaged)
            } else {
                Some(&Condition::Operational)
            }
        }
        None if current_group_size > 0 => Some(&Condition::Operational),
        condition => condition,
    };
    let result = match condition {
        Some(Condition::Operational) => {
            if current_group_size > 0 {
                if Some(&current_group_size) == record.damaged_groups.get(groups_index) {
                    find_valid_options(cache, record, index + 1, groups_index + 1, 0)
                } else {
                    0
                }
            } else {
                find_valid_options(cache, record, index + 1, groups_index, current_group_size)
            }
        }
        Some(Condition::Damaged) => {
            if &current_group_size >= record.damaged_groups.get(groups_index).unwrap_or(&0) {
                0
            } else {
                find_valid_options(
                    cache,
                    record,
                    index + 1,
                    groups_index,
                    current_group_size + 1,
                )
            }
        }
        Some(Condition::Unknown) => {
            find_valid_options(cache, record, index + 1, groups_index, current_group_size)
                + find_valid_options(
                    cache,
                    record,
                    index + 1,
                    groups_index,
                    current_group_size + 1,
                )
        }
        None => usize::from(groups_index == record.damaged_groups.len()),
    };
    cache.insert(cache_key, result);
    result
}

pub fn part1(input: &str) -> usize {
    let records = parse_input(input);
    records
        .into_par_iter()
        .map(|mut record| find_valid_options(&mut HashMap::new(), &mut record, 0, 0, 0))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let records = parse_input(input);

    records
        .into_par_iter()
        .map(|record| {
            let mut conditions = Vec::new();
            conditions.append(&mut record.conditions.clone());
            conditions.push(Condition::Unknown);
            conditions.append(&mut record.conditions.clone());
            conditions.push(Condition::Unknown);
            conditions.append(&mut record.conditions.clone());
            conditions.push(Condition::Unknown);
            conditions.append(&mut record.conditions.clone());
            conditions.push(Condition::Unknown);
            conditions.append(&mut record.conditions.clone());

            let mut damaged_groups = Vec::new();
            damaged_groups.append(&mut record.damaged_groups.clone());
            damaged_groups.append(&mut record.damaged_groups.clone());
            damaged_groups.append(&mut record.damaged_groups.clone());
            damaged_groups.append(&mut record.damaged_groups.clone());
            damaged_groups.append(&mut record.damaged_groups.clone());

            let mut record = Record {
                conditions,
                damaged_groups,
            };

            find_valid_options(&mut HashMap::new(), &mut record, 0, 0, 0)
        })
        .sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 21, part2 = 525_152)]
    static EXAMPLE_INPUT: &str = "
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    ";

    #[example_input(part1 = 1, part2 = 1)]
    static EXAMPLE_INPUT_LINE1: &str = "
        ???.### 1,1,3
    ";

    #[example_input(part1 = 4, part2 = 16_384)]
    static EXAMPLE_INPUT_LINE2: &str = "
        .??..??...?##. 1,1,3
    ";

    #[example_input(part1 = 1, part2 = 1)]
    static EXAMPLE_INPUT_LINE3: &str = "
        ?#?#?#?#?#?#?#? 1,3,1,6
    ";

    #[example_input(part1 = 1, part2 = 16)]
    static EXAMPLE_INPUT_LINE4: &str = "
        ????.#...#... 4,1,1
    ";

    #[example_input(part1 = 4, part2 = 2500)]
    static EXAMPLE_INPUT_LINE5: &str = "
        ????.######..#####. 1,6,5
    ";

    #[example_input(part1 = 10, part2 = 506_250)]
    static EXAMPLE_INPUT_LINE6: &str = "
        ?###???????? 3,2,1
    ";

    #[test]
    #[allow(clippy::too_many_lines)]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            Record {
                conditions: vec![
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Operational,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                ],
                damaged_groups: vec![1, 1, 3],
            },
            Record {
                conditions: vec![
                    Condition::Operational,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Operational,
                    Condition::Operational,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Operational,
                    Condition::Operational,
                    Condition::Operational,
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Operational,
                ],
                damaged_groups: vec![1, 1, 3],
            },
            Record {
                conditions: vec![
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Unknown,
                ],
                damaged_groups: vec![1, 3, 1, 6],
            },
            Record {
                conditions: vec![
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Operational,
                    Condition::Damaged,
                    Condition::Operational,
                    Condition::Operational,
                    Condition::Operational,
                    Condition::Damaged,
                    Condition::Operational,
                    Condition::Operational,
                    Condition::Operational,
                ],
                damaged_groups: vec![4, 1, 1],
            },
            Record {
                conditions: vec![
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Operational,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Operational,
                    Condition::Operational,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Operational,
                ],
                damaged_groups: vec![1, 6, 5],
            },
            Record {
                conditions: vec![
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                ],
                damaged_groups: vec![3, 2, 1],
            },
        ];
        assert_eq!(actual, expected);
    }
}
