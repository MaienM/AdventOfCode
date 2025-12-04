puzzle_runner::register_chapter!(book = "2022", title = "Monkey Math");

use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Add,
    Rem,
    Mul,
    Div,
}
impl Operation {
    fn apply(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Rem => lhs - rhs,
            Operation::Mul => lhs * rhs,
            Operation::Div => lhs / rhs,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Job<'a> {
    Number(u64),
    Operation(&'a str, Operation, &'a str),
}

fn parse_input(input: &str) -> HashMap<&str, Job<'_>> {
    parse!(input => {
        [monkeys split on '\n' into (HashMap<_, _>) with
            { name ": " [job split] }
            => {
                let job = match job[..] {
                    [num] => Job::Number(num.parse().unwrap()),
                    [lhs, op, rhs] => Job::Operation(
                        lhs,
                        match op {
                            "+" => Operation::Add,
                            "-" => Operation::Rem,
                            "*" => Operation::Mul,
                            "/" => Operation::Div,
                            _ => panic!(),
                        },
                        rhs,
                    ),
                    _ => panic!(),
                };
                (name, job)
            }
        ]
    } => monkeys)
}

pub fn part1(input: &str) -> u64 {
    let mut jobs = parse_input(input);
    let mut results: HashMap<&str, u64> = HashMap::new();

    // Move all numbers to the results.
    jobs.retain(|name, job| match job {
        Job::Number(num) => {
            results.insert(name, *num);
            false
        }
        Job::Operation(..) => true,
    });

    // Perform calcualtions until none are lhs.
    while !jobs.is_empty() {
        jobs.retain(|name, job| match job {
            Job::Operation(lhs, operation, rhs) => {
                if results.contains_key(lhs) && results.contains_key(rhs) {
                    let lhs = *results.get(lhs).unwrap();
                    let rhs = *results.get(rhs).unwrap();
                    let num = operation.apply(lhs, rhs);
                    results.insert(name, num);
                    return false;
                }
                true
            }
            Job::Number(_) => panic!(),
        });
    }

    *results.get("root").unwrap()
}

pub fn part2(input: &str) -> u64 {
    let mut jobs = parse_input(input);
    let mut results: HashMap<&str, u64> = HashMap::new();

    // Take out root and humn.
    let root = jobs.remove(&"root").unwrap();
    let humn = jobs.remove(&"humn").unwrap();

    // Take out numbers.
    jobs.retain(|name, job| match job {
        Job::Number(num) => {
            results.insert(name, *num);
            false
        }
        Job::Operation(..) => true,
    });

    // Do all the calculations that can be done now.
    loop {
        let before = jobs.len();
        jobs.retain(|name, job| match job {
            Job::Operation(lhs, operation, rhs) => {
                if results.contains_key(lhs) && results.contains_key(rhs) {
                    let lhs = *results.get(lhs).unwrap();
                    let rhs = *results.get(rhs).unwrap();
                    let num = operation.apply(lhs, rhs);
                    results.insert(name, num);
                    return false;
                }
                true
            }
            Job::Number(_) => panic!(),
        });
        if jobs.len() == before {
            break;
        }
    }

    // One of the inputs of root should be known. Set the other one to the same value and start
    // working backwards.
    jobs.insert("humn", humn);
    let mut current = match root {
        Job::Operation(lhs, _, rhs) => {
            if results.contains_key(lhs) {
                (*results.get(lhs).unwrap(), jobs.remove_entry(rhs).unwrap())
            } else {
                (*results.get(rhs).unwrap(), jobs.remove_entry(lhs).unwrap())
            }
        }
        Job::Number(_) => panic!(),
    };
    loop {
        current = match current {
            (result, ("humn", _)) => {
                return result;
            }
            (wanted_result, (_, Job::Operation(lhs, operation, rhs))) => {
                match (results.get(lhs), operation, results.get(rhs)) {
                    (Option::Some(lhs), Operation::Add, Option::None) => {
                        (wanted_result - lhs, jobs.remove_entry(rhs).unwrap())
                    }
                    (Option::None, Operation::Add, Option::Some(rhs)) => {
                        (wanted_result - rhs, jobs.remove_entry(lhs).unwrap())
                    }
                    (Option::Some(lhs), Operation::Rem, Option::None) => {
                        (lhs - wanted_result, jobs.remove_entry(rhs).unwrap())
                    }
                    (Option::None, Operation::Rem, Option::Some(rhs)) => {
                        (wanted_result + rhs, jobs.remove_entry(lhs).unwrap())
                    }
                    (Option::Some(lhs), Operation::Mul, Option::None) => {
                        (wanted_result / lhs, jobs.remove_entry(rhs).unwrap())
                    }
                    (Option::None, Operation::Mul, Option::Some(rhs)) => {
                        (wanted_result / rhs, jobs.remove_entry(lhs).unwrap())
                    }
                    (Option::Some(lhs), Operation::Div, Option::None) => {
                        (lhs / wanted_result, jobs.remove_entry(rhs).unwrap())
                    }
                    (Option::None, Operation::Div, Option::Some(rhs)) => {
                        (wanted_result * rhs, jobs.remove_entry(lhs).unwrap())
                    }
                    expr => panic!("{expr:?}"),
                }
            }
            _ => panic!("{current:?}"),
        };
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 152, part2 = 301)]
    static EXAMPLE_INPUT: &str = "
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
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
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
        ]
        .into_iter()
        .collect();
        assert_eq!(actual, expected);
    }
}
