use aoc::*;

fn run(input: String) -> String {
    let numbers = parse_list_of_numbers(input);
    let mut count = 0;
    let mut last_number = numbers[0];
    for number in numbers {
        if number > last_number {
            count = count + 1;
        }
        last_number = number;
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
        assert_eq!(run(input.to_string()), "7");
    }
}
