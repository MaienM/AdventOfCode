use aoc::grid::Point;
use aoc::runner::*;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Stone {
    width: usize,
    top: usize,
    points: Vec<Point>,
}
impl Stone {
    pub fn offset(&mut self, amount: usize) {
        self.top += amount;
        self.points.iter_mut().for_each(|p| p.y += amount);
    }

    pub fn apply(&self, move_: &Move) -> Self {
        match move_ {
            Move::Left => {
                if self.points[0].x == 0 {
                    return self.clone();
                }
                return Stone {
                    width: self.width,
                    top: self.top,
                    points: self.points.iter().map(|p| *p - Point::new(1, 0)).collect(),
                };
            }
            Move::Right => {
                if self.points[0].x + self.width == 7 {
                    return self.clone();
                }
                return Stone {
                    width: self.width,
                    top: self.top,
                    points: self.points.iter().map(|p| *p + Point::new(1, 0)).collect(),
                };
            }
            Move::Down => Stone {
                width: self.width,
                top: self.top - 1,
                points: self.points.iter().map(|p| *p - Point::new(0, 1)).collect(),
            },
        }
    }
}

fn get_stones() -> Vec<Stone> {
    // Stones are all aligned so that they are in the proper X position for dropping, and so that their bottom edge has y=0.
    return vec![
        Stone {
            width: 4,
            top: 0,
            points: vec![
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(4, 0),
                Point::new(5, 0),
            ],
        },
        Stone {
            width: 3,
            top: 2,
            points: vec![
                Point::new(2, 1),
                Point::new(3, 0),
                Point::new(4, 1),
                Point::new(3, 1),
                Point::new(3, 2),
            ],
        },
        Stone {
            width: 3,
            top: 2,
            points: vec![
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(4, 0),
                Point::new(4, 1),
                Point::new(4, 2),
            ],
        },
        Stone {
            width: 1,
            top: 3,
            points: vec![
                Point::new(2, 0),
                Point::new(2, 1),
                Point::new(2, 2),
                Point::new(2, 3),
            ],
        },
        Stone {
            width: 2,
            top: 1,
            points: vec![
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(2, 1),
                Point::new(3, 1),
            ],
        },
    ];
}

#[derive(Debug, Eq, PartialEq)]
enum Move {
    Left,
    Right,
    Down,
}

fn print_field(stone: &Stone, points: &HashSet<Point>, name: &str) {
    println!("== {} ==", name);
    for i in 0..stone.top {
        let y = stone.top - i;
        print!("|");
        for x in 0..7 {
            let point = Point::new(x, y);
            if stone.points.contains(&point) {
                print!("@");
            } else if points.contains(&point) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("| {}\n", y);
    }
    println!("+-------+");
    println!(" 0123456");
}

fn parse_input(input: String) -> Vec<Move> {
    return input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!(),
        })
        .collect();
}

fn do_drop<'a>(
    top: usize,
    points: &mut HashSet<Point>,
    moveloop: &mut impl Iterator<Item = &'a Move>,
    mut stone: Stone,
) -> usize {
    stone.offset(top + 1);
    stone = stone.apply(moveloop.next().unwrap());
    stone = stone.apply(moveloop.next().unwrap());
    stone = stone.apply(moveloop.next().unwrap());
    stone = stone.apply(moveloop.next().unwrap());

    'move_: loop {
        // Move down. If this fails the stone is done moving and we move to the next one.
        let after_gravity = stone.apply(&Move::Down);
        for point in after_gravity.points.iter() {
            if points.contains(point) {
                break 'move_;
            }
        }

        // Apply wind movement. If this fails the stone doesn't move, but it does continue to fall.
        let after_wind = after_gravity.apply(moveloop.next().unwrap());
        for point in after_wind.points.iter() {
            if points.contains(point) {
                stone = after_gravity;
                continue 'move_;
            }
        }
        stone = after_wind;
    }

    let top = usize::max(top, stone.top);

    stone.points.into_iter().for_each(|p| {
        points.insert(p);
    });

    return top;
}

