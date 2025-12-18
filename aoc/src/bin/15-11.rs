puzzle_runner::register_chapter!(title = "Corporate Policy");

type Password = Vec<char>;

#[inline]
fn next_char(chr: char) -> char {
    (chr as u8 + 1) as char
}

#[inline]
fn next_valid_char(chr: char) -> char {
    match chr {
        // skip over forbidden invalid characters
        'h' => 'j',
        'k' => 'm',
        'n' => 'p',

        'z' => '_',
        _ => next_char(chr),
    }
}

#[inline]
fn increment(password: &mut Password) {
    let mut idx = password.len() - 1;
    while password[idx] == 'z' {
        password[idx] = 'a';
        idx -= 1;
    }
    password[idx] = next_valid_char(password[idx]);
}

#[inline]
fn is_valid(password: &Password) -> bool {
    // We eliminate the invalid characters before getting to this function & skip over them when
    // incrementing, so we don't need to check for them here.
    let mut chain = false;
    let mut pairs = 0;
    if password[0] == password[1] {
        pairs += 1;
    }
    for (a, b, c) in password.iter().tuple_windows() {
        if (a != b) & (b == c) {
            pairs += 1;
        }
        chain |= (next_char(*a) == *b) && (next_char(*b) == *c);
    }
    chain && pairs >= 2
}

fn next_valid_password(input: &str, times: u8) -> String {
    let mut password: Vec<_> = input.chars().collect();

    // If there are currently forbidden characters we can increment those & set everything after
    // that to 'a' as that is the first cycle that's worth checking.
    if let Some((idx, chr)) = password
        .iter()
        .find_position(|c| **c == 'i' || **c == 'o' || **c == 'l')
    {
        password[idx] = next_valid_char(*chr);
        for chr in password.iter_mut().skip(idx + 1) {
            *chr = 'a';
        }
    }

    for _ in 0..times {
        increment(&mut password);
        while !is_valid(&password) {
            increment(&mut password);
        }
    }

    password.iter().join("")
}

#[register_part]
fn part1(input: &str) -> String {
    next_valid_password(input, 1)
}

#[register_part]
fn part2(input: &str) -> String {
    next_valid_password(input, 2)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = "abcdffaa")]
    static EXAMPLE_INPUT_1: &str = "abcdefgh";

    #[example_input(part1 = "ghjaabcc")]
    static EXAMPLE_INPUT_2: &str = "ghijklmn";
}
