use aoc::utils::parse;

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    parse!(input => {
        [reports split on '\n' with [split as usize]]
    } => reports)
}

fn is_safe(report: &Vec<usize>) -> bool {
    let mut report = report.clone();
    if report.first() > report.last() {
        report.reverse();
    }
    let iter = report.iter().zip(report.iter().skip(1));
    for (l, r) in iter {
        if l >= r || (r - l) > 3 {
            return false;
        }
    }
    true
}

pub fn part1(input: &str) -> usize {
    let reports = parse_input(input);
    reports.into_iter().filter(is_safe).count()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 2)]
    static EXAMPLE_INPUT: &str = "
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    ";

    // #[test]
    // fn example_parse() {
    //     let actual = parse_input(&EXAMPLE_INPUT);
    //     let expected = vec![1, 2];
    //     assert_eq!(actual, expected);
    // }
}
