use aoc::grid::Grid as BaseGrid;
use aoc::*;

type Grid = BaseGrid<i8>;

fn parse_input(input: String) -> Grid {
    return input
        .trim()
        .split("\n")
        .map(|line| {
            return line
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<i8>>();
        })
        .collect::<Grid>();
}

fn part1(input: String) -> i64 {
    let grid = parse_input(input);
    let mut sum = 0_i64;
    for (point, value) in grid.by_cell() {
        if (point.x > 0 && value >= grid.get(point.x - 1, point.y).unwrap())
            || (point.x < grid.width - 1 && value >= grid.get(point.x + 1, point.y).unwrap())
            || (point.y > 0 && value >= grid.get(point.x, point.y - 1).unwrap())
            || (point.y < grid.height - 1 && value >= grid.get(point.x, point.y + 1).unwrap())
        {
            continue;
        }
        sum += (value + 1) as i64;
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
        let expected = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]
        .into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 15);
    }
}
