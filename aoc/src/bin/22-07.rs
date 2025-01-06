aoc::setup!(title = "No Space Left On Device");

use std::{collections::HashMap, iter::Peekable, vec::IntoIter};

type Listing<'a> = HashMap<&'a str, Entry<'a>>;

#[derive(Debug, Eq, PartialEq)]
enum Entry<'a> {
    File(usize),
    Dir(Listing<'a>),
}
impl Entry<'_> {
    fn size(&self) -> usize {
        match self {
            Entry::File(fsize) => *fsize,
            Entry::Dir(items) => items.values().map(Entry::size).sum(),
        }
    }
}

fn parse_input_lines<'a>(
    dir: &mut Listing<'a>,
    lines: &mut Peekable<impl Iterator<Item = &'a str>>,
) -> &'a str {
    loop {
        let line = lines.next();
        if line.is_none() {
            return "/";
        }
        let mut parts = line.unwrap().split(' ');
        match parts.nth(1).unwrap() {
            "cd" => match parts.next().unwrap() {
                "/" => return "/",
                ".." => return "..",
                name => {
                    let Some(Entry::Dir(subdir)) = dir.get_mut(name) else {
                        panic!()
                    };
                    match parse_input_lines(subdir, lines) {
                        "/" => return "/",
                        ".." => continue,
                        _ => panic!(),
                    }
                }
            },
            "ls" => {
                while lines.peek().filter(|l| !l.starts_with('$')).is_some() {
                    let [left, right]: [&'a str; 2] = lines
                        .next()
                        .unwrap()
                        .splitn(2, ' ')
                        .collect::<Vec<&'a str>>()
                        .try_into()
                        .unwrap();
                    match (left, right) {
                        ("dir", name) => dir.insert(name, Entry::Dir(Listing::new())),
                        (size, name) => dir.insert(name, Entry::File(size.parse().unwrap())),
                    };
                }
            }
            _ => panic!(),
        }
    }
}

fn parse_input(input: &str) -> Entry {
    let mut lines: Peekable<IntoIter<&'_ str>> = input
        .lines()
        .collect::<Vec<&'_ str>>()
        .into_iter()
        .peekable();
    let mut root = Listing::new();
    while lines.peek().is_some() {
        parse_input_lines(&mut root, &mut lines);
    }
    Entry::Dir(root)
}

fn get_dir_sizes(matches: &mut Vec<usize>, entry: &Entry) {
    if let Entry::Dir(dir) = entry {
        matches.push(entry.size());
        for e in dir.values() {
            get_dir_sizes(matches, e);
        }
    };
}

pub fn part1(input: &str) -> usize {
    let root = parse_input(input);
    let mut sizes = vec![];
    get_dir_sizes(&mut sizes, &root);
    sizes.into_iter().filter(|s| s <= &100_000).sum()
}

pub fn part2(input: &str) -> usize {
    let root = parse_input(input);
    let space_needed = 30_000_000 - (70_000_000 - root.size());
    let mut sizes = vec![];
    get_dir_sizes(&mut sizes, &root);
    sizes
        .into_iter()
        .filter(|s| s >= &space_needed)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use aoc_runner::example_input;
    use common_macros::hash_map;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 95_437, part2 = 24_933_642)]
    static EXAMPLE_INPUT: &str = "
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = Entry::Dir(hash_map! {
            "a" => Entry::Dir(hash_map!{
                "e" => Entry::Dir(hash_map!{
                    "i" => Entry::File(584),
                }),
                "f" => Entry::File(29_116),
                "g" => Entry::File(2_557),
                "h.lst" => Entry::File(62_596),
            }),
            "b.txt" => Entry::File(14_848_514),
            "c.dat" => Entry::File(8_504_156),
            "d" => Entry::Dir(hash_map!{
                "j" => Entry::File(4_060_174),
                "d.log" => Entry::File(8_033_020),
                "d.ext" => Entry::File(5_626_152),
                "k" => Entry::File(7_214_296),
            }),
        });
        assert_eq!(actual, expected);
    }
}
