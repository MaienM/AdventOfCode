use std::collections::HashMap;
use std::iter::Peekable;
use std::vec::IntoIter;

use aoc::runner::*;

type Listing<'a> = HashMap<&'a str, Entry<'a>>;

#[derive(Debug, Eq, PartialEq)]
enum Entry<'a> {
    File(usize),
    Dir(Listing<'a>),
}
impl<'a> Entry<'a> {
    fn size(&self) -> usize {
        match self {
            Entry::File(fsize) => *fsize,
            Entry::Dir(items) => items.values().map(Entry::size).sum(),
        }
    }
}

fn parse_input_lines<'a>(
    dir: &mut Listing<'a>,
    mut lines: &mut Peekable<impl Iterator<Item = &'a str>>,
) -> &'a str {
    loop {
        let line = lines.next();
        if line.is_none() {
            return "/";
        }
        let mut parts = line.unwrap().split(" ");
        match parts.nth(1).unwrap() {
            "cd" => match parts.next().unwrap() {
                "/" => return "/",
                ".." => return "..",
                name => {
                    // if !dir.contains_key(name) {
                    //     dir.insert(name, Entry::Dir(HashMap::new()));
                    // }
                    let mut subdir = match dir.get_mut(name) {
                        Some(Entry::Dir(listing)) => listing,
                        _ => panic!(),
                    };
                    match parse_input_lines(&mut subdir, &mut lines) {
                        "/" => return "/",
                        ".." => continue,
                        _ => panic!(),
                    }
                }
            },
            "ls" => {
                while lines.peek().filter(|l| !l.starts_with("$")).is_some() {
                    let [left, right]: [&'a str; 2] = lines
                        .next()
                        .unwrap()
                        .splitn(2, " ")
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

fn parse_input<'a>(input: &'a String) -> Entry {
    let mut lines: Peekable<IntoIter<&'a str>> = input
        .trim()
        .split("\n")
        .map(str::trim)
        .collect::<Vec<&'a str>>()
        .into_iter()
        .peekable();
    let mut root = Listing::new();
    while lines.peek().is_some() {
        parse_input_lines(&mut root, &mut lines);
    }
    return Entry::Dir(root);
}

fn get_dir_sizes(matches: &mut Vec<usize>, entry: &Entry) {
    match entry {
        Entry::Dir(dir) => {
            matches.push(entry.size());
            for e in dir.values() {
                get_dir_sizes(matches, e);
            }
        }
        _ => {}
    };
}

pub fn part1(input: String) -> usize {
    let root = parse_input(&input);
    let mut sizes = vec![];
    get_dir_sizes(&mut sizes, &root);
    return sizes.into_iter().filter(|s| s <= &100_000).sum();
}

pub fn part2(input: String) -> usize {
    let root = parse_input(&input);
    let space_needed = 30_000_000 - (70_000_000 - root.size());
    let mut sizes = vec![];
    get_dir_sizes(&mut sizes, &root);
    return sizes
        .into_iter()
        .filter(|s| s >= &space_needed)
        .min()
        .unwrap();
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use common_macros::hash_map;
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
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
        let input = EXAMPLE_INPUT.to_string();
        let actual = parse_input(&input);
        let expected = Entry::Dir(hash_map! {
            "a" => Entry::Dir(hash_map!{
                "e" => Entry::Dir(hash_map!{
                    "i" => Entry::File(584),
                }),
                "f" => Entry::File(29116),
                "g" => Entry::File(2557),
                "h.lst" => Entry::File(62596),
            }),
            "b.txt" => Entry::File(14848514),
            "c.dat" => Entry::File(8504156),
            "d" => Entry::Dir(hash_map!{
                "j" => Entry::File(4060174),
                "d.log" => Entry::File(8033020),
                "d.ext" => Entry::File(5626152),
                "k" => Entry::File(7214296),
            }),
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 95_437);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 24_933_642);
    }
}
