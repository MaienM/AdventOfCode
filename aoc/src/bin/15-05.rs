puzzle_lib::setup!(title = "Doesn't He Have Intern-Elves For This?");

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[inline]
fn is_nice(string: &str) -> bool {
    let mut prev = '_';
    let mut has_double = false;
    let mut vowels = 0u8;
    for c in string.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,

            'b' if prev == 'a' => return false,
            'd' if prev == 'c' => return false,
            'q' if prev == 'p' => return false,
            'y' if prev == 'x' => return false,

            _ => {}
        }
        if c == prev {
            has_double = true;
        }
        prev = c;
    }
    has_double && vowels >= 3
}

pub fn part1(input: &str) -> usize {
    let strings = parse_input(input);
    strings.into_iter().filter(|s| is_nice(s)).count()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 2)]
    static EXAMPLE_INPUT: &str = "
        ugknbfddgicrmopn
        aaa
        jchzalrnumimnmhp
        haegwjzuvuyypxyu
        dvszwmarrgswjxmb
    ";

    #[test]
    fn example_is_nice() {
        let strings = parse_input(&EXAMPLE_INPUT);
        assert_eq!(
            strings.into_iter().map(|s| is_nice(s)).collect::<Vec<_>>(),
            vec![true, true, false, false, false]
        );
    }
}
