use aoc::utils::parse;

#[derive(Debug, Eq, PartialEq)]
enum Item {
    File(usize, u8),
    Empty(u8),
}

fn parse_input(input: &str) -> Vec<Item> {
    parse!(input => [nums chars as u8]);
    nums.into_iter()
        .enumerate()
        .map(|(i, n)| {
            if i % 2 == 0 {
                Item::File(i / 2, n)
            } else {
                Item::Empty(n)
            }
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    1
}

pub fn part2(input: &str) -> usize {
    let mut filesystem = parse_input(input);
    let mut idx = filesystem.len() - 1;
    while idx > 0 {
        let size = match filesystem.get(idx) {
            Some(Item::File(_, size)) => *size,
            _ => {
                idx -= 1;
                continue;
            }
        };
        let empty = filesystem
            .iter()
            .enumerate()
            .take(idx)
            .find(|(_, i)| match i {
                Item::Empty(s) => *s >= size,
                Item::File(..) => false,
            });
        if let Some((empty_idx, Item::Empty(empty_size))) = empty {
            let empty_size = *empty_size;
            if size == empty_size {
                filesystem.swap(empty_idx, idx);
            } else {
                let file = filesystem.remove(idx);
                filesystem.insert(idx, Item::Empty(size));
                filesystem[empty_idx] = file;
                filesystem.insert(empty_idx + 1, Item::Empty(empty_size - size));
            }
        }
        idx -= 1;
    }

    let mut checksum = 0;
    let mut i = 0;
    for item in filesystem {
        match item {
            Item::File(id, n) => {
                for _ in 0..n {
                    checksum += id * i;
                    i += 1;
                }
            }
            Item::Empty(n) => {
                i += n as usize;
            }
        }
    }
    checksum
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 1, part2 = 2858)]
    static EXAMPLE_INPUT: &str = "2333133121414131402";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            Item::File(0, 2),
            Item::Empty(3),
            Item::File(1, 3),
            Item::Empty(3),
            Item::File(2, 1),
            Item::Empty(3),
            Item::File(3, 3),
            Item::Empty(1),
            Item::File(4, 2),
            Item::Empty(1),
            Item::File(5, 4),
            Item::Empty(1),
            Item::File(6, 4),
            Item::Empty(1),
            Item::File(7, 3),
            Item::Empty(1),
            Item::File(8, 4),
            Item::Empty(0),
            Item::File(9, 2),
        ];
        assert_eq!(actual, expected);
    }
}
