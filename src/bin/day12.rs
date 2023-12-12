use aoc::utils::parse::splitn;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
struct Record {
    conditions: Vec<Condition>,
    damaged_groups: Vec<usize>,
}

fn parse_input(input: &str) -> Vec<Record> {
    input
        .split('\n')
        .map(|line| {
            let (conditions, groups) = splitn!(line, ' ', str, str);
            Record {
                conditions: conditions.chars().map(Condition::from).collect(),
                damaged_groups: groups.split(',').map(|s| s.parse().unwrap()).collect(),
            }
        })
        .collect()
}

fn _find_valid_options(record: &mut Record, indexes: &[usize], index_of_indexes: usize) -> usize {
    if let Some(index) = indexes.get(index_of_indexes) {
        record.conditions[*index] = Condition::Operational;
        let count_a = _find_valid_options(record, indexes, index_of_indexes + 1);
        record.conditions[*index] = Condition::Damaged;
        let count_b = _find_valid_options(record, indexes, index_of_indexes + 1);
        count_a + count_b
    } else {
        let mut groups = Vec::new();
        let mut last = &Condition::Operational;
        let mut len = 0;
        for condition in &record.conditions {
            match (last, condition) {
                (Condition::Damaged, Condition::Operational) => {
                    groups.push(len);
                    len = 0;
                }
                (_, Condition::Damaged) => {
                    len += 1;
                }
                _ => {}
            }
            last = condition;
        }
        if len > 0 {
            groups.push(len);
        }
        usize::from(groups == record.damaged_groups)
    }
}

fn find_valid_options(record: &mut Record) -> usize {
    let indexes: Vec<_> = record
        .conditions
        .iter()
        .enumerate()
        .filter(|(_, c)| c == &&Condition::Unknown)
        .map(|(i, _)| i)
        .collect();
    _find_valid_options(record, &indexes, 0)
}

pub fn part1(input: &str) -> usize {
    let records = parse_input(input);
    records
        .into_iter()
        .map(|mut record| find_valid_options(&mut record))
        .sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 21, test)]
    static EXAMPLE_INPUT: &str = "
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
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
