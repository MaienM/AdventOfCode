use aoc::*;

fn mean(numbers: &Vec<i32>) -> i32 {
    let mut sorted = numbers.clone();
    sorted.sort();
    let len = sorted.len();
    if len % 2 == 0 {
        let avg = f64::from(sorted[len / 2 - 1] + sorted[len / 2]) / 2.0;
        return avg.round() as i32;
    } else {
        return sorted[len / 2];
    }
}

fn part1(input: String) -> i64 {
    let positions = parse_number_list(input, ",");
    let target = mean(&positions);
    return positions
        .iter()
        .map(|p| (target - p).abs())
        .sum::<i32>()
        .into();
}

fn main() {
    run(part1, missing);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 37);
    }
}
