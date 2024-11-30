use std::collections::VecDeque;

use aoc::utils::parse;

fn parse_input(input: &str) -> Vec<i64> {
    parse!(input => { [numbers split on '\n' as i64] } => numbers)
}

fn mix(numbers: Vec<i64>, times: usize) -> Vec<i64> {
    let mut mixing: VecDeque<(usize, i64)> = numbers.into_iter().enumerate().collect();
    let len = mixing.len();
    for _ in 0..times {
        for i in 0..len {
            let (idx, offset) = mixing
                .iter()
                .enumerate()
                .find(|(_, (idx, _))| idx == &i)
                .map(|(idx, (_, n))| (idx as i64, n))
                .unwrap();

            let mut new_idx = (idx + offset) % (len as i64 - 1);
            while new_idx < 0 {
                new_idx += len as i64 - 1;
            }

            if idx != new_idx {
                let item = mixing.remove(idx as usize).unwrap();
                mixing.insert(new_idx as usize, item);
            }
        }
    }
    mixing.into_iter().map(|(_i, n)| n).collect()
}

fn get_coordinates(numbers: &[i64]) -> i64 {
    let offset = numbers
        .iter()
        .enumerate()
        .find(|(_, n)| n == &&0)
        .unwrap()
        .0;
    let len = numbers.len();
    numbers[(1000 + offset) % len] + numbers[(2000 + offset) % len] + numbers[(3000 + offset) % len]
}

pub fn part1(input: &str) -> i64 {
    let numbers = parse_input(input);
    let numbers = mix(numbers, 1);
    get_coordinates(&numbers)
}

pub fn part2(input: &str) -> i64 {
    let numbers = parse_input(input);
    let numbers = numbers.into_iter().map(|n| n * 811_589_153).collect();
    let numbers = mix(numbers, 10);
    get_coordinates(&numbers)
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 3, part2 = 1_623_178_306)]
    static EXAMPLE_INPUT: &str = "
        1
        2
        -3
        3
        -2
        0
        4
    ";

    fn zerofirst(mut numbers: Vec<i64>) -> Vec<i64> {
        let idx = numbers
            .iter()
            .enumerate()
            .find(|(_, n)| n == &&0)
            .unwrap()
            .0;
        let mut zerofirst = numbers.split_off(idx);
        zerofirst.append(&mut numbers);
        zerofirst
    }

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![1, 2, -3, 3, -2, 0, 4];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_zerofirst() {
        let actual = zerofirst(vec![1, 2, -3, 3, -2, 0, 4]);
        let expected = vec![0, 4, 1, 2, -3, 3, -2];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_mix() {
        let actual = zerofirst(mix(vec![1, 2, -3, 3, -2, 0, 4], 1));
        let expected = vec![0, 3, -2, 1, 2, -3, 4];
        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn example_mix_multi() {
        let start = vec![
            811_589_153,
            1_623_178_306,
            -2_434_767_459,
            2_434_767_459,
            -1_623_178_306,
            0,
            3_246_356_612,
        ];
        assert_eq!(
            zerofirst(mix(start.clone(), 1)),
            vec![
                0,
                -2_434_767_459,
                3_246_356_612,
                -1_623_178_306,
                2_434_767_459,
                1_623_178_306,
                811_589_153,
            ]
        );
        assert_eq!(
            zerofirst(mix(start.clone(), 2)),
            vec![
                0,
                2_434_767_459,
                1_623_178_306,
                3_246_356_612,
                -2_434_767_459,
                -1_623_178_306,
                811_589_153,
            ]
        );
        assert_eq!(
            zerofirst(mix(start.clone(), 3)),
            vec![
                0,
                811_589_153,
                2_434_767_459,
                3_246_356_612,
                1_623_178_306,
                -1_623_178_306,
                -2_434_767_459,
            ]
        );
        assert_eq!(
            zerofirst(mix(start.clone(), 4)),
            vec![
                0,
                1_623_178_306,
                -2_434_767_459,
                811_589_153,
                2_434_767_459,
                3_246_356_612,
                -1_623_178_306,
            ]
        );
        assert_eq!(
            zerofirst(mix(start.clone(), 5)),
            vec![
                0,
                811_589_153,
                -1_623_178_306,
                1_623_178_306,
                -2_434_767_459,
                3_246_356_612,
                2_434_767_459,
            ]
        );
        assert_eq!(
            zerofirst(mix(start.clone(), 6)),
            vec![
                0,
                811_589_153,
                -1_623_178_306,
                3_246_356_612,
                -2_434_767_459,
                1_623_178_306,
                2_434_767_459,
            ]
        );
        assert_eq!(
            zerofirst(mix(start.clone(), 7)),
            vec![
                0,
                -2_434_767_459,
                2_434_767_459,
                1_623_178_306,
                -1_623_178_306,
                811_589_153,
                3_246_356_612,
            ]
        );
        assert_eq!(
            zerofirst(mix(start.clone(), 8)),
            vec![
                0,
                1_623_178_306,
                3_246_356_612,
                811_589_153,
                -2_434_767_459,
                2_434_767_459,
                -1_623_178_306,
            ]
        );
        assert_eq!(
            zerofirst(mix(start.clone(), 9)),
            vec![
                0,
                811_589_153,
                1_623_178_306,
                -2_434_767_459,
                3_246_356_612,
                2_434_767_459,
                -1_623_178_306,
            ]
        );
        assert_eq!(
            zerofirst(mix(start, 10)),
            vec![
                0,
                -2_434_767_459,
                1_623_178_306,
                3_246_356_612,
                -1_623_178_306,
                2_434_767_459,
                811_589_153,
            ]
        );
    }
}
