use aoc::runner::*;
use derive_new::new;

#[derive(Debug, PartialEq, new)]
struct TargetArea {
    pub x: (i16, i16),
    pub y: (i16, i16),
}
impl TargetArea {
    fn contains_x(&self, x: i16) -> bool {
        return self.x.0 <= x && x <= self.x.1;
    }

    fn contains_y(&self, y: i16) -> bool {
        return self.y.0 <= y && y <= self.y.1;
    }
}

fn parse_input(input: String) -> TargetArea {
    // Safety? Trim? Split? Parse? Bah, who needs that nonsense. This is faster and therefore _clearly_ superior.
    unsafe {
        let mut bytes = input.bytes();
        let mut nums = [0i16; 4];

        for i in 0..2 {
            while bytes.next().unwrap_unchecked() != b'=' {}
            for j in 0..2 {
                let mut b = bytes.next().unwrap_unchecked();
                let mut neg = false;
                if b == b'-' {
                    neg = true;
                    b = bytes.next().unwrap_unchecked();
                }
                let mut num = 0;
                while b >= b'0' {
                    num = num * 10 + b - b'0';
                    b = bytes.next().unwrap_unchecked();
                }
                bytes.next();
                nums[i * 2 + j] = if neg { -(num as i16) } else { num as i16 };
            }
        }

        return TargetArea::new((nums[0], nums[1]), (nums[2], nums[3]));
    }
}

fn ends_up_at_target(mut xvel: i16, mut yvel: i16, target: &TargetArea) -> bool {
    let mut x = 0;
    let mut y = 0;
    while x <= target.x.1 && y >= target.y.0 {
        x += xvel;
        y += yvel;
        xvel = (xvel - 1).max(0);
        yvel -= 1;

        if target.contains_x(x) && target.contains_y(y) {
            return true;
        }
    }
    return false;
}

pub fn part1(input: String) -> i16 {
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

pub fn part2(input: String) -> i16 {
    let target = parse_input(input);
    /*
     * Despite having instructions on how to handle negative X velocities these will never get us to our goal, so we need not consider them. The highest x velocity that could be suitable is one that would get us to the right edge in one step, which is target.x.1.
     *
     * The logic used in part 1 gives us the bounds for yvel for upwards trajectories (-target.y.0 - 1). For downward trajectories we are essentially only considering the last step, giving us a bound of (target.y.0).
     */
    let mut count = 0;
    for xvel in 0..=(target.x.1) {
        for yvel in (target.y.0)..(-target.y.0) {
            if ends_up_at_target(xvel, yvel, &target) {
                count += 1;
            }
        }
    }
    return count;
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

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

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 112);
    }
}