fn do_drops<'a>(
    mut top: usize,
    points: &mut HashSet<Point>,
    moveloop: &mut impl Iterator<Item = &'a Move>,
    stoneloop: &mut impl Iterator<Item = &'a Stone>,
    drops: usize,
) -> usize {
    for _ in 0..drops {
        top = do_drop(top, points, moveloop, stoneloop.next().unwrap().clone());
    }
    return top;
}

fn simulate(input: String, cycles: usize) -> (usize, HashSet<Point>) {
    let moves = parse_input(input);
    let stones = get_stones();

    let mut moveloop = moves.iter().cycle();
    let mut stoneloop = stones.iter().cycle();

    let mut points: HashSet<Point> = HashSet::new();
    let mut top = 0;
    for x in 0..7 {
        points.insert(Point::new(x, 0));
    }

    // Run a full cycles until we detect a stable loop.
    let mut finished = 0;
    let mut changes = Vec::new();
    let mut loop_size = 0;
    let mut change_per_loop = 0;
    'findloop: while finished < cycles {
        let drops = usize::min(cycles - finished, stones.len());
        finished += drops;
        let new_top = do_drops(top, &mut points, &mut moveloop, &mut stoneloop, drops);
        changes.push(new_top - top);
        top = new_top;

        let cl = changes.len();
        for len in 5..=(cl / 2) {
            if changes[(cl - len)..cl] == changes[(cl - 2 * len)..(cl - len)] {
                loop_size = stones.len() * len;
                change_per_loop = changes[(cl - len)..cl].into_iter().sum();
                println!(
                    "Found loop with length {}, tower growing {} per iteration.",
                    loop_size, change_per_loop
                );
                break 'findloop;
            }
        }
    }
    if cycles == finished {
        return (top, points);
    }

    // Figure out how many times the loop needs to be repeated.
    let loops = (cycles - finished) / loop_size;
    finished += loops * loop_size;

    // Run the leftover cycles.
    top = do_drops(
        top,
        &mut points,
        &mut moveloop,
        &mut stoneloop,
        cycles - finished,
    );

    // Apply the loops to the height.
    top += loops * change_per_loop;

    return (top, points);
}

pub fn part1(input: String) -> usize {
    return simulate(input, 2_022).0;
}

