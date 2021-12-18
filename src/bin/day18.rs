use std::{collections::VecDeque, fmt::Debug, iter::Peekable, str::Chars};

use aoc::runner::*;

#[derive(Eq, PartialEq)]
struct Pair(Item, Item);
impl Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "[{:?},{:?}]", self.0, self.1);
    }
}

#[derive(Eq, PartialEq)]
enum Item {
    Pair(Box<Pair>),
    Num(i8),
}
impl Item {
    fn as_pair(&mut self) -> &mut Box<Pair> {
        return match self {
            Item::Pair(pair) => pair,
            _ => panic!("Expected a pair, got {:?}.", self),
        };
    }

    fn as_num(&self) -> i8 {
        return match self {
            Item::Num(num) => *num,
            _ => panic!("Expected a num, got {:?}.", self),
        };
    }
}
impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pair(pair) => write!(f, "{:?}", pair),
            Self::Num(num) => write!(f, "{}", num),
        }
    }
}

type Path = VecDeque<i8>;

fn parse_item(chars: &mut Peekable<Chars>) -> Item {
    return match chars.peek().unwrap() {
        '[' => Item::Pair(Box::new(parse_pair(chars))),
        _ => Item::Num(chars.next().unwrap().to_digit(10).unwrap() as i8),
    };
}

fn parse_pair(chars: &mut Peekable<Chars>) -> Pair {
    assert_eq!(chars.next().unwrap(), '[');
    let left = parse_item(chars);
    assert_eq!(chars.next().unwrap(), ',');
    let right = parse_item(chars);
    assert_eq!(chars.next().unwrap(), ']');
    return Pair(left, right);
}

fn parse_input(input: String) -> Vec<Pair> {
    return input
        .trim()
        .split("\n")
        .map(str::trim)
        .map(|line| {
            let mut chars = line.chars().peekable();
            return parse_pair(&mut chars);
        })
        .collect();
}

fn get_numeric_paths(pair: &Pair, path: &mut Path, found: &mut Vec<Path>) {
    path.push_back(0);
    match &pair.0 {
        Item::Pair(p) => get_numeric_paths(&p, path, found),
        Item::Num(_) => found.push(path.clone()),
    }
    path.pop_back();
    path.push_back(1);
    match &pair.1 {
        Item::Pair(p) => get_numeric_paths(&p, path, found),
        Item::Num(_) => found.push(path.clone()),
    }
    path.pop_back();
}

fn find_explode_target(pair: &Pair, depth: i8) -> Option<Path> {
    if depth >= 4 {
        match pair {
            Pair(Item::Pair(_), _) => {
                return Some(vec![0].into());
            }
            Pair(_, Item::Pair(_)) => {
                return Some(vec![1].into());
            }
            _ => {
                return None;
            }
        }
    }

    match &pair.0 {
        Item::Pair(p) => {
            let path = find_explode_target(&p, depth + 1);
            if path.is_some() {
                return path.map(|mut p| {
                    p.push_front(0);
                    return p;
                });
            }
        }
        _ => {}
    }
    match &pair.1 {
        Item::Pair(p) => {
            let path = find_explode_target(&p, depth + 1);
            if path.is_some() {
                return path.map(|mut p| {
                    p.push_front(1);
                    return p;
                });
            }
        }
        _ => {}
    }
    return None;
}

fn get_at_path<'a>(root: &'a mut Pair, mut path: Path) -> &'a mut Item {
    let mut item = match path.pop_front().unwrap() {
        0 => &mut root.0,
        1 => &mut root.1,
        _ => panic!("NONONONONONONO"),
    };
    for side in path {
        match (side, item) {
            (0, Item::Pair(p)) => {
                item = &mut p.0;
            }
            (1, Item::Pair(p)) => {
                item = &mut p.1;
            }
            _ => panic!("AAAH"),
        }
    }
    return item;
}

fn add_to_num_at_path(root: &mut Pair, mut path: Path, num: i8) {
    let side = path.pop_back();
    let pair = if path.is_empty() {
        root
    } else {
        get_at_path(root, path).as_pair()
    };
    match side {
        Some(0) => {
            pair.0 = Item::Num(pair.0.as_num() + num);
        }
        Some(1) => {
            pair.1 = Item::Num(pair.1.as_num() + num);
        }
        _ => panic!("How could this happen to me."),
    }
}

