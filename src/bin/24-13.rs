use aoc::utils::{matrix::Matrix, parse, point::Point2};

type Point = Point2<u64>;

#[derive(Debug, PartialEq, Eq)]
struct Game {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

fn parse_input(input: &str) -> Vec<Game> {
    parse!(input => {
        [games split on "\n\n" with
            {
                "Button A: X+" [xa as u64] ", Y+" [ya as u64] '\n'
                "Button B: X+" [xb as u64] ", Y+" [yb as u64] '\n'
                "Prize: X=" [xp as u64] ", Y=" [yp as u64]
            } => {
                Game {
                    button_a: Point::new(xa, ya),
                    button_b: Point::new(xb, yb),
                    prize: Point::new(xp, yp),
                }
            }
        ]
    } => games)
}

fn find_wincondition(game: &Game) -> Option<u64> {
    let mut matrix = Matrix::new([
        [
            game.button_a.x as f64,
            game.button_b.x as f64,
            game.prize.x as f64,
        ],
        [
            game.button_a.y as f64,
            game.button_b.y as f64,
            game.prize.y as f64,
        ],
    ]);
    matrix.gauss_jordan_elimination();
    let a = matrix[0][2].round() as u64;
    let b = matrix[1][2].round() as u64;
    if game.button_a.x * a + game.button_b.x * b == game.prize.x
        && game.button_a.y * a + game.button_b.y * b == game.prize.y
    {
        Some(a * 3 + b)
    } else {
        None
    }
}

pub fn part1(input: &str) -> u64 {
    let games = parse_input(input);
    games
        .into_iter()
        .filter_map(|game| find_wincondition(&game))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let games = parse_input(input);
    games
        .into_iter()
        .filter_map(|mut game| {
            game.prize.x += 10_000_000_000_000;
            game.prize.y += 10_000_000_000_000;
            find_wincondition(&game)
        })
        .sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 480, part2 = 875_318_608_908)]
    static EXAMPLE_INPUT: &str = "
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            Game {
                button_a: Point::new(94, 34),
                button_b: Point::new(22, 67),
                prize: Point::new(8400, 5400),
            },
            Game {
                button_a: Point::new(26, 66),
                button_b: Point::new(67, 21),
                prize: Point::new(12748, 12176),
            },
            Game {
                button_a: Point::new(17, 86),
                button_b: Point::new(84, 37),
                prize: Point::new(7870, 6450),
            },
            Game {
                button_a: Point::new(69, 23),
                button_b: Point::new(27, 71),
                prize: Point::new(18641, 10279),
            },
        ];
        assert_eq!(actual, expected);
    }
}
