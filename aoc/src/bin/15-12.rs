puzzle_lib::setup!(title = "JSAbacusFramework.io");

fn parse_input(input: &str) -> Vec<isize> {
    parse!(input => { [nums find matches /r"-?\d+"/ as isize] } => nums)
}

pub fn part1(input: &str) -> isize {
    let numbers = parse_input(input);
    numbers.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 6)]
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
}
