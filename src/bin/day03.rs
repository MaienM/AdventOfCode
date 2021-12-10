use aoc::*;

fn parse_grid_of_bits(input: String) -> Vec<Vec<i32>> {
    return input
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
}

fn part1(input: String) -> i32 {
    let grid = parse_grid_of_bits(input);
    let mut count_per_pos: Vec<[i32; 2]> = grid.first().unwrap().iter().map(|_| [0, 0]).collect();
    let columns = count_per_pos.len();
    for line in grid {
        for (i, bit) in line.iter().enumerate() {
            count_per_pos[i][bit.to_owned() as usize] += 1;
        }
    }

    let mut gamma = 0;
    for counts in count_per_pos {
        gamma = gamma << 1;
        if counts[1] > counts[0] {
            gamma += 1;
        }
    }

    // Epsilon is really just gamma with all bits flipped, so just calculate it that way.
    let filled_bits = (2 as i32).pow(columns as u32) - 1;
    let epsilon = gamma ^ filled_bits;

    return gamma * epsilon;
}

fn main() {
    run(part1, missing);
}

#[cfg(test)]
mod tests {
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
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 198);
    }
}
