use std::collections::HashMap;

puzzle_lib::setup!(title = "Doesn't He Have Intern-Elves For This?");

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[inline]
fn is_nice_1(string: &str) -> bool {
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
    strings.into_iter().filter(|s| is_nice_1(s)).count()
}

#[inline]
fn is_nice_2(string: &str) -> bool {
    let mut prevprev = '_';
    let mut prev = '_';
    let mut hasrepeat = false;
    for c in string.chars() {
        if c == prevprev {
            hasrepeat = true;
            break;
        }
        prevprev = prev;
        prev = c;
    }
    if !hasrepeat {
        return false;
    }

    let mut pair_indexes: HashMap<(char, char), usize> = HashMap::new();
    for (idx, pair) in string.chars().tuple_windows().enumerate() {
        if let Some(prev_idx) = pair_indexes.get(&pair) {
            if idx - prev_idx > 1 {
                return true;
            }
        } else {
            pair_indexes.insert(pair, idx);
        }
    }
    false
}

pub fn part2(input: &str) -> usize {
    let strings = parse_input(input);
    strings.into_iter().filter(|s| is_nice_2(s)).count()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 2)]
    static EXAMPLE_INPUT_1: &str = "
        ugknbfddgicrmopn
        aaa
        jchzalrnumimnmhp
        haegwjzuvuyypxyu
        dvszwmarrgswjxmb
    ";

    #[example_input(part2 = 2)]
    static EXAMPLE_INPUT_2: &str = "
        qjhvhtzxzqqjkmpb
        xxyxx
        uurcxstgmygtbstg
        ieodomkazucvgmuy
    ";

    #[test]
    fn example_is_nice_1() {
        let strings = parse_input(&EXAMPLE_INPUT_1);
        assert_eq!(
            strings.into_iter().map(is_nice_1).collect::<Vec<_>>(),
            vec![true, true, false, false, false]
        );
    }

    #[test]
    fn example_is_nice_2() {
        let strings = parse_input(&EXAMPLE_INPUT_2);
        assert_eq!(
            strings.into_iter().map(is_nice_2).collect::<Vec<_>>(),
            vec![true, true, false, false]
        );
    }
}
