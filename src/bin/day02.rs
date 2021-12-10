use aoc::*;

fn part1(input: String) -> String {
    let lines = input.trim().split("\n").map(|l| l.trim());
    let mut hpos = 0;
    let mut vpos = 0;
    for line in lines {
        let mut parts = line.splitn(2, " ");
        let direction = parts
            .next()
            .unwrap_or_else(|| panic!("Unable to get direction from line {}.", line));
        let distance = parts
            .next()
            .unwrap_or_else(|| panic!("Unable to get distance from line {}.", line))
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Unable to parse distance from line {}.", line));
        match direction {
            "forward" => hpos += distance,
            "down" => vpos += distance,
            "up" => vpos -= distance,
            _ => panic!("Invalid line {}.", line),
        }
    }
    return (hpos * vpos).to_string();
}

fn main() {
    run(part1, missing);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    ";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), "150");
    }
}
