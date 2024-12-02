use std::mem;

use aoc::utils::parse;

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    parse!(input => {
        [reports split on '\n' with [split as usize]]
    } => reports)
}

fn orient(report: &mut [usize]) {
    if report.first() > report.last() {
        report.reverse();
    }
}

fn is_safe(report: &[usize]) -> Option<()> {
    let iter = report.iter().zip(report.iter().skip(1));
    for (l, r) in iter {
        if l >= r || (r - l) > 3 {
            return None;
        }
    }
    Some(())
}

fn is_safe_dampened(report: &mut Vec<usize>) -> Option<()> {
    let mut swap = report.pop().unwrap();
    if is_safe(report).is_some() {
        return Some(());
    }

    for i in (0..report.len()).rev() {
        mem::swap(&mut report[i], &mut swap);
        if is_safe(report).is_some() {
            return Some(());
        }
    }
    None
}

pub fn part1(input: &str) -> usize {
    let mut reports = parse_input(input);
    reports
        .iter_mut()
        .filter_map(|r| {
            orient(r);
            is_safe(r)
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    let mut reports = parse_input(input);
    reports
        .iter_mut()
        .filter_map(|r| {
            orient(r);
            is_safe_dampened(r)
        })
        .count()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 2, part2 = 4)]
    static EXAMPLE_INPUT: &str = "
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(actual, expected);
    }
}
