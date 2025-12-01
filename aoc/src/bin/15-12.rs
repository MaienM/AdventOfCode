puzzle_lib::setup!(title = "JSAbacusFramework.io");

use std::iter::Peekable;

#[inline]
fn read_array<I: Iterator<Item = char>>(chars: &mut Peekable<I>, ignore_red: bool) -> isize {
    if chars.peek() == Some(&']') {
        return 0;
    }

    let mut sum = 0;
    loop {
        sum += read_item(chars, ignore_red).unwrap_or(0);
        match chars.next() {
            Some(']') => break,
            Some(',') => {}
            c => panic!("Unexpected {c:?}"),
        }
    }
    sum
}

#[inline]
fn read_object<I: Iterator<Item = char>>(chars: &mut Peekable<I>, ignore_red: bool) -> isize {
    if chars.peek() == Some(&'}') {
        return 0;
    }

    let mut sum = 0;
    let mut ignore = false;
    loop {
        let _ = read_item(chars, ignore_red);
        assert!(chars.next() == Some(':'));
        match read_item(chars, ignore_red) {
            Ok(value) => {
                sum += value;
            }
            Err(()) => {
                ignore = true;
            }
        }
        match chars.next() {
            Some('}') => break,
            Some(',') => {}
            c => panic!("Unexpected {c:?}"),
        }
    }
    if ignore { 0 } else { sum }
}

#[inline]
fn read_item<I: Iterator<Item = char>>(
    chars: &mut Peekable<I>,
    ignore_red: bool,
) -> Result<isize, ()> {
    match chars.next() {
        Some('"') => {
            let mut contents = chars.take_while(|v| *v != '"');
            if contents.join("") == "red" && ignore_red {
                Err(())
            } else {
                Ok(0)
            }
        }
        Some('[') => Ok(read_array(chars, ignore_red)),
        Some('{') => Ok(read_object(chars, ignore_red)),
        Some(start) => {
            let rest = chars
                .peeking_take_while(|c: &char| c.is_ascii_digit())
                .join("");
            Ok(format!("{start}{rest}").parse().unwrap())
        }
        c => panic!("Unexpected {c:?}"),
    }
}

pub fn part1(input: &str) -> isize {
    read_item(&mut input.chars().peekable(), false).unwrap()
}

pub fn part2(input: &str) -> isize {
    read_item(&mut input.chars().peekable(), true).unwrap()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 6, part2 = 6)]
    static EXAMPLE_INPUT_1: &str = "[1,2,3]";

    #[example_input(part1 = 6)]
    static EXAMPLE_INPUT_2: &str = r#"{"a":2,"b":4}"#;

    #[example_input(part1 = 3)]
    static EXAMPLE_INPUT_3: &str = "[[[3]]]";

    #[example_input(part1 = 3)]
    static EXAMPLE_INPUT_4: &str = r#"{"a":{"b":4},"c":-1}"#;

    #[example_input(part1 = 0)]
    static EXAMPLE_INPUT_5: &str = r#"{"a":[-1,1]}"#;

    #[example_input(part1 = 0)]
    static EXAMPLE_INPUT_6: &str = r#"[-1,{"a":1}]"#;

    #[example_input(part1 = 0)]
    static EXAMPLE_INPUT_7: &str = "[]";

    #[example_input(part1 = 0)]
    static EXAMPLE_INPUT_8: &str = "{}";

    #[example_input(part1 = 6, part2 = 4)]
    static EXAMPLE_INPUT_9: &str = r#"[1,{"c":"red","b":2},3]"#;

    #[example_input(part1 = 15, part2 = 0)]
    static EXAMPLE_INPUT_10: &str = r#"{"d":"red","e":[1,2,3,4],"f":5}"#;

    #[example_input(part1 = 6, part2 = 6)]
    static EXAMPLE_INPUT_11: &str = r#"[1,"red",5]"#;
}