fn do_explode(root: &mut Pair) -> bool {
    let target_path = find_explode_target(root, 1);
    if target_path.is_none() {
        return false;
    }

    let mut numeric_paths: Vec<Path> = Vec::new();
    get_numeric_paths(&root, &mut VecDeque::new(), &mut numeric_paths);

    let target_path = target_path.unwrap();
    let target_item = get_at_path(root, target_path.clone());
    let target_pair = target_item.as_pair();
    let target_num_left = target_pair.0.as_num();
    let target_num_right = target_pair.1.as_num();
    *target_item = Item::Num(0);

    let partition_point = numeric_paths.partition_point(|p| p < &target_path);

    if partition_point > 0 {
        add_to_num_at_path(
            root,
            numeric_paths[partition_point - 1].clone(),
            target_num_left,
        );
    }
    if partition_point <= numeric_paths.len() - 3 {
        // The item at and after the partition point are the ones in the target pair, so skip those.
        add_to_num_at_path(
            root,
            numeric_paths[partition_point + 2].clone(),
            target_num_right,
        );
    }

    return true;
}

fn do_split(pair: &mut Pair) -> bool {
    match &mut pair.0 {
        Item::Pair(p) => {
            if do_split(p) {
                return true;
            }
        }
        Item::Num(n) => {
            if *n > 9 {
                pair.0 = Item::Pair(Box::new(Pair(Item::Num(*n / 2), Item::Num((*n + 1) / 2))));
                return true;
            }
        }
    }
    match &mut pair.1 {
        Item::Pair(p) => {
            return do_split(p);
        }
        Item::Num(n) => {
            if *n > 9 {
                pair.1 = Item::Pair(Box::new(Pair(Item::Num(*n / 2), Item::Num((*n + 1) / 2))));
                return true;
            }
        }
    }
    return false;
}

fn do_reduce(pair: &mut Pair) {
    while do_explode(pair) || do_split(pair) {}
}

fn sum(left: Pair, right: Pair) -> Pair {
    let mut result = Pair(Item::Pair(Box::new(left)), Item::Pair(Box::new(right)));
    do_reduce(&mut result);
    return result;
}

fn get_magnitude(pair: &Pair) -> i32 {
    let left = match &pair.0 {
        Item::Pair(p) => get_magnitude(p),
        Item::Num(n) => *n as i32,
    };
    let right = match &pair.1 {
        Item::Pair(p) => get_magnitude(p),
        Item::Num(n) => *n as i32,
    };
    return left * 3 + right * 2;
}

fn part1(input: String) -> i32 {
    let mut pairs: VecDeque<Pair> = parse_input(input).into();
    let mut result = pairs.pop_front().unwrap();
    for pair in pairs {
        result = sum(result, pair);
    }
    return get_magnitude(&result);
}

