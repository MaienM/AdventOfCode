puzzle_runner::register_chapter!(book = "2023", title = "Lens Library");

const EMPTY_VEC: Vec<Lens> = Vec::new();

#[derive(Debug, PartialEq)]
struct Step<'a> {
    label: &'a str,
    operation: Operation,
}
#[derive(Debug, PartialEq)]
enum Operation {
    Remove,
    Place(usize),
}

struct Lens<'a> {
    label: &'a str,
    focal: usize,
}

fn parse_input(input: &str) -> Vec<&str> {
    parse!(input => {
        [steps split on ',']
    } => steps)
}

fn parse_step(input: &str) -> Step<'_> {
    if input.contains('=') {
        parse!(input => { label '=' [focal as usize] } => Step { label, operation: Operation::Place(focal) })
    } else {
        parse!(input => { label '-' } => Step { label, operation: Operation::Remove })
    }
}

fn hash(value: &str) -> usize {
    let mut hash = 0;
    for chr in value.chars() {
        hash += chr as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

#[register_part]
fn part1(input: &str) -> usize {
    let steps = parse_input(input);
    steps.into_iter().map(hash).sum()
}

#[register_part]
fn part2(input: &str) -> usize {
    let steps = parse_input(input).into_iter().map(parse_step);
    let mut boxes: [Vec<Lens>; 256] = [EMPTY_VEC; 256];
    for step in steps {
        let currbox = &mut boxes[hash(step.label)];
        let idx = currbox.iter().enumerate().find_map(|(idx, lens)| {
            if lens.label == step.label {
                Some(idx)
            } else {
                None
            }
        });
        match (step.operation, idx) {
            (Operation::Remove, None) => {}
            (Operation::Remove, Some(idx)) => {
                currbox.remove(idx);
            }
            (Operation::Place(focal), _) => {
                let lens = Lens {
                    label: step.label,
                    focal,
                };
                match idx {
                    Some(idx) => currbox[idx] = lens,
                    None => currbox.push(lens),
                }
            }
        }
    }
    boxes
        .into_iter()
        .enumerate()
        .map(|(boxidx, currbox)| {
            (boxidx + 1)
                * currbox
                    .into_iter()
                    .enumerate()
                    .map(|(lensidx, lens)| (lensidx + 1) * lens.focal)
                    .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 1320, part2 = 145)]
    static EXAMPLE_INPUT: &str = "
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            "rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7",
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_parse_step() {
        let actual: Vec<_> = parse_input(&EXAMPLE_INPUT)
            .into_iter()
            .map(parse_step)
            .collect();
        let expected = vec![
            Step {
                label: "rn",
                operation: Operation::Place(1),
            },
            Step {
                label: "cm",
                operation: Operation::Remove,
            },
            Step {
                label: "qp",
                operation: Operation::Place(3),
            },
            Step {
                label: "cm",
                operation: Operation::Place(2),
            },
            Step {
                label: "qp",
                operation: Operation::Remove,
            },
            Step {
                label: "pc",
                operation: Operation::Place(4),
            },
            Step {
                label: "ot",
                operation: Operation::Place(9),
            },
            Step {
                label: "ab",
                operation: Operation::Place(5),
            },
            Step {
                label: "pc",
                operation: Operation::Remove,
            },
            Step {
                label: "pc",
                operation: Operation::Place(6),
            },
            Step {
                label: "ot",
                operation: Operation::Place(7),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn hash() {
        assert_eq!(super::hash("HASH"), 52);
    }
}
