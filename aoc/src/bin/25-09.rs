puzzle_runner::register_chapter!(book = "2025", title = "Movie Theater");

use puzzle_lib::point::Point2;

fn parse_input(input: &str) -> Vec<Point2> {
    parse!(input => {
        [points split on '\n' with
            { [x as usize] ',' [y as usize] }
            => Point2::new(x, y)
        ]
    } => points)
}

pub fn part1(input: &str) -> usize {
    let points = parse_input(input);
    points
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 50)]
    static EXAMPLE_INPUT: &str = "
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    ";
}
