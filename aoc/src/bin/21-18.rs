puzzle_lib::setup!(title = "Snailfish");

use std::collections::LinkedList;

type Path = Vec<u8>;
type Entry = (Path, u8);
type Entries = LinkedList<Entry>;

fn parse_line(line: &str) -> Entries {
    let mut path = Path::new();
    let mut entries = Entries::new();
    for chr in line.chars() {
        match chr {
            '[' => {
                path.push(0);
            }
            ']' => {
                if path.pop().is_none() {
                    break;
                }
            }
            ',' => {
                path.pop();
                path.push(1);
            }
            chr => {
                entries.push_back((path.clone(), chr.to_digit(10).unwrap() as u8));
            }
        }
    }
    entries
}

fn parse_input(input: &str) -> Vec<Entries> {
    parse!(input => { [lines split on '\n' with parse_line] } => lines)
}

fn do_explode(entries: &mut Entries) -> bool {
    let index = entries.iter().position(|(p, _)| p.len() > 4);
    if index.is_none() {
        return false;
    }
    let index = index.unwrap();

    let mut right = entries.split_off(index);
    let left = entries;
    let pair_left = right.pop_front().unwrap();
    let pair_right = right.pop_front().unwrap();

    if let Some(mut entry) = left.pop_back() {
        entry.1 += pair_left.1;
        left.push_back(entry);
    }
    if let Some(mut entry) = right.pop_front() {
        entry.1 += pair_right.1;
        right.push_front(entry);
    }

    let mut path = pair_right.0;
    path.pop();
    left.push_back((path, 0));

    left.append(&mut right);

    true
}

fn do_split(entries: &mut Entries) -> bool {
    let index = entries.iter().position(|(_, v)| v > &9);
    if index.is_none() {
        return false;
    }
    let index = index.unwrap();

    let mut right = entries.split_off(index);
    let left = entries;
    let (mut path, value) = right.pop_front().unwrap();

    path.push(0);
    left.push_back((path.clone(), value / 2));
    path.pop();
    path.push(1);
    left.push_back((path.clone(), value.div_ceil(2)));

    left.append(&mut right);

    true
}

fn do_reduce(entries: &mut Entries) {
    while do_explode(entries) || do_split(entries) {}
}

fn sum(left: &Entries, right: &Entries) -> Entries {
    let mut entries = Entries::new();
    for (i, input) in [(0, left), (1, right)] {
        entries.append(
            &mut input
                .iter()
                .map(|(p, v)| {
                    let mut pnew = Path::new();
                    pnew.push(i);
                    for pelem in p {
                        pnew.push(*pelem);
                    }
                    (pnew, *v)
                })
                .collect(),
        );
    }
    do_reduce(&mut entries);
    entries
}

fn get_magnitude(entries: &Entries) -> u32 {
    entries
        .iter()
        .map(|(p, v)| {
            // For each level this is nested on the left side (s=0) multiply by 3, for the right (s=1) by 2. This works out to 3-s.
            let pathmul = p.iter().map(|s| u32::from(3 - s)).product::<u32>();
            u32::from(*v) * pathmul
        })
        .sum()
}

pub fn part1(input: &str) -> u32 {
    let mut lines = parse_input(input);
    let mut result = lines.remove(0);
    for line in lines {
        result = sum(&result, &line);
    }
    get_magnitude(&result)
}

