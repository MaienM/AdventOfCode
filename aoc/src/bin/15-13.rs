puzzle_runner::register_chapter!(book = 2015, title = "Knights of the Dinner Table");

use std::collections::HashMap;

type Rule<'a> = (&'a str, &'a str, i16);

fn parse_input(input: &str) -> Vec<Rule<'_>> {
    parse!(input => {
        [rules split on '\n' with
            {
                left
                " would "
                [mul match {
                    "gain" => 1,
                    "lose" => -1,
                }]
                ' '
                [delta as i16]
                " happiness units by sitting next to "
                right
                '.'
            }
            => (left, right, mul * delta)
        ]
    } => rules)
}

fn solve(rules: Vec<Rule>) -> i16 {
    let mut names: Vec<_> = rules.iter().map(|r| r.0).unique().collect();
    let edges: HashMap<_, _> = rules.into_iter().map(|(l, r, d)| ((l, r), d)).collect();

    let first = names.pop().unwrap();
    let len = names.len();
    names
        .into_iter()
        .permutations(len)
        .map(|rest| {
            let mut score = 0;
            let mut prev = first;
            for next in rest {
                score +=
                    edges.get(&(prev, next)).unwrap_or(&0) + edges.get(&(next, prev)).unwrap_or(&0);
                prev = next;
            }
            score +=
                edges.get(&(first, prev)).unwrap_or(&0) + edges.get(&(prev, first)).unwrap_or(&0);
            score
        })
        .max()
        .unwrap()
}

#[register_part]
fn part1(input: &str) -> i16 {
    let rules = parse_input(input);
    solve(rules)
}

#[register_part]
fn part2(input: &str) -> i16 {
    let mut rules = parse_input(input);
    rules.push(("Me", rules.first().unwrap().0, 0));
    solve(rules)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 330)]
    static EXAMPLE_INPUT: &str = "
        Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol.
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            ("Alice", "Bob", 54),
            ("Alice", "Carol", -79),
            ("Alice", "David", -2),
            ("Bob", "Alice", 83),
            ("Bob", "Carol", -7),
            ("Bob", "David", -63),
            ("Carol", "Alice", -62),
            ("Carol", "Bob", 60),
            ("Carol", "David", 55),
            ("David", "Alice", 46),
            ("David", "Bob", -7),
            ("David", "Carol", 41),
        ];
        assert_eq!(actual, expected);
    }
}
