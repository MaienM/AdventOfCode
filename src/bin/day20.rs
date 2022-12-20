use aoc::runner::*;

fn parse_input(input: String) -> Vec<i64> {
    return input
        .trim()
        .split("\n")
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
}

fn mix(numbers: Vec<i64>, times: usize) -> Vec<i64> {
    let mut mixing: Vec<(usize, i64)> = numbers.into_iter().enumerate().collect();
    let len = mixing.len();
    for _ in 0..times {
        for i in 0..len {
            let (idx, offset) = mixing
                .iter()
                .enumerate()
                .find(|(_, (idx, _))| idx == &i)
                .map(|(idx, (_, n))| (idx as i64, n))
                .unwrap();

            let mut new_idx = (idx + offset) % (len as i64 - 1);
            while new_idx < 0 {
                new_idx += len as i64 - 1;
            }

            if idx != new_idx {
                let item = mixing.remove(idx as usize);
                mixing.insert(new_idx as usize, item);
            }
        }
    }
    return mixing.into_iter().map(|(_i, n)| n).collect();
}

fn get_coordinates(numbers: Vec<i64>) -> i64 {
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

pub fn part1(input: String) -> i64 {
    let numbers = parse_input(input);
    let numbers = mix(numbers, 1);
    return get_coordinates(numbers);
}

pub fn part2(input: String) -> i64 {
    let numbers = parse_input(input);
    let numbers = numbers.into_iter().map(|n| n * 811_589_153).collect();
    let numbers = mix(numbers, 10);
    return get_coordinates(numbers);
}

fn main() {
    run(part1, part2);
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
        let actual = mix(vec![1, 2, -3, 3, -2, 0, 4], 1);
        let expected = vec![-2, 1, 2, -3, 4, 0, 3];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_mix_multi() {
        let start = vec![
            811589153,
            1623178306,
            -2434767459,
            2434767459,
            -1623178306,
            0,
            3246356612,
        ];
        assert_eq!(
            mix(start.clone(), 1),
            vec![
                0,
                -2434767459,
                3246356612,
                -1623178306,
                2434767459,
                1623178306,
                811589153,
            ]
        );
        assert_eq!(
            mix(start.clone(), 2),
            vec![
                0,
                2434767459,
                1623178306,
                3246356612,
                -2434767459,
                -1623178306,
                811589153,
            ]
        );
        assert_eq!(
            mix(start.clone(), 3),
            vec![
                2434767459,
                3246356612,
                1623178306,
                -1623178306,
                -2434767459,
                0,
                811589153,
            ]
        );
        assert_eq!(
            mix(start.clone(), 4),
            vec![
                0,
                1623178306,
                -2434767459,
                811589153,
                2434767459,
                3246356612,
                -1623178306,
            ]
        );
        assert_eq!(
            mix(start.clone(), 5),
            vec![
                0,
                811589153,
                -1623178306,
                1623178306,
                -2434767459,
                3246356612,
                2434767459,
            ]
        );
        assert_eq!(
            mix(start.clone(), 6),
            vec![
                811589153,
                -1623178306,
                3246356612,
                -2434767459,
                1623178306,
                2434767459,
                0,
            ]
        );
        assert_eq!(
            mix(start.clone(), 7),
            vec![
                3246356612,
                0,
                -2434767459,
                2434767459,
                1623178306,
                -1623178306,
                811589153,
            ]
        );
        assert_eq!(
            mix(start.clone(), 8),
            vec![
                -2434767459,
                2434767459,
                -1623178306,
                0,
                1623178306,
                3246356612,
                811589153,
            ]
        );
        assert_eq!(
            mix(start.clone(), 9),
            vec![
                1623178306,
                -2434767459,
                3246356612,
                2434767459,
                -1623178306,
                0,
                811589153,
            ]
        );
        assert_eq!(
            mix(start.clone(), 10),
            vec![
                0,
                -2434767459,
                1623178306,
                3246356612,
                -1623178306,
                2434767459,
                811589153,
            ]
        );
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 3);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 1_623_178_306);
    }
}
