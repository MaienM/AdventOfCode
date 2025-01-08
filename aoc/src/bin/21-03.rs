puzzle_lib::setup!(title = "Binary Diagnostic");

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    parse!(input => {
        [nums split on '\n' with [chars as u8]]
    } => nums)
}

fn get_most_common_per_position(nums: &[Vec<u8>]) -> Vec<u8> {
    let mut count_per_pos: Vec<[u32; 2]> = (0..nums[0].len()).map(|_| [0, 0]).collect();
    for num in nums {
        for (idx, bit) in num.iter().enumerate() {
            count_per_pos[idx][*bit as usize] += 1;
        }
    }
    // When there are an equal number of 0 and 1 in a position this position will be set to 1.
    count_per_pos
        .iter()
        .map(|counts| u8::from(counts[1] >= counts[0]))
        .collect()
}

fn bits_to_decimal(bits: &[u8]) -> u32 {
    bits.iter().fold(0, |acc, b| (acc << 1) + u32::from(*b))
}

fn calculate_generator_rating(mut nums: Vec<Vec<u8>>, use_most_common: bool) -> u32 {
    for i in 0..nums[0].len() {
        let most_common = get_most_common_per_position(&nums);
        let target = if use_most_common {
            most_common[i]
        } else {
            1 - most_common[i]
        };
        nums.retain(|bits| bits[i] == target);
        if nums.len() == 1 {
            break;
        }
    }
    bits_to_decimal(&nums[0])
}

pub fn part1(input: &str) -> u32 {
    let nums = parse_input(input);
    let most_common_per_pos = get_most_common_per_position(&nums);

    let gamma = bits_to_decimal(&most_common_per_pos);

    // Epsilon is really just gamma with all bits flipped, so just calculate it that way.
    let mask = 2_u32.pow(most_common_per_pos.len() as u32) - 1;
    let epsilon = gamma ^ mask;

    gamma * epsilon
}

pub fn part2(input: &str) -> u32 {
    let nums = parse_input(input);

    let oxygen = calculate_generator_rating(nums.clone(), true);
    let scrubber = calculate_generator_rating(nums, false);

    oxygen * scrubber
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 198, part2 = 230)]
    static EXAMPLE_INPUT: &str = "
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
        let actual = parse_input(&EXAMPLE_INPUT);
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
        assert_eq!(actual, expected);
    }
}
