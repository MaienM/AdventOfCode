type Map = Vec<Vec<bool>>;

fn parse_input(input: &str) -> Vec<Map> {
    input
        .split("\n\n")
        .map(|block| {
            block
                .split('\n')
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect()
}

fn rotate(map: &Map) -> Map {
    (0..map[0].len())
        .map(|i| map.iter().map(|row| row[i]).collect())
        .collect()
}

fn find_reflection_row(map: &Map) -> Option<usize> {
    let len = map.len();
    'row: for i in 0..(len - 1) {
        for o in 0..=i.min(len - i - 2) {
            if map[i - o] != map[i + o + 1] {
                continue 'row;
            }
        }
        return Some(i);
    }
    None
}

pub fn part1(input: &str) -> usize {
    let maps = parse_input(input);
    maps.into_iter()
        .map(|map| {
            if let Some(row) = find_reflection_row(&map) {
                (row + 1) * 100
            } else {
                let map = rotate(&map);
                let column = find_reflection_row(&map).unwrap();
                column + 1
            }
        })
        .sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 405, test)]
    static EXAMPLE_INPUT: &str = "
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            vec![
                vec![true, false, true, true, false, false, true, true, false],
                vec![false, false, true, false, true, true, false, true, false],
                vec![true, true, false, false, false, false, false, false, true],
                vec![true, true, false, false, false, false, false, false, true],
                vec![false, false, true, false, true, true, false, true, false],
                vec![false, false, true, true, false, false, true, true, false],
                vec![true, false, true, false, true, true, false, true, false],
            ],
            vec![
                vec![true, false, false, false, true, true, false, false, true],
                vec![true, false, false, false, false, true, false, false, true],
                vec![false, false, true, true, false, false, true, true, true],
                vec![true, true, true, true, true, false, true, true, false],
                vec![true, true, true, true, true, false, true, true, false],
                vec![false, false, true, true, false, false, true, true, true],
                vec![true, false, false, false, false, true, false, false, true],
            ],
        ];
        assert_eq!(actual, expected);
    }
}
