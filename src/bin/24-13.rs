use aoc::utils::{parse, point::Point2};

#[derive(Debug, PartialEq, Eq)]
struct Game {
    button_a: Point2,
    button_b: Point2,
    prize: Point2,
}

fn parse_input(input: &str) -> Vec<Game> {
    parse!(input => {
        [games split on "\n\n" with
            {
                "Button A: X+" [xa as usize] ", Y+" [ya as usize] '\n'
                "Button B: X+" [xb as usize] ", Y+" [yb as usize] '\n'
                "Prize: X=" [xp as usize] ", Y=" [yp as usize]
            } => {
                Game {
                    button_a: Point2::new(xa, ya),
                    button_b: Point2::new(xb, yb),
                    prize: Point2::new(xp, yp),
                }
            }
        ]
    } => games)
}

pub fn part1(input: &str) -> usize {
    let games = parse_input(input);
    games
        .into_iter()
        .filter_map(|game| {
            (1..usize::min(100, game.prize.x / game.button_a.x + 1))
                .filter_map(|a| {
                    let b = (game.prize.x - (game.button_a.x * a)) / game.button_b.x;
                    if game.button_a.x * a + game.button_b.x * b != game.prize.x
                        || game.button_a.y * a + game.button_b.y * b != game.prize.y
                    {
                        return None;
                    }
                    Some(a * 3 + b)
                })
                .min()
        })
        .sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 480)]
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
                button_a: Point2::new(94, 34),
                button_b: Point2::new(22, 67),
                prize: Point2::new(8400, 5400),
            },
            Game {
                button_a: Point2::new(26, 66),
                button_b: Point2::new(67, 21),
                prize: Point2::new(12748, 12176),
            },
            Game {
                button_a: Point2::new(17, 86),
                button_b: Point2::new(84, 37),
                prize: Point2::new(7870, 6450),
            },
            Game {
                button_a: Point2::new(69, 23),
                button_b: Point2::new(27, 71),
                prize: Point2::new(18641, 10279),
            },
        ];
        assert_eq!(actual, expected);
    }
}
