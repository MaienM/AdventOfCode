use aoc::utils::parse;

type HMap = [u8; 5];

fn parse_input(input: &str) -> (Vec<HMap>, Vec<HMap>) {
    parse!(input => [blocks split on "\n\n" with [split on '\n']]);

    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for block in blocks {
        let mut map = [0; 5];
        for line in &block {
            for (idx, chr) in line.char_indices() {
                map[idx] += u8::from(chr == '#');
            }
        }
        if block[0] == "#####" {
            locks.push(map);
        } else {
            keys.push(map);
        }
    }
    (locks, keys)
}

pub fn part1(input: &str) -> usize {
    let (locks, keys) = parse_input(input);
    let mut result = 0;
    for lock in &locks {
        'key: for key in &keys {
            for i in 0..5 {
                if lock[i] + key[i] >= 8 {
                    continue 'key;
                }
            }
            result += 1;
        }
    }
    result
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 3)]
    static EXAMPLE_INPUT: &str = "
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = (
            vec![
                [1,6,4,5,4],
                [2,3,1,6,4],
            ],
            vec![
                [6,1,3,2,4],
                [5,4,5,1,3],
                [4,1,3,1,2],
            ],
        );
        assert_eq!(actual, expected);
    }
}
