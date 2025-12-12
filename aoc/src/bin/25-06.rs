puzzle_runner::register_chapter!(book = 2025, title = "Trash Compactor");

#[derive(Debug, Eq, PartialEq)]
enum Operator {
    Add,
    Mul,
}

type Problem<'a> = (Vec<&'a str>, Operator);

fn parse_input(input: &str) -> Vec<Problem<'_>> {
    let lines: Vec<_> = input.lines().collect();
    let indexes: Vec<_> = (0..(lines.first().unwrap().len()))
        .filter(|i| lines.iter().all(|line| &line[*i..=*i] == " "))
        .collect();
    let mut rows: Vec<_> = lines
        .into_iter()
        .map(move |mut line| {
            let mut row = Vec::new();
            let mut start = 0;
            for idx in &indexes {
                let (next, rest) = line.split_at(idx - start);
                row.push(next);
                line = &rest[1..];
                start = *idx + 1;
            }
            row.push(line);
            row
        })
        .collect();

    let mut problems = Vec::new();
    loop {
        let mut problem: Vec<_> = rows.iter_mut().filter_map(Vec::pop).collect();
        if problem.is_empty() {
            break;
        }

        let operator = match problem.pop().unwrap().trim() {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            v => invalid!(operator v),
        };
        problems.push((problem, operator));
    }
    problems
}

#[register_part]
fn part1(input: &str) -> usize {
    let problems = parse_input(input);
    problems
        .into_iter()
        .map(|(nums, operator)| {
            nums.into_iter()
                .map(|num| num.trim().parse::<usize>().unwrap())
                .reduce(|a, b| match operator {
                    Operator::Add => a + b,
                    Operator::Mul => a * b,
                })
                .unwrap()
        })
        .sum()
}

#[register_part]
fn part2(input: &str) -> usize {
    let problems = parse_input(input);
    problems
        .into_iter()
        .map(|(nums, operator)| {
            let mut nums: Vec<_> = nums.into_iter().map(str::chars).collect();
            let mut sum = match operator {
                Operator::Add => 0,
                Operator::Mul => 1,
            };
            loop {
                let num = nums
                    .iter_mut()
                    .filter_map(Iterator::next)
                    .filter_map(|c| c.to_digit(10))
                    .reduce(|a, b| a * 10 + b);
                let Some(num) = num else {
                    break;
                };
                let num = num as usize;
                match operator {
                    Operator::Add => sum += num,
                    Operator::Mul => sum *= num,
                }
            }
            sum
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 4_277_556, part2 = 3_263_827)]
    static EXAMPLE_INPUT: &str = "
        123 328  51 64 
         45 64  387 23 
          6 98  215 314
        *   +   *   +  
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            (vec!["64 ", "23 ", "314"], Operator::Add),
            (vec![" 51", "387", "215"], Operator::Mul),
            (vec!["328", "64 ", "98 "], Operator::Add),
            (vec!["123", " 45", "  6"], Operator::Mul),
        ];
        assert_eq!(actual, expected);
    }
}