pub fn part2(input: &str) -> u32 {
    let lines = parse_input(input);
    let mut highest = 0;
    for line1 in &lines {
        for line2 in &lines {
            if line1 == line2 {
                continue;
            }

            let result = get_magnitude(&sum(line1, line2));
            if result > highest {
                highest = result;
            }
        }
    }
    highest
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 4140, part2 = 3993)]
    static EXAMPLE_INPUT: &str = "
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    ";

    #[test]
    fn example_parse_input() {
        assert_eq!(
            parse_input("[1,2]"),
            vec![LinkedList::from([(vec![0], 1), (vec![1], 2)])]
        );
        assert_eq!(
            parse_input("[[1,2],3]"),
            vec![LinkedList::from([
                (vec![0, 0], 1),
                (vec![0, 1], 2),
                (vec![1], 3)
            ])]
        );
        assert_eq!(
            parse_input("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"),
            vec![LinkedList::from([
                (vec![0, 0, 0, 0], 1),
                (vec![0, 0, 0, 1], 3),
                (vec![0, 0, 1, 0], 5),
                (vec![0, 0, 1, 1], 3),
                (vec![0, 1, 0, 0], 1),
                (vec![0, 1, 0, 1], 3),
                (vec![0, 1, 1, 0], 8),
                (vec![0, 1, 1, 1], 7),
                (vec![1, 0, 0, 0], 4),
                (vec![1, 0, 0, 1], 9),
                (vec![1, 0, 1, 0], 6),
                (vec![1, 0, 1, 1], 9),
                (vec![1, 1, 0, 0], 8),
                (vec![1, 1, 0, 1], 2),
                (vec![1, 1, 1, 0], 7),
                (vec![1, 1, 1, 1], 3),
            ])]
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn example_do_explode() {
        let mut value = LinkedList::from([
            (vec![0, 0, 0, 0, 0], 9),
            (vec![0, 0, 0, 0, 1], 8),
            (vec![0, 0, 0, 1], 1),
            (vec![0, 0, 1], 2),
            (vec![0, 1], 3),
            (vec![1], 4),
        ]);
        assert!(do_explode(&mut value));
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0, 0, 0], 0),
                (vec![0, 0, 0, 1], 9),
                (vec![0, 0, 1], 2),
                (vec![0, 1], 3),
                (vec![1], 4)
            ])
        );

        let mut value = LinkedList::from([
            (vec![0], 7),
            (vec![1, 0], 6),
            (vec![1, 1, 0], 5),
            (vec![1, 1, 1, 0], 4),
            (vec![1, 1, 1, 1, 0], 3),
            (vec![1, 1, 1, 1, 1], 2),
        ]);
        assert!(do_explode(&mut value));
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0], 7),
                (vec![1, 0], 6),
                (vec![1, 1, 0], 5),
                (vec![1, 1, 1, 0], 7),
                (vec![1, 1, 1, 1], 0),
            ]),
        );

        let mut value = LinkedList::from([
            (vec![0, 0], 6),
            (vec![0, 1, 0], 5),
            (vec![0, 1, 1, 0], 4),
            (vec![0, 1, 1, 1, 0], 3),
            (vec![0, 1, 1, 1, 1], 2),
            (vec![1], 1),
        ]);
        assert!(do_explode(&mut value));
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0], 6),
                (vec![0, 1, 0], 5),
                (vec![0, 1, 1, 0], 7),
                (vec![0, 1, 1, 1], 0),
                (vec![1], 3),
            ]),
        );

        let mut value = LinkedList::from([
            (vec![0, 0], 3),
            (vec![0, 1, 0], 2),
            (vec![0, 1, 1, 0], 1),
            (vec![0, 1, 1, 1, 0], 7),
            (vec![0, 1, 1, 1, 1], 3),
            (vec![1, 0], 6),
            (vec![1, 1, 0], 5),
            (vec![1, 1, 1, 0], 4),
            (vec![1, 1, 1, 1, 0], 3),
            (vec![1, 1, 1, 1, 1], 2),
        ]);
        assert!(do_explode(&mut value));
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0], 3),
                (vec![0, 1, 0], 2),
                (vec![0, 1, 1, 0], 8),
                (vec![0, 1, 1, 1], 0),
                (vec![1, 0], 9),
                (vec![1, 1, 0], 5),
                (vec![1, 1, 1, 0], 4),
                (vec![1, 1, 1, 1, 0], 3),
                (vec![1, 1, 1, 1, 1], 2),
            ]),
        );

        let mut value = LinkedList::from([
            (vec![0, 0], 3),
            (vec![0, 1, 0], 2),
            (vec![0, 1, 1, 0], 8),
            (vec![0, 1, 1, 1], 0),
            (vec![1, 0], 9),
            (vec![1, 1, 0], 5),
            (vec![1, 1, 1, 0], 4),
            (vec![1, 1, 1, 1, 0], 3),
            (vec![1, 1, 1, 1, 1], 2),
        ]);
        assert!(do_explode(&mut value));
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0], 3),
                (vec![0, 1, 0], 2),
                (vec![0, 1, 1, 0], 8),
                (vec![0, 1, 1, 1], 0),
                (vec![1, 0], 9),
                (vec![1, 1, 0], 5),
                (vec![1, 1, 1, 0], 7),
                (vec![1, 1, 1, 1], 0),
            ]),
        );
    }

    #[test]
    fn example_do_actions() {
        let mut value = LinkedList::from([
            (vec![0, 0, 0, 0, 0], 4),
            (vec![0, 0, 0, 0, 1], 3),
            (vec![0, 0, 0, 1], 4),
            (vec![0, 0, 1], 4),
            (vec![0, 1, 0], 7),
            (vec![0, 1, 1, 0, 0], 8),
            (vec![0, 1, 1, 0, 1], 4),
            (vec![0, 1, 1, 1], 9),
            (vec![1, 0], 1),
            (vec![1, 1], 1),
        ]);

        assert!(do_explode(&mut value));
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0, 0, 0], 0),
                (vec![0, 0, 0, 1], 7),
                (vec![0, 0, 1], 4),
                (vec![0, 1, 0], 7),
                (vec![0, 1, 1, 0, 0], 8),
                (vec![0, 1, 1, 0, 1], 4),
                (vec![0, 1, 1, 1], 9),
                (vec![1, 0], 1),
                (vec![1, 1], 1)
            ])
        );

        assert!(do_explode(&mut value));
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0, 0, 0], 0),
                (vec![0, 0, 0, 1], 7),
                (vec![0, 0, 1], 4),
                (vec![0, 1, 0], 15),
                (vec![0, 1, 1, 0], 0),
                (vec![0, 1, 1, 1], 13),
                (vec![1, 0], 1),
                (vec![1, 1], 1)
            ])
        );

        assert!(!do_explode(&mut value));
        assert!(do_split(&mut value));
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0, 0, 0], 0),
                (vec![0, 0, 0, 1], 7),
                (vec![0, 0, 1], 4),
                (vec![0, 1, 0, 0], 7),
                (vec![0, 1, 0, 1], 8),
                (vec![0, 1, 1, 0], 0),
                (vec![0, 1, 1, 1], 13),
                (vec![1, 0], 1),
                (vec![1, 1], 1)
            ])
        );

        assert!(!do_explode(&mut value));
        assert!(do_split(&mut value));
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0, 0, 0], 0),
                (vec![0, 0, 0, 1], 7),
                (vec![0, 0, 1], 4),
                (vec![0, 1, 0, 0], 7),
                (vec![0, 1, 0, 1], 8),
                (vec![0, 1, 1, 0], 0),
                (vec![0, 1, 1, 1, 0], 6),
                (vec![0, 1, 1, 1, 1], 7),
                (vec![1, 0], 1),
                (vec![1, 1], 1)
            ])
        );

        assert!(do_explode(&mut value));
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0, 0, 0], 0),
                (vec![0, 0, 0, 1], 7),
                (vec![0, 0, 1], 4),
                (vec![0, 1, 0, 0], 7),
                (vec![0, 1, 0, 1], 8),
                (vec![0, 1, 1, 0], 6),
                (vec![0, 1, 1, 1], 0),
                (vec![1, 0], 8),
                (vec![1, 1], 1)
            ])
        );

        assert!(!do_explode(&mut value));
        assert!(!do_split(&mut value));
    }

    #[test]
    fn example_magnitude() {
        assert_eq!(
            get_magnitude(&LinkedList::from([
                (vec![0, 0], 1),
                (vec![0, 1], 2),
                (vec![1, 0, 0], 3),
                (vec![1, 0, 1], 4),
                (vec![1, 1], 5)
            ])),
            143
        );
        assert_eq!(
            get_magnitude(&LinkedList::from([
                (vec![0, 0, 0, 0], 0),
                (vec![0, 0, 0, 1], 7),
                (vec![0, 0, 1], 4),
                (vec![0, 1, 0, 0], 7),
                (vec![0, 1, 0, 1], 8),
                (vec![0, 1, 1, 0], 6),
                (vec![0, 1, 1, 1], 0),
                (vec![1, 0], 8),
                (vec![1, 1], 1)
            ])),
            1384
        );
        assert_eq!(
            get_magnitude(&LinkedList::from([
                (vec![0, 0, 0, 0], 1),
                (vec![0, 0, 0, 1], 1),
                (vec![0, 0, 1, 0], 2),
                (vec![0, 0, 1, 1], 2),
                (vec![0, 1, 0], 3),
                (vec![0, 1, 1], 3),
                (vec![1, 0], 4),
                (vec![1, 1], 4)
            ])),
            445
        );
        assert_eq!(
            get_magnitude(&LinkedList::from([
                (vec![0, 0, 0, 0], 3),
                (vec![0, 0, 0, 1], 0),
                (vec![0, 0, 1, 0], 5),
                (vec![0, 0, 1, 1], 3),
                (vec![0, 1, 0], 4),
                (vec![0, 1, 1], 4),
                (vec![1, 0], 5),
                (vec![1, 1], 5)
            ])),
            791
        );
        assert_eq!(
            get_magnitude(&LinkedList::from([
                (vec![0, 0, 0, 0], 5),
                (vec![0, 0, 0, 1], 0),
                (vec![0, 0, 1, 0], 7),
                (vec![0, 0, 1, 1], 4),
                (vec![0, 1, 0], 5),
                (vec![0, 1, 1], 5),
                (vec![1, 0], 6),
                (vec![1, 1], 6)
            ])),
            1137
        );
        assert_eq!(
            get_magnitude(&LinkedList::from([
                (vec![0, 0, 0, 0], 8),
                (vec![0, 0, 0, 1], 7),
                (vec![0, 0, 1, 0], 7),
                (vec![0, 0, 1, 1], 7),
                (vec![0, 1, 0, 0], 8),
                (vec![0, 1, 0, 1], 6),
                (vec![0, 1, 1, 0], 7),
                (vec![0, 1, 1, 1], 7),
                (vec![1, 0, 0, 0], 0),
                (vec![1, 0, 0, 1], 7),
                (vec![1, 0, 1, 0], 6),
                (vec![1, 0, 1, 1], 6),
                (vec![1, 1, 0], 8),
                (vec![1, 1, 1], 7)
            ])),
            3488
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn example_sum() {
        let mut value = LinkedList::from([
            (vec![0, 0, 0], 0),
            (vec![0, 0, 1, 0], 4),
            (vec![0, 0, 1, 1], 5),
            (vec![0, 1, 0], 0),
            (vec![0, 1, 1], 0),
            (vec![1, 0, 0, 0], 4),
            (vec![1, 0, 0, 1], 5),
            (vec![1, 0, 1, 0], 2),
            (vec![1, 0, 1, 1], 6),
            (vec![1, 1, 0], 9),
            (vec![1, 1, 1], 5),
        ]);
        value = sum(
            &value,
            &LinkedList::from([
                (vec![0], 7),
                (vec![1, 0, 0, 0], 3),
                (vec![1, 0, 0, 1], 7),
                (vec![1, 0, 1, 0], 4),
                (vec![1, 0, 1, 1], 3),
                (vec![1, 1, 0, 0], 6),
                (vec![1, 1, 0, 1], 3),
                (vec![1, 1, 1, 0], 8),
                (vec![1, 1, 1, 1], 8),
            ]),
        );
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0, 0, 0], 4),
                (vec![0, 0, 0, 1], 0),
                (vec![0, 0, 1, 0], 5),
                (vec![0, 0, 1, 1], 4),
                (vec![0, 1, 0, 0], 7),
                (vec![0, 1, 0, 1], 7),
                (vec![0, 1, 1, 0], 6),
                (vec![0, 1, 1, 1], 0),
                (vec![1, 0, 0], 8),
                (vec![1, 0, 1, 0], 7),
                (vec![1, 0, 1, 1], 7),
                (vec![1, 1, 0, 0], 7),
                (vec![1, 1, 0, 1], 9),
                (vec![1, 1, 1, 0], 5),
                (vec![1, 1, 1, 1], 0)
            ])
        );
        value = sum(
            &value,
            &LinkedList::from([
                (vec![0, 0], 2),
                (vec![0, 1, 0, 0], 0),
                (vec![0, 1, 0, 1], 8),
                (vec![0, 1, 1, 0], 3),
                (vec![0, 1, 1, 1], 4),
                (vec![1, 0, 0, 0], 6),
                (vec![1, 0, 0, 1], 7),
                (vec![1, 0, 1], 1),
                (vec![1, 1, 0], 7),
                (vec![1, 1, 1, 0], 1),
                (vec![1, 1, 1, 1], 6),
            ]),
        );
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0, 0, 0], 6),
                (vec![0, 0, 0, 1], 7),
                (vec![0, 0, 1, 0], 6),
                (vec![0, 0, 1, 1], 7),
                (vec![0, 1, 0, 0], 7),
                (vec![0, 1, 0, 1], 7),
                (vec![0, 1, 1, 0], 0),
                (vec![0, 1, 1, 1], 7),
                (vec![1, 0, 0, 0], 8),
                (vec![1, 0, 0, 1], 7),
                (vec![1, 0, 1, 0], 7),
                (vec![1, 0, 1, 1], 7),
                (vec![1, 1, 0, 0], 8),
                (vec![1, 1, 0, 1], 8),
                (vec![1, 1, 1, 0], 8),
                (vec![1, 1, 1, 1], 0)
            ])
        );
        value = sum(
            &value,
            &LinkedList::from([
                (vec![0, 0, 0, 0], 2),
                (vec![0, 0, 0, 1], 4),
                (vec![0, 0, 1], 7),
                (vec![0, 1, 0], 6),
                (vec![0, 1, 1, 0], 0),
                (vec![0, 1, 1, 1], 5),
                (vec![1, 0, 0, 0], 6),
                (vec![1, 0, 0, 1], 8),
                (vec![1, 0, 1, 0], 2),
                (vec![1, 0, 1, 1], 8),
                (vec![1, 1, 0, 0], 2),
                (vec![1, 1, 0, 1], 1),
                (vec![1, 1, 1, 0], 4),
                (vec![1, 1, 1, 1], 5),
            ]),
        );
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0, 0, 0], 7),
                (vec![0, 0, 0, 1], 0),
                (vec![0, 0, 1, 0], 7),
                (vec![0, 0, 1, 1], 7),
                (vec![0, 1, 0, 0], 7),
                (vec![0, 1, 0, 1], 7),
                (vec![0, 1, 1, 0], 7),
                (vec![0, 1, 1, 1], 8),
                (vec![1, 0, 0, 0], 7),
                (vec![1, 0, 0, 1], 7),
                (vec![1, 0, 1, 0], 8),
                (vec![1, 0, 1, 1], 8),
                (vec![1, 1, 0, 0], 7),
                (vec![1, 1, 0, 1], 7),
                (vec![1, 1, 1, 0], 8),
                (vec![1, 1, 1, 1], 7)
            ])
        );
        value = sum(
            &value,
            &LinkedList::from([
                (vec![0], 7),
                (vec![1, 0], 5),
                (vec![1, 1, 0, 0], 3),
                (vec![1, 1, 0, 1], 8),
                (vec![1, 1, 1, 0], 1),
                (vec![1, 1, 1, 1], 4),
            ]),
        );
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0, 0, 0], 7),
                (vec![0, 0, 0, 1], 7),
                (vec![0, 0, 1, 0], 7),
                (vec![0, 0, 1, 1], 8),
                (vec![0, 1, 0, 0], 9),
                (vec![0, 1, 0, 1], 5),
                (vec![0, 1, 1, 0], 8),
                (vec![0, 1, 1, 1], 7),
                (vec![1, 0, 0, 0], 6),
                (vec![1, 0, 0, 1], 8),
                (vec![1, 0, 1, 0], 0),
                (vec![1, 0, 1, 1], 8),
                (vec![1, 1, 0, 0], 9),
                (vec![1, 1, 0, 1], 9),
                (vec![1, 1, 1, 0], 9),
                (vec![1, 1, 1, 1], 0)
            ])
        );
        value = sum(
            &value,
            &LinkedList::from([
                (vec![0, 0], 2),
                (vec![0, 1, 0], 2),
                (vec![0, 1, 1], 2),
                (vec![1, 0], 8),
                (vec![1, 1, 0], 8),
                (vec![1, 1, 1], 1),
            ]),
        );
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0, 0, 0], 6),
                (vec![0, 0, 0, 1], 6),
                (vec![0, 0, 1, 0], 6),
                (vec![0, 0, 1, 1], 6),
                (vec![0, 1, 0, 0], 6),
                (vec![0, 1, 0, 1], 0),
                (vec![0, 1, 1, 0], 6),
                (vec![0, 1, 1, 1], 7),
                (vec![1, 0, 0, 0], 7),
                (vec![1, 0, 0, 1], 7),
                (vec![1, 0, 1, 0], 8),
                (vec![1, 0, 1, 1], 9),
                (vec![1, 1, 0], 8),
                (vec![1, 1, 1, 0], 8),
                (vec![1, 1, 1, 1], 1)
            ])
        );
        value = sum(&value, &LinkedList::from([(vec![0], 2), (vec![1], 9)]));
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0, 0, 0], 6),
                (vec![0, 0, 0, 1], 6),
                (vec![0, 0, 1, 0], 7),
                (vec![0, 0, 1, 1], 7),
                (vec![0, 1, 0, 0], 0),
                (vec![0, 1, 0, 1], 7),
                (vec![0, 1, 1, 0], 7),
                (vec![0, 1, 1, 1], 7),
                (vec![1, 0, 0, 0], 5),
                (vec![1, 0, 0, 1], 5),
                (vec![1, 0, 1, 0], 5),
                (vec![1, 0, 1, 1], 6),
                (vec![1, 1], 9)
            ])
        );
        value = sum(
            &value,
            &LinkedList::from([
                (vec![0], 1),
                (vec![1, 0, 0, 0], 9),
                (vec![1, 0, 0, 1], 3),
                (vec![1, 0, 1], 9),
                (vec![1, 1, 0, 0], 9),
                (vec![1, 1, 0, 1], 0),
                (vec![1, 1, 1, 0], 0),
                (vec![1, 1, 1, 1], 7),
            ]),
        );
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0, 0, 0], 7),
                (vec![0, 0, 0, 1], 8),
                (vec![0, 0, 1, 0], 6),
                (vec![0, 0, 1, 1], 7),
                (vec![0, 1, 0, 0], 6),
                (vec![0, 1, 0, 1], 8),
                (vec![0, 1, 1, 0], 0),
                (vec![0, 1, 1, 1], 8),
                (vec![1, 0, 0, 0], 7),
                (vec![1, 0, 0, 1], 7),
                (vec![1, 0, 1, 0], 5),
                (vec![1, 0, 1, 1], 0),
                (vec![1, 1, 0, 0], 5),
                (vec![1, 1, 0, 1], 5),
                (vec![1, 1, 1, 0], 5),
                (vec![1, 1, 1, 1], 6)
            ])
        );
        value = sum(
            &value,
            &LinkedList::from([
                (vec![0, 0, 0], 5),
                (vec![0, 0, 1, 0], 7),
                (vec![0, 0, 1, 1], 4),
                (vec![0, 1], 7),
                (vec![1], 1),
            ]),
        );
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0, 0, 0], 7),
                (vec![0, 0, 0, 1], 7),
                (vec![0, 0, 1, 0], 7),
                (vec![0, 0, 1, 1], 7),
                (vec![0, 1, 0, 0], 8),
                (vec![0, 1, 0, 1], 7),
                (vec![0, 1, 1, 0], 8),
                (vec![0, 1, 1, 1], 7),
                (vec![1, 0, 0, 0], 7),
                (vec![1, 0, 0, 1], 0),
                (vec![1, 0, 1, 0], 7),
                (vec![1, 0, 1, 1], 7),
                (vec![1, 1], 9)
            ])
        );
        value = sum(
            &value,
            &LinkedList::from([
                (vec![0, 0, 0, 0], 4),
                (vec![0, 0, 0, 1], 2),
                (vec![0, 0, 1], 2),
                (vec![0, 1], 6),
                (vec![1, 0], 8),
                (vec![1, 1], 7),
            ]),
        );
        assert_eq!(
            value,
            LinkedList::from([
                (vec![0, 0, 0, 0], 8),
                (vec![0, 0, 0, 1], 7),
                (vec![0, 0, 1, 0], 7),
                (vec![0, 0, 1, 1], 7),
                (vec![0, 1, 0, 0], 8),
                (vec![0, 1, 0, 1], 6),
                (vec![0, 1, 1, 0], 7),
                (vec![0, 1, 1, 1], 7),
                (vec![1, 0, 0, 0], 0),
                (vec![1, 0, 0, 1], 7),
                (vec![1, 0, 1, 0], 6),
                (vec![1, 0, 1, 1], 6),
                (vec![1, 1, 0], 8),
                (vec![1, 1, 1], 7)
            ])
        );
    }
}
