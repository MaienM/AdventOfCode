use aoc::runner::*;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Add,
    Rem,
    Mul,
    Div,
}

#[derive(Debug, Eq, PartialEq)]
enum Job<'a> {
    Number(u64),
    Operation(&'a str, Operation, &'a str),
}

fn parse_input<'a>(input: &'a str) -> Vec<(&'a str, Job<'a>)> {
    return input
        .trim()
        .split("\n")
        .map(|line| {
            let mut parts = line.trim().split(" ");
            let name = parts.next().unwrap().strip_suffix(':').unwrap();
            let job = match (parts.next(), parts.next(), parts.next()) {
                (Option::Some(num), Option::None, Option::None) => {
                    Job::Number(num.parse().unwrap())
                }
                (Option::Some(left), Option::Some(operation), Option::Some(right)) => {
                    Job::Operation(
                        left,
                        match operation {
                            "+" => Operation::Add,
                            "-" => Operation::Rem,
                            "*" => Operation::Mul,
                            "/" => Operation::Div,
                            _ => panic!(),
                        },
                        right,
                    )
                }
                _ => panic!(),
            };
            return (name, job);
        })
        .collect();
}

pub fn part1(input: String) -> u64 {
    let mut jobs = parse_input(input.as_str());
    let mut results: HashMap<&str, u64> = HashMap::new();

    jobs.retain(|(name, job)| {
        match job {
            Job::Number(num) => {
                results.insert(name, *num);
                return false;
            }
            _ => return true,
        };
    });

    while !jobs.is_empty() {
        jobs.retain(|(name, job)| match job {
            Job::Operation(left, operation, right) => {
                if results.contains_key(left) && results.contains_key(right) {
                    let left = *results.get(left).unwrap();
                    let right = *results.get(right).unwrap();
                    let num = match operation {
                        Operation::Add => left + right,
                        Operation::Rem => left - right,
                        Operation::Mul => left * right,
                        Operation::Div => left / right,
                    };
                    results.insert(name, num);
                    return false;
                }
                return true;
            }
            _ => panic!(),
        });
    }

    return *results.get("root").unwrap();
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT);
        let mut expected = vec![
            ("root", Job::Operation("pppw", Operation::Add, "sjmn")),
            ("dbpl", Job::Number(5)),
            ("cczh", Job::Operation("sllz", Operation::Add, "lgvd")),
            ("zczc", Job::Number(2)),
            ("ptdq", Job::Operation("humn", Operation::Rem, "dvpt")),
            ("dvpt", Job::Number(3)),
            ("lfqf", Job::Number(4)),
            ("humn", Job::Number(5)),
            ("ljgn", Job::Number(2)),
            ("sjmn", Job::Operation("drzm", Operation::Mul, "dbpl")),
            ("sllz", Job::Number(4)),
            ("pppw", Job::Operation("cczh", Operation::Div, "lfqf")),
            ("lgvd", Job::Operation("ljgn", Operation::Mul, "ptdq")),
            ("drzm", Job::Operation("hmdt", Operation::Rem, "zczc")),
            ("hmdt", Job::Number(32)),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 152);
    }
}
