use aoc::runner::*;

#[derive(Debug, Eq, PartialEq)]
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

fn check(left: Item, right: Item) -> Option<bool> {
    match (left, right) {
        (Item::List(left), Item::List(right)) => {
            let llen = left.len();
            let rlen = right.len();
            for (l, r) in left.into_iter().zip(right.into_iter()) {
                let result = check(l, r);
                if result.is_some() {
                    return result;
                }
            }
            return check(Item::Number(llen as u8), Item::Number(rlen as u8));
        }
        (Item::Number(left), Item::List(right)) => {
            return check(Item::List(vec![Item::Number(left)]), Item::List(right));
        }
        (Item::List(left), Item::Number(right)) => {
            return check(Item::List(left), Item::List(vec![Item::Number(right)]));
        }
        (Item::Number(left), Item::Number(right)) => {
            if left == right {
                return Option::None;
            }
            return Option::Some(left < right);
        }
    };
}

pub fn part1(input: String) -> usize {
    let pairs = parse_input(input);
    let mut result = 0;
    for (i, (left, right)) in pairs.into_iter().enumerate() {
        if check(left, right).unwrap_or(false) {
            result += i + 1;
        }
    }
    return result;
}

fn main() {
    run(part1, missing::<i64>);
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
}
