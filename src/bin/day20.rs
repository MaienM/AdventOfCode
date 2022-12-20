use aoc::runner::*;

fn parse_input(input: String) -> Vec<i16> {
    return input
        .trim()
        .split("\n")
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
}

fn mix(numbers: Vec<i16>) -> Vec<i16> {
    let mut mixing: Vec<(usize, i16)> = numbers.into_iter().enumerate().collect();
    let len = mixing.len();
    for i in 0..len {
        let (idx, offset) = mixing
            .iter()
            .enumerate()
            .find(|(_, (idx, _))| idx == &i)
            .map(|(idx, (_, n))| (idx as i16, n))
            .unwrap();

        let mut new_idx = (idx + offset) % (len as i16 - 1);
        while new_idx < 0 {
            new_idx += len as i16 - 1;
        }

        if idx != new_idx {
            let item = mixing.remove(idx as usize);
            mixing.insert(new_idx as usize, item);
        }
    }
    return mixing.into_iter().map(|(_i, n)| n).collect();
}

pub fn part1(input: String) -> i16 {
    let numbers = parse_input(input);
    let numbers = mix(numbers);
    let offset = numbers
        .iter()
        .enumerate()
        .find(|(_, n)| n == &&0)
        .unwrap()
        .0;
    let len = numbers.len();
    return numbers[(1000 + offset) % len]
        + numbers[(2000 + offset) % len]
        + numbers[(3000 + offset) % len];
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        1
        2
        -3
        3
        -2
        0
        4
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = vec![1, 2, -3, 3, -2, 0, 4];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_mix() {
        let actual = mix(vec![1, 2, -3, 3, -2, 0, 4]);
        let expected = vec![-2, 1, 2, -3, 4, 0, 3];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 3);
    }
}
