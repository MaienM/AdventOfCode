use aoc::grid::Grid;
use aoc::runner::*;

fn parse_input(input: String) -> Grid {
    let grid = input
        .trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!("Invalid character {}.", c),
                })
                .collect()
        })
        .collect();
    return Grid::new(grid).unwrap();
}

fn get_most_common_per_position(grid: &Grid) -> Vec<u32> {
    let mut count_per_pos: Vec<[u32; 2]> = (0..grid.width).map(|_| [0, 0]).collect();
    for (point, bit) in grid.by_cell() {
        count_per_pos[point.x][bit.to_owned() as usize] += 1;
    }
    return count_per_pos
        .iter()
        .map(|counts| {
            if counts[0] > counts[1] {
                return 0;
            } else {
                return 1;
            }
        })
        .collect();
}

fn bit_list_to_decimal(bits: &Vec<u32>) -> u32 {
    let mut result = 0;
    for bit in bits {
        result = result << 1;
        result += bit;
    }
    return result;
}

pub fn part1(input: String) -> i64 {
    let grid = parse_input(input);
    let most_common_per_pos = get_most_common_per_position(&grid);

    let gamma = bit_list_to_decimal(&most_common_per_pos);
    // Epsilon is really just gamma with all bits flipped, so just calculate it that way.
    let mask = (2 as u32).pow(most_common_per_pos.len() as u32) - 1;
    let epsilon = gamma ^ mask;

    return (gamma * epsilon).into();
}

pub fn part2(input: String) -> i64 {
    let grid = parse_input(input);

    let mut oxygen_candidates = grid.clone();
    let mut scrubber_candidates = grid.clone();

    for i in 0..grid.width {
        let oxygen_most_common = get_most_common_per_position(&oxygen_candidates);
        let oxygen_criteria = oxygen_most_common[i];
        oxygen_candidates = oxygen_candidates
            .into_iter()
            .filter(|bits| bits[i] == oxygen_criteria)
            .collect();
        if oxygen_candidates.height == 1 {
            break;
        }
    }

    for i in 0..grid.width {
        let scrubber_most_common = get_most_common_per_position(&scrubber_candidates);
        let scrubber_criteria = 1 - scrubber_most_common[i];
        scrubber_candidates = scrubber_candidates
            .into_iter()
            .filter(|bits| bits[i] == scrubber_criteria)
            .collect();
        if scrubber_candidates.height == 1 {
            break;
        }
    }

    let oxygen = bit_list_to_decimal(&oxygen_candidates.into_iter().next().unwrap());
    let scrubber = bit_list_to_decimal(&scrubber_candidates.into_iter().next().unwrap());

    return (oxygen * scrubber).into();
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = vec![
            vec![0, 0, 1, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 1, 1, 0],
            vec![1, 0, 1, 1, 1],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 1, 1, 1],
            vec![0, 0, 1, 1, 1],
            vec![1, 1, 1, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 1],
            vec![0, 0, 0, 1, 0],
            vec![0, 1, 0, 1, 0],
        ];
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 198);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 230);
    }
}