fn main() {
    run(part1, missing::<i8>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn example_parse_pair() {
        assert_eq!(
            parse_input("[1,2]".to_string()),
            vec![Pair(Item::Num(1), Item::Num(2))]
        );
        assert_eq!(
            parse_input("[[1,2],3]".to_string()),
            vec![Pair(
                Item::Pair(Box::new(Pair(Item::Num(1), Item::Num(2)))),
                Item::Num(3)
            )]
        );
        assert_eq!(
            parse_input(
                "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]".to_string()
            ),
            vec![Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(1), Item::Num(3)))),
                        Item::Pair(Box::new(Pair(Item::Num(5), Item::Num(3)))),
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(1), Item::Num(3)))),
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(7)))),
                    ))),
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(4), Item::Num(9)))),
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(9)))),
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(2)))),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(3)))),
                    ))),
                ))),
            )]
        );
    }

    #[test]
    fn example_do_explode() {
        let mut value = Pair(
            Item::Pair(Box::new(Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(9), Item::Num(8)))),
                        Item::Num(1),
                    ))),
                    Item::Num(2),
                ))),
                Item::Num(3),
            ))),
            Item::Num(4),
        );
        assert!(do_explode(&mut value));
        assert_eq!(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(9)))),
                        Item::Num(2)
                    ))),
                    Item::Num(3)
                ))),
                Item::Num(4)
            )
        );

        let mut value = Pair(
            Item::Num(7),
            Item::Pair(Box::new(Pair(
                Item::Num(6),
                Item::Pair(Box::new(Pair(
                    Item::Num(5),
                    Item::Pair(Box::new(Pair(
                        Item::Num(4),
                        Item::Pair(Box::new(Pair(Item::Num(3), Item::Num(2)))),
                    ))),
                ))),
            ))),
        );
        assert!(do_explode(&mut value));
        assert_eq!(
            value,
            Pair(
                Item::Num(7),
                Item::Pair(Box::new(Pair(
                    Item::Num(6),
                    Item::Pair(Box::new(Pair(
                        Item::Num(5),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(0))))
                    )))
                )))
            )
        );

        let mut value = Pair(
            Item::Pair(Box::new(Pair(
                Item::Num(6),
                Item::Pair(Box::new(Pair(
                    Item::Num(5),
                    Item::Pair(Box::new(Pair(
                        Item::Num(4),
                        Item::Pair(Box::new(Pair(Item::Num(3), Item::Num(2)))),
                    ))),
                ))),
            ))),
            Item::Num(1),
        );
        assert!(do_explode(&mut value));
        assert_eq!(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Num(6),
                    Item::Pair(Box::new(Pair(
                        Item::Num(5),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(0))))
                    )))
                ))),
                Item::Num(3)
            )
        );

        let mut value = Pair(
            Item::Pair(Box::new(Pair(
                Item::Num(3),
                Item::Pair(Box::new(Pair(
                    Item::Num(2),
                    Item::Pair(Box::new(Pair(
                        Item::Num(1),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(3)))),
                    ))),
                ))),
            ))),
            Item::Pair(Box::new(Pair(
                Item::Num(6),
                Item::Pair(Box::new(Pair(
                    Item::Num(5),
                    Item::Pair(Box::new(Pair(
                        Item::Num(4),
                        Item::Pair(Box::new(Pair(Item::Num(3), Item::Num(2)))),
                    ))),
                ))),
            ))),
        );
        assert!(do_explode(&mut value));
        assert_eq!(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Num(3),
                    Item::Pair(Box::new(Pair(
                        Item::Num(2),
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(0))))
                    )))
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Num(9),
                    Item::Pair(Box::new(Pair(
                        Item::Num(5),
                        Item::Pair(Box::new(Pair(
                            Item::Num(4),
                            Item::Pair(Box::new(Pair(Item::Num(3), Item::Num(2))))
                        )))
                    )))
                )))
            )
        );

        let mut value = Pair(
            Item::Pair(Box::new(Pair(
                Item::Num(3),
                Item::Pair(Box::new(Pair(
                    Item::Num(2),
                    Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(0)))),
                ))),
            ))),
            Item::Pair(Box::new(Pair(
                Item::Num(9),
                Item::Pair(Box::new(Pair(
                    Item::Num(5),
                    Item::Pair(Box::new(Pair(
                        Item::Num(4),
                        Item::Pair(Box::new(Pair(Item::Num(3), Item::Num(2)))),
                    ))),
                ))),
            ))),
        );
        assert!(do_explode(&mut value));
        assert_eq!(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Num(3),
                    Item::Pair(Box::new(Pair(
                        Item::Num(2),
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(0))))
                    )))
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Num(9),
                    Item::Pair(Box::new(Pair(
                        Item::Num(5),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(0))))
                    )))
                )))
            )
        );
    }

    #[test]
    fn example_do_actions() {
        let mut pair = Pair(
            Item::Pair(Box::new(Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(4), Item::Num(3)))),
                        Item::Num(4),
                    ))),
                    Item::Num(4),
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Num(7),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(4)))),
                        Item::Num(9),
                    ))),
                ))),
            ))),
            Item::Pair(Box::new(Pair(Item::Num(1), Item::Num(1)))),
        );
        assert!(do_explode(&mut pair));
        assert_eq!(
            pair,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(7)))),
                        Item::Num(4)
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Num(7),
                        Item::Pair(Box::new(Pair(
                            Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(4)))),
                            Item::Num(9)
                        )))
                    )))
                ))),
                Item::Pair(Box::new(Pair(Item::Num(1), Item::Num(1))))
            )
        );
        assert!(do_explode(&mut pair));
        assert_eq!(
            pair,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(7)))),
                        Item::Num(4)
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Num(15),
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(13))))
                    )))
                ))),
                Item::Pair(Box::new(Pair(Item::Num(1), Item::Num(1))))
            )
        );
        assert!(!do_explode(&mut pair));
        assert!(do_split(&mut pair));
        assert_eq!(
            pair,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(7)))),
                        Item::Num(4)
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(8)))),
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(13))))
                    )))
                ))),
                Item::Pair(Box::new(Pair(Item::Num(1), Item::Num(1))))
            )
        );
        assert!(!do_explode(&mut pair));
        assert!(do_split(&mut pair));
        assert_eq!(
            pair,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(7)))),
                        Item::Num(4)
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(8)))),
                        Item::Pair(Box::new(Pair(
                            Item::Num(0),
                            Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(7))))
                        )))
                    )))
                ))),
                Item::Pair(Box::new(Pair(Item::Num(1), Item::Num(1))))
            )
        );
        assert!(do_explode(&mut pair));
        assert_eq!(
            pair,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(7)))),
                        Item::Num(4)
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(8)))),
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(0))))
                    )))
                ))),
                Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(1))))
            )
        );
        assert!(!do_explode(&mut pair));
        assert!(!do_split(&mut pair));
    }

    #[test]
    fn example_magnitude() {
        assert_eq!(
            get_magnitude(&Pair(
                Item::Pair(Box::new(Pair(Item::Num(1), Item::Num(2)))),
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(Item::Num(3), Item::Num(4)))),
                    Item::Num(5)
                )))
            )),
            143
        );
        assert_eq!(
            get_magnitude(&Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(7)))),
                        Item::Num(4)
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(8)))),
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(0))))
                    )))
                ))),
                Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(1))))
            )),
            1384
        );
        assert_eq!(
            get_magnitude(&Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(1), Item::Num(1)))),
                        Item::Pair(Box::new(Pair(Item::Num(2), Item::Num(2))))
                    ))),
                    Item::Pair(Box::new(Pair(Item::Num(3), Item::Num(3))))
                ))),
                Item::Pair(Box::new(Pair(Item::Num(4), Item::Num(4))))
            )),
            445
        );
        assert_eq!(
            get_magnitude(&Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(3), Item::Num(0)))),
                        Item::Pair(Box::new(Pair(Item::Num(5), Item::Num(3))))
                    ))),
                    Item::Pair(Box::new(Pair(Item::Num(4), Item::Num(4))))
                ))),
                Item::Pair(Box::new(Pair(Item::Num(5), Item::Num(5))))
            )),
            791
        );
        assert_eq!(
            get_magnitude(&Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(5), Item::Num(0)))),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(4))))
                    ))),
                    Item::Pair(Box::new(Pair(Item::Num(5), Item::Num(5))))
                ))),
                Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(6))))
            )),
            1137
        );
        assert_eq!(
            get_magnitude(&Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7))))
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(6)))),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7))))
                    )))
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(6))))
                    ))),
                    Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(7))))
                )))
            )),
            3488
        );
    }

    #[test]
    fn example_sum() {
        let mut value = Pair(
            Item::Pair(Box::new(Pair(
                Item::Pair(Box::new(Pair(
                    Item::Num(0),
                    Item::Pair(Box::new(Pair(Item::Num(4), Item::Num(5)))),
                ))),
                Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(0)))),
            ))),
            Item::Pair(Box::new(Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(Item::Num(4), Item::Num(5)))),
                    Item::Pair(Box::new(Pair(Item::Num(2), Item::Num(6)))),
                ))),
                Item::Pair(Box::new(Pair(Item::Num(9), Item::Num(5)))),
            ))),
        );
        value = sum(
            value,
            Pair(
                Item::Num(7),
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(3), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(4), Item::Num(3)))),
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(3)))),
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(8)))),
                    ))),
                ))),
            ),
        );
        assert_eq!(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(4), Item::Num(0)))),
                        Item::Pair(Box::new(Pair(Item::Num(5), Item::Num(4))))
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(0))))
                    )))
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Num(8),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7))))
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(9)))),
                        Item::Pair(Box::new(Pair(Item::Num(5), Item::Num(0))))
                    )))
                )))
            )
        );

        value = sum(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Num(2),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(8)))),
                        Item::Pair(Box::new(Pair(Item::Num(3), Item::Num(4)))),
                    ))),
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(7)))),
                        Item::Num(1),
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Num(7),
                        Item::Pair(Box::new(Pair(Item::Num(1), Item::Num(6)))),
                    ))),
                ))),
            ),
        );
        assert_eq!(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(7))))
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(7))))
                    )))
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7))))
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(8)))),
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(0))))
                    )))
                )))
            )
        );

        value = sum(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(2), Item::Num(4)))),
                        Item::Num(7),
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Num(6),
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(5)))),
                    ))),
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(8)))),
                        Item::Pair(Box::new(Pair(Item::Num(2), Item::Num(8)))),
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(2), Item::Num(1)))),
                        Item::Pair(Box::new(Pair(Item::Num(4), Item::Num(5)))),
                    ))),
                ))),
            ),
        );
        assert_eq!(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(0)))),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7))))
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(8))))
                    )))
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(8))))
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(7))))
                    )))
                )))
            )
        );

        value = sum(
            value,
            Pair(
                Item::Num(7),
                Item::Pair(Box::new(Pair(
                    Item::Num(5),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(3), Item::Num(8)))),
                        Item::Pair(Box::new(Pair(Item::Num(1), Item::Num(4)))),
                    ))),
                ))),
            ),
        );
        assert_eq!(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(8))))
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(9), Item::Num(5)))),
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(7))))
                    )))
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(8)))),
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(8))))
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(9), Item::Num(9)))),
                        Item::Pair(Box::new(Pair(Item::Num(9), Item::Num(0))))
                    )))
                )))
            )
        );

        value = sum(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Num(2),
                    Item::Pair(Box::new(Pair(Item::Num(2), Item::Num(2)))),
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Num(8),
                    Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(1)))),
                ))),
            ),
        );
        assert_eq!(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(6)))),
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(6))))
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(0)))),
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(7))))
                    )))
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(9))))
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Num(8),
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(1))))
                    )))
                )))
            )
        );

        value = sum(value, Pair(Item::Num(2), Item::Num(9)));
        assert_eq!(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(6)))),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7))))
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7))))
                    )))
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(5), Item::Num(5)))),
                        Item::Pair(Box::new(Pair(Item::Num(5), Item::Num(6))))
                    ))),
                    Item::Num(9)
                )))
            )
        );

        value = sum(
            value,
            Pair(
                Item::Num(1),
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(9), Item::Num(3)))),
                        Item::Num(9),
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(9), Item::Num(0)))),
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(7)))),
                    ))),
                ))),
            ),
        );
        assert_eq!(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(8)))),
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(7))))
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(8)))),
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(8))))
                    )))
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(5), Item::Num(0))))
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(5), Item::Num(5)))),
                        Item::Pair(Box::new(Pair(Item::Num(5), Item::Num(6))))
                    )))
                )))
            )
        );

        value = sum(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Num(5),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(4)))),
                    ))),
                    Item::Num(7),
                ))),
                Item::Num(1),
            ),
        );
        assert_eq!(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7))))
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(7))))
                    )))
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(0)))),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7))))
                    ))),
                    Item::Num(9)
                )))
            )
        );

        value = sum(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(4), Item::Num(2)))),
                        Item::Num(2),
                    ))),
                    Item::Num(6),
                ))),
                Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(7)))),
            ),
        );
        assert_eq!(
            value,
            Pair(
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7))))
                    ))),
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(6)))),
                        Item::Pair(Box::new(Pair(Item::Num(7), Item::Num(7))))
                    )))
                ))),
                Item::Pair(Box::new(Pair(
                    Item::Pair(Box::new(Pair(
                        Item::Pair(Box::new(Pair(Item::Num(0), Item::Num(7)))),
                        Item::Pair(Box::new(Pair(Item::Num(6), Item::Num(6))))
                    ))),
                    Item::Pair(Box::new(Pair(Item::Num(8), Item::Num(7))))
                )))
            )
        );
    }

    #[test]
    fn example_part1() {
        assert_eq!(
            part1(
                "
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
                "
                .to_string()
            ),
            4140
        );
    }
}
