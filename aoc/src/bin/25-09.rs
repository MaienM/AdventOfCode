puzzle_runner::register_chapter!(book = "2025", title = "Movie Theater");

use std::ops::RangeInclusive;

use puzzle_lib::point::Point2;

fn parse_input(input: &str) -> Vec<Point2> {
    parse!(input => {
        [points split on '\n' with
            { [x as usize] ',' [y as usize] }
            => Point2::new(x, y)
        ]
    } => points)
}

enum Line {
    Horizontal(usize, RangeInclusive<usize>),
    Vertical(usize, RangeInclusive<usize>),
}

fn minmax(a: usize, b: usize) -> (usize, usize) {
    if a < b { (a, b) } else { (b, a) }
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

pub fn part2(input: &str) -> usize {
    let points = parse_input(input);
    let mut lines: Vec<Line> = Vec::new();
    for (a, b) in points.iter().zip(points.iter().cycle().skip(1)) {
        let x = minmax(a.x, b.x);
        let y = minmax(a.y, b.y);
        if x.0 == x.1 {
            lines.push(Line::Vertical(x.0, y.0..=y.1));
        } else {
            lines.push(Line::Horizontal(y.0, x.0..=x.1));
        }
    }
    points
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let size = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);
            (a, b, size)
        })
        .sorted_unstable_by_key(|(_, _, size)| *size)
        .rev()
        .find(|(a, b, _)| {
            // Check for any right-angle intersections between the outside lines of the regions &
            // the lines between the red points. If there are any that means that some section of
            // this rectange will be outside & we can skip it.
            let xbounds = minmax(a.x, b.x);
            let ybounds = minmax(a.y, b.y);
            for line in &lines {
                match line {
                    Line::Horizontal(y, xrange) if ybounds.0 < *y && *y < ybounds.1 => {
                        if xrange.contains(&(xbounds.0 + 1)) || xrange.contains(&(xbounds.1 - 1)) {
                            return false;
                        }
                    }
                    Line::Vertical(x, yrange) if xbounds.0 < *x && *x < xbounds.1 => {
                        if yrange.contains(&(ybounds.0 + 1)) || yrange.contains(&(ybounds.1 - 1)) {
                            return false;
                        }
                    }
                    _ => {}
                }
            }
            true
        })
        .unwrap()
        .2
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 50, part2 = 24)]
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
