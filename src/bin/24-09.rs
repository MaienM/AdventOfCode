use std::cmp::Ordering;

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

#[allow(dead_code)]
fn print_filesystem(filesystem: &[Item]) {
    for item in filesystem {
        let (chr, size) = match item {
            Item::File(idx, size) => (char::from_digit(*idx as u32, 10).unwrap(), size),
            Item::Empty(size) => ('.', size),
        };
        for _ in 0..*size {
            print!("{chr}");
        }
    }
    println!();
}

fn checksum(filesystem: &[Item]) -> usize {
    let mut checksum = 0;
    let mut i = 0;
    for item in filesystem {
        match item {
            Item::File(id, n) => {
                for _ in 0..*n {
                    checksum += id * i;
                    i += 1;
                }
            }
            Item::Empty(n) => {
                i += *n as usize;
            }
        }
    }
    checksum
}

fn defragment(filesystem: &mut Vec<Item>, move_whole_files: bool) {
    let mut idx = filesystem.len();
    while idx > 0 {
        idx -= 1;
        let file = filesystem.get(idx);
        let Some(Item::File(file_id, file_size)) = file else {
            continue;
        };
        let file_id = *file_id;
        let file_size = *file_size;

        let empty_min_size = if move_whole_files { file_size } else { 1 };
        let empty = filesystem
            .iter()
            .enumerate()
            .take(idx)
            .find(|(_, i)| match i {
                Item::Empty(s) => *s >= empty_min_size,
                Item::File(..) => false,
            });
        let Some((empty_idx, Item::Empty(empty_size))) = empty else {
            continue;
        };
        let empty_size = *empty_size;

        match file_size.cmp(&empty_size) {
            Ordering::Less => {
                filesystem[empty_idx] = Item::Empty(file_size);
                filesystem.insert(empty_idx + 1, Item::Empty(empty_size - file_size));
                filesystem.swap(idx + 1, empty_idx);
            }
            Ordering::Equal => {
                filesystem.swap(empty_idx, idx);
            }
            Ordering::Greater => {
                filesystem[idx] = Item::File(file_id, empty_size);
                filesystem[empty_idx] = Item::File(file_id, file_size - empty_size);
                filesystem.swap(empty_idx, idx);
                idx += 1;
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut filesystem = parse_input(input);
    defragment(&mut filesystem, false);
    checksum(&filesystem)
}

pub fn part2(input: &str) -> usize {
    let mut filesystem = parse_input(input);
    defragment(&mut filesystem, true);
    checksum(&filesystem)
}

aoc_runner::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_runner::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 1928, part2 = 2858)]
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

    #[test]
    fn example_checksum() {
        assert_eq!(
            1928,
            checksum(&vec![
                Item::File(0, 2),
                Item::File(9, 2),
                Item::File(8, 1),
                Item::File(1, 3),
                Item::File(8, 3),
                Item::File(2, 1),
                Item::File(7, 3),
                Item::File(3, 3),
                Item::File(6, 1),
                Item::File(4, 2),
                Item::File(6, 1),
                Item::File(5, 4),
                Item::File(6, 2),
            ]),
        );
        assert_eq!(
            2858,
            checksum(&vec![
                Item::File(0, 2),
                Item::File(9, 2),
                Item::File(2, 1),
                Item::File(1, 3),
                Item::File(7, 3),
                Item::Empty(1),
                Item::File(4, 2),
                Item::Empty(1),
                Item::File(3, 3),
                Item::Empty(4),
                Item::File(5, 4),
                Item::Empty(1),
                Item::File(6, 4),
                Item::Empty(5),
                Item::File(8, 4),
            ]),
        );
    }
}
