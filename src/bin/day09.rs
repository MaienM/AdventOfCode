use aoc::*;

#[derive(Debug, PartialEq)]
struct Grid {
    numbers: Vec<Vec<i8>>,
    width: usize,
    height: usize,
}

fn parse_input(input: String) -> Grid {
    let numbers = input
        .trim()
        .split("\n")
        .map(|line| {
            return line
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect();
        })
        .collect::<Vec<Vec<i8>>>();
    let width = numbers[0].len();
    let height = numbers.len();
    return Grid {
        numbers,
        width,
        height,
    };
}

fn part1(input: String) -> i64 {
    let grid = parse_input(input);
    let mut sum = 0_i64;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let number = grid.numbers[y][x];
            if (x > 0 && number >= grid.numbers[y][x - 1])
                || (x < grid.width - 1 && number >= grid.numbers[y][x + 1])
                || (y > 0 && number >= grid.numbers[y - 1][x])
                || (y < grid.height - 1 && number >= grid.numbers[y + 1][x])
            {
                continue;
            }
            sum += (number + 1) as i64;
        }
    }
    return sum;
}

fn main() {
    run(part1, missing);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = Grid {
            numbers: vec![
                vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
            ],
            width: 10,
            height: 5,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 15);
    }
}