pub fn part2(input: String) -> usize {
    return simulate(input, 1_000_000_000_000).0;
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = vec![
            Move::Right,
            Move::Right,
            Move::Right,
            Move::Left,
            Move::Left,
            Move::Right,
            Move::Left,
            Move::Right,
            Move::Right,
            Move::Left,
            Move::Left,
            Move::Left,
            Move::Right,
            Move::Right,
            Move::Left,
            Move::Right,
            Move::Right,
            Move::Right,
            Move::Left,
            Move::Left,
            Move::Left,
            Move::Right,
            Move::Right,
            Move::Right,
            Move::Left,
            Move::Left,
            Move::Left,
            Move::Right,
            Move::Left,
            Move::Left,
            Move::Left,
            Move::Right,
            Move::Right,
            Move::Left,
            Move::Right,
            Move::Right,
            Move::Left,
            Move::Left,
            Move::Right,
            Move::Right,
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 3_068);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 1_514_285_714_288);
    }

    #[test]
    fn example_do_drop() {
        let mut before = HashSet::new();
        for x in 0..7 {
            before.insert(Point::new(x, 0));
        }

        let (top, points) = simulate(EXAMPLE_INPUT.to_string(), 1);
        assert_eq!(top, 1);
        assert_eq!(
            points.difference(&before).collect::<HashSet<&Point>>(),
            vec![
                Point::new(2, 1),
                Point::new(3, 1),
                Point::new(4, 1),
                Point::new(5, 1),
            ]
            .iter()
            .collect(),
        );

        let before = points;
        let (top, points) = simulate(EXAMPLE_INPUT.to_string(), 2);
        assert_eq!(top, 4);
        assert_eq!(
            points.difference(&before).collect::<HashSet<&Point>>(),
            vec![
                Point::new(3, 2),
                Point::new(2, 3),
                Point::new(3, 3),
                Point::new(4, 3),
                Point::new(3, 4),
            ]
            .iter()
            .collect(),
        );

        let before = points;
        let (top, points) = simulate(EXAMPLE_INPUT.to_string(), 3);
        assert_eq!(top, 6);
        assert_eq!(
            points.difference(&before).collect::<HashSet<&Point>>(),
            vec![
                Point::new(0, 4),
                Point::new(1, 4),
                Point::new(2, 4),
                Point::new(2, 5),
                Point::new(2, 6),
            ]
            .iter()
            .collect(),
        );

        let before = points;
        let (top, points) = simulate(EXAMPLE_INPUT.to_string(), 4);
        assert_eq!(top, 7);
        assert_eq!(
            points.difference(&before).collect::<HashSet<&Point>>(),
            vec![
                Point::new(4, 4),
                Point::new(4, 5),
                Point::new(4, 6),
                Point::new(4, 7),
            ]
            .iter()
            .collect(),
        );

        let before = points;
        let (top, points) = simulate(EXAMPLE_INPUT.to_string(), 5);
        assert_eq!(top, 9);
        assert_eq!(
            points.difference(&before).collect::<HashSet<&Point>>(),
            vec![
                Point::new(4, 8),
                Point::new(5, 8),
                Point::new(4, 9),
                Point::new(5, 9),
            ]
            .iter()
            .collect(),
        );

        let before = points;
        let (top, points) = simulate(EXAMPLE_INPUT.to_string(), 6);
        assert_eq!(top, 10);
        assert_eq!(
            points.difference(&before).collect::<HashSet<&Point>>(),
            vec![
                Point::new(1, 10),
                Point::new(2, 10),
                Point::new(3, 10),
                Point::new(4, 10),
            ]
            .iter()
            .collect(),
        );

        let before = points;
        let (top, points) = simulate(EXAMPLE_INPUT.to_string(), 7);
        assert_eq!(top, 13);
        assert_eq!(
            points.difference(&before).collect::<HashSet<&Point>>(),
            vec![
                Point::new(2, 11),
                Point::new(1, 12),
                Point::new(2, 12),
                Point::new(3, 12),
                Point::new(2, 13),
            ]
            .iter()
            .collect(),
        );

        let before = points;
        let (top, points) = simulate(EXAMPLE_INPUT.to_string(), 8);
        assert_eq!(top, 15);
        assert_eq!(
            points.difference(&before).collect::<HashSet<&Point>>(),
            vec![
                Point::new(3, 13),
                Point::new(4, 13),
                Point::new(5, 13),
                Point::new(5, 14),
                Point::new(5, 15),
            ]
            .iter()
            .collect(),
        );

        let before = points;
        let (top, points) = simulate(EXAMPLE_INPUT.to_string(), 9);
        assert_eq!(top, 17);
        assert_eq!(
            points.difference(&before).collect::<HashSet<&Point>>(),
            vec![
                Point::new(4, 14),
                Point::new(4, 15),
                Point::new(4, 16),
                Point::new(4, 17),
            ]
            .iter()
            .collect(),
        );

        let before = points;
        let (top, points) = simulate(EXAMPLE_INPUT.to_string(), 10);
        assert_eq!(top, 17);
        assert_eq!(
            points.difference(&before).collect::<HashSet<&Point>>(),
            vec![
                Point::new(0, 13),
                Point::new(1, 13),
                Point::new(0, 14),
                Point::new(1, 14),
            ]
            .iter()
            .collect(),
        );
    }
}
