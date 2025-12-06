puzzle_runner::register_chapter!(book = "2025", title = "Trash Compactor");

enum Operator {
    Add,
    Mul,
}

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Operator>) {
    let mut lines: Vec<_> = input.lines().collect();
    let input = lines.pop().unwrap();
    parse!(input =>
        [operators chars try match {
            '+' => Some(Operator::Add),
            '*' => Some(Operator::Mul),
            _ => None,
        }]
    );
    let input = lines.join("\n");
    parse!(input =>
        [numbers split on '\n' with [find matches /r"\d+"/ as usize]]
    );
    (numbers, operators)
}

pub fn part1(input: &str) -> usize {
    let (mut numbers, operators) = parse_input(input);
    let mut sums: Vec<_> = numbers.pop().unwrap();
    for line in numbers {
        for (i, num) in line.iter().enumerate() {
            match operators[i] {
                Operator::Add => sums[i] += num,
                Operator::Mul => sums[i] *= num,
            }
        }
    }
    sums.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 4_277_556)]
    static EXAMPLE_INPUT: &str = "
        123 328  51 64 
         45 64  387 23 
          6 98  215 314
        *   +   *   +  
    ";

    // #[test]
    // fn example_parse() {
    //     let actual = parse_input(&EXAMPLE_INPUT);
    //     let expected = vec![1, 2];
    //     assert_eq!(actual, expected);
    // }
}
