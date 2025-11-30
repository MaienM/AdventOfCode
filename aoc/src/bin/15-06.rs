puzzle_lib::setup!(title = "Probably a Fire Hazard");

use puzzle_lib::{
    grid::FullGrid,
    point::{Point2, Point2Range},
};

#[derive(Debug, Eq, PartialEq)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

type Instruction = (Action, Point2Range<usize>);

fn parse_input(input: &str) -> Vec<Instruction> {
    parse!(input => {
        [instructions split on '\n' with
            {
                [action matching /r"\D+"/ match {
                    "turn on " => Action::TurnOn,
                    "turn off " => Action::TurnOff,
                    "toggle " => Action::Toggle,
                }]
                [x1 as usize]
                ','
                [y1 as usize]
                " through "
                [x2 as usize]
                ','
                [y2 as usize]
            }
            => (action, (Point2::new(x1, y1)..=Point2::new(x2, y2)).into())
        ]
    } => instructions)
}

pub fn part1(input: &str) -> usize {
    let instructions = parse_input(input);
    let mut grid = FullGrid::new_default(1000, 1000);
    for (action, range) in instructions {
        match action {
            Action::TurnOn => {
                for point in range {
                    grid[point] = true;
                }
            }
            Action::TurnOff => {
                for point in range {
                    grid[point] = false;
                }
            }
            Action::Toggle => {
                for point in range {
                    grid[point] = !grid[point];
                }
            }
        }
    }
    grid.into_iter_data().filter(|v| *v).count()
}

pub fn part2(input: &str) -> usize {
    let instructions = parse_input(input);
    let mut grid = FullGrid::<u8>::new_default(1000, 1000);
    for (action, range) in instructions {
        match action {
            Action::TurnOn => {
                for point in range {
                    grid[point] += 1;
                }
            }
            Action::TurnOff => {
                for point in range {
                    grid[point] = grid[point].saturating_sub(1);
                }
            }
            Action::Toggle => {
                for point in range {
                    grid[point] += 2;
                }
            }
        }
    }
    grid.into_iter_data().map_into::<usize>().sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 998_996, part2 = 1_001_996)]
    static EXAMPLE_INPUT: &str = "
        turn on 0,0 through 999,999
        toggle 0,0 through 999,0
        turn off 499,499 through 500,500
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            (
                Action::TurnOn,
                (Point2::new(0, 0)..=Point2::new(999, 999)).into(),
            ),
            (
                Action::Toggle,
                (Point2::new(0, 0)..=Point2::new(999, 0)).into(),
            ),
            (
                Action::TurnOff,
                (Point2::new(499, 499)..=Point2::new(500, 500)).into(),
            ),
        ];
        assert_eq!(actual, expected);
    }
}
