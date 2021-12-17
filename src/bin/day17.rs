use aoc::runner::*;

#[macro_use]
extern crate derive_new;

#[derive(Debug, PartialEq, new)]
struct TargetArea {
    pub x: (i32, i32),
    pub y: (i32, i32),
}
impl TargetArea {
    fn contains_x(&self, x: i32) -> bool {
        return self.x.0 <= x && x <= self.x.1;
    }

    fn contains_y(&self, y: i32) -> bool {
        return self.y.0 <= y && y <= self.y.1;
    }
}

fn parse_input(input: String) -> TargetArea {
    assert!(input.trim().starts_with("target area: "));
    let mut ranges = input.trim()[13..].splitn(2, ",").map(|p| {
        let bounds: [i32; 2] = p.trim()[2..]
            .splitn(2, "..")
            .map(str::parse)
            .map(Result::unwrap)
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();
        return (bounds[0], bounds[1]);
    });
    return TargetArea::new(ranges.next().unwrap(), ranges.next().unwrap());
}

fn part1(input: String) -> i32 {
    let target = parse_input(input);
    /*
     * X and Y are completely independent, so we can just ignore X for this part.
     *
     * The path is a parabolic curve; so if we start at yvel A we'll arrive back at (x, 0) with yvel -A at some point.
     * The higher the velociy is at this point the higher the peak has been. The ideal would be if it ends up at the bottom edge of the target area in one step, meaning it moves from y=0 to y=target.y.1 in one step.
     * To accomplish this yvel=target.y.0, which means that one step earlier (the step that got it to y=0) yvel=target.y.0. This gives us an optimal starting velocity of -target.y.0 - 1.
     */
    let mut y = 0;
    let mut yvel = -target.y.0 - 1;
    while yvel > 0 {
        y += yvel;
        yvel -= 1;
    }
    return y;
}

fn main() {
    run(part1, missing::<i8>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE_INPUT: &'static str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn example_parse() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT.to_string()),
            TargetArea::new((20, 30), (-10, -5))
        );
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 45);
    }
}
