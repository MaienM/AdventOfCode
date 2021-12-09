use aoc::*;

fn run(input: String) -> String {
    let numbers = parse_list_of_numbers(input);
    let mut count = 0;
    let mut last_numbers = [numbers[0], numbers[1], numbers[2]];
    let mut last_sum: i32 = last_numbers.iter().sum();
    for number in numbers {
        last_numbers = [last_numbers[1], last_numbers[2], number];
        let sum = last_numbers.iter().sum();
        if sum > last_sum {
            count = count + 1;
        }
        last_sum = sum;
    }
    return count.to_string();
}

fn main() {
    dorun(run);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "
            199
            200
            208
            210
            200
            207
            240
            269
            260
            263
        ";
        assert_eq!(run(input.to_string()), "5");
    }
}
