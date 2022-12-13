use aoc::runner::*;
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Item {
    List(Vec<Item>),
    Number(u8),
}

fn parse_line(line: &str) -> Item {
    let mut stack = Vec::new();
    let mut list = Vec::new();
    let mut number = Option::None;
    for chr in line.chars() {
        match chr {
            '[' => {
                stack.push(list);
                list = Vec::new();
            }
            ']' => {
                if number.is_some() {
                    list.push(Item::Number(number.unwrap()));
                    number = Option::None;
                }
                let mut parent = stack.pop().unwrap();
                parent.push(Item::List(list));
                list = parent;
            }
            ',' => {
                if number.is_some() {
                    list.push(Item::Number(number.unwrap()));
                    number = Option::None;
                }
            }
            '0'..='9' => {
                number = Option::Some(number.unwrap_or(0) * 10 + chr.to_digit(10).unwrap() as u8);
            }
            _ => panic!("Unexpected character {}", chr),
        }
    }
    return list.pop().unwrap();
}

fn parse_input(input: String) -> Vec<(Item, Item)> {
    return input
        .trim()
        .split("\n\n")
        .map(str::trim)
        .map(|block| {
            let mut lines = block.split("\n");
            let left = parse_line(lines.next().unwrap().trim());
            let right = parse_line(lines.next().unwrap().trim());
            assert!(lines.next().is_none());
            return (left, right);
        })
        .collect();
}

fn compare(left: &Item, right: &Item) -> Ordering {
    match (left, right) {
        (Item::List(left), Item::List(right)) => {
            let llen = left.len();
            let rlen = right.len();
            for (l, r) in left.into_iter().zip(right.into_iter()) {
                let result = compare(l, r);
                if result.is_ne() {
                    return result;
                }
            }
            return compare(&Item::Number(llen as u8), &Item::Number(rlen as u8));
        }
        (Item::Number(_), Item::List(_)) => {
            return compare(&Item::List(vec![left.clone()]), right);
        }
        (Item::List(_), Item::Number(_)) => {
            return compare(left, &Item::List(vec![right.clone()]));
        }
        (Item::Number(left), Item::Number(right)) => {
            if left < right {
                return Ordering::Less;
            } else if left == right {
                return Ordering::Equal;
            } else {
                return Ordering::Greater;
            }
        }
    };
}

pub fn part1(input: String) -> usize {
    let pairs = parse_input(input);
    let mut result = 0;
    for (i, (left, right)) in pairs.into_iter().enumerate() {
        if compare(&left, &right).is_lt() {
            result += i + 1;
        }
    }
    return result;
}

pub fn part2(input: String) -> usize {
    let mut packets: Vec<Item> = parse_input(input)
        .into_iter()
        .map(|p| [p.0, p.1])
        .flatten()
        .collect();

    let divider1 = parse_line("[[2]]");
    let divider2 = parse_line("[[6]]");
    packets.push(divider1.clone());
    packets.push(divider2.clone());

    packets.sort_by(|l, r| compare(l, r));

    let mut idx1 = 0;
    for (i, packet) in packets.into_iter().enumerate() {
        if packet == divider1 {
            idx1 = i + 1;
        } else if packet == divider2 {
            return idx1 * (i + 1);
        }
    }
    panic!();
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = vec![
            (
                Item::List(vec![
                    Item::Number(1),
                    Item::Number(1),
                    Item::Number(3),
                    Item::Number(1),
                    Item::Number(1),
                ]),
                Item::List(vec![
                    Item::Number(1),
                    Item::Number(1),
                    Item::Number(5),
                    Item::Number(1),
                    Item::Number(1),
                ]),
            ),
            (
                Item::List(vec![
                    Item::List(vec![Item::Number(1)]),
                    Item::List(vec![Item::Number(2), Item::Number(3), Item::Number(4)]),
                ]),
                Item::List(vec![Item::List(vec![Item::Number(1)]), Item::Number(4)]),
            ),
            (
                Item::List(vec![Item::Number(9)]),
                Item::List(vec![Item::List(vec![
                    Item::Number(8),
                    Item::Number(7),
                    Item::Number(6),
                ])]),
            ),
            (
                Item::List(vec![
                    Item::List(vec![Item::Number(4), Item::Number(4)]),
                    Item::Number(4),
                    Item::Number(4),
                ]),
                Item::List(vec![
                    Item::List(vec![Item::Number(4), Item::Number(4)]),
                    Item::Number(4),
                    Item::Number(4),
                    Item::Number(4),
                ]),
            ),
            (
                Item::List(vec![
                    Item::Number(7),
                    Item::Number(7),
                    Item::Number(7),
                    Item::Number(7),
                ]),
                Item::List(vec![Item::Number(7), Item::Number(7), Item::Number(7)]),
            ),
            (Item::List(vec![]), Item::List(vec![Item::Number(3)])),
            (
                Item::List(vec![Item::List(vec![Item::List(vec![])])]),
                Item::List(vec![Item::List(vec![])]),
            ),
            (
                Item::List(vec![
                    Item::Number(1),
                    Item::List(vec![
                        Item::Number(2),
                        Item::List(vec![
                            Item::Number(3),
                            Item::List(vec![
                                Item::Number(4),
                                Item::List(vec![Item::Number(5), Item::Number(6), Item::Number(7)]),
                            ]),
                        ]),
                    ]),
                    Item::Number(8),
                    Item::Number(9),
                ]),
                Item::List(vec![
                    Item::Number(1),
                    Item::List(vec![
                        Item::Number(2),
                        Item::List(vec![
                            Item::Number(3),
                            Item::List(vec![
                                Item::Number(4),
                                Item::List(vec![Item::Number(5), Item::Number(6), Item::Number(0)]),
                            ]),
                        ]),
                    ]),
                    Item::Number(8),
                    Item::Number(9),
                ]),
            ),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 13);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 140);
    }
}
