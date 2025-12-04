puzzle_runner::register_chapter!(book = "2015", title = "I Was Told There Would Be No Math");

fn parse_input(input: &str) -> Vec<[u16; 3]> {
    parse!(input => {
        [boxes split on '\n' with
            { [l as u16] 'x' [w as u16] 'x' [h as u16] }
            => [l, w, h]
        ]
    } => boxes)
}

pub fn part1(input: &str) -> u32 {
    let boxes = parse_input(input);
    boxes
        .into_iter()
        .map(|dim| {
            let sides = [dim[0] * dim[1], dim[1] * dim[2], dim[0] * dim[2]];
            (2 * sides.iter().sum::<u16>() + sides.iter().min().unwrap()) as u32
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let boxes = parse_input(input);
    boxes
        .into_iter()
        .map(|mut dim| {
            dim.sort_unstable();
            (2 * (dim[0] + dim[1]) + dim[0] * dim[1] * dim[2]) as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 58, part2 = 34)]
    static EXAMPLE_INPUT_1: &str = "2x3x4";

    #[example_input(part1 = 43, part2 = 14)]
    static EXAMPLE_INPUT_2: &str = "1x1x10";

    #[test]
    fn example_parse_1() {
        let actual = parse_input(&EXAMPLE_INPUT_1);
        let expected = vec![[2, 3, 4]];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_parse_2() {
        let actual = parse_input(&EXAMPLE_INPUT_2);
        let expected = vec![[1, 1, 10]];
        assert_eq!(actual, expected);
    }
}
