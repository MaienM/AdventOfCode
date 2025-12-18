puzzle_runner::register_chapter!(title = "Elves Look, Elves Say");

fn parse_input(input: &str) -> Vec<u8> {
    parse!(input => { [nums chars as u8] } => nums)
}

fn look_and_say(nums: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut iter = nums.into_iter();

    let mut current = iter.next().unwrap();
    let mut count = 1;
    for next in iter {
        if next == current {
            count += 1;
        } else {
            result.extend([count, current]);
            current = next;
            count = 1;
        }
    }
    result.extend([count, current]);
    result
}

#[register_part]
fn part1(input: &str) -> usize {
    let mut nums = parse_input(input);
    for _ in 0..40 {
        nums = look_and_say(nums);
    }
    nums.len()
}

#[register_part]
fn part2(input: &str) -> usize {
    let mut nums = parse_input(input);
    for _ in 0..50 {
        nums = look_and_say(nums);
    }
    nums.len()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input]
    static EXAMPLE_INPUT: &str = "1";

    #[test]
    fn example_times_5() {
        let mut nums = parse_input(&EXAMPLE_INPUT);
        assert_eq!(nums, vec![1]);
        nums = look_and_say(nums);
        assert_eq!(nums, vec![1, 1]);
        nums = look_and_say(nums);
        assert_eq!(nums, vec![2, 1]);
        nums = look_and_say(nums);
        assert_eq!(nums, vec![1, 2, 1, 1]);
        nums = look_and_say(nums);
        assert_eq!(nums, vec![1, 1, 1, 2, 2, 1]);
        nums = look_and_say(nums);
        assert_eq!(nums, vec![3, 1, 2, 2, 1, 1]);
    }
}
