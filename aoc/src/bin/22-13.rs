puzzle_lib::setup!(title = "Distress Signal");

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
            _ => panic!("Unexpected character {chr}"),
        }
    }
    list.pop().unwrap()
}

fn parse_input(input: &str) -> Vec<(Item, Item)> {
    parse!(input => {
        [pairs split on "\n\n" with
            { left '\n' right }
            => (parse_line(left), parse_line(right))
        ]
    } => pairs)
}

fn compare(left: &Item, right: &Item) -> Ordering {
    match (left, right) {
        (Item::List(left), Item::List(right)) => {
            let len_left = left.len();
            let len_right = right.len();
            for (l, r) in left.iter().zip(right.iter()) {
                let result = compare(l, r);
                if result.is_ne() {
                    return result;
                }
            }
            len_left.cmp(&len_right)
        }
        (Item::Number(_), Item::List(_)) => compare(&Item::List(vec![left.clone()]), right),
        (Item::List(_), Item::Number(_)) => compare(left, &Item::List(vec![right.clone()])),
        (Item::Number(left), Item::Number(right)) => left.cmp(right),
    }
}

pub fn part1(input: &str) -> usize {
    let pairs = parse_input(input);
    let mut result = 0;
    for (i, (left, right)) in pairs.into_iter().enumerate() {
        if compare(&left, &right).is_lt() {
            result += i + 1;
        }
    }
    result
}

pub fn part2(input: &str) -> usize {
    let mut packets: Vec<Item> = parse_input(input)
        .into_iter()
        .flat_map(|p| [p.0, p.1])
        .collect();

    let divider1 = parse_line("[[2]]");
    let divider2 = parse_line("[[6]]");
    packets.push(divider1.clone());
    packets.push(divider2.clone());

    packets.sort_by(compare);

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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 13, part2 = 140)]
    static EXAMPLE_INPUT: &str = "
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
        let actual = parse_input(&EXAMPLE_INPUT);
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
}
