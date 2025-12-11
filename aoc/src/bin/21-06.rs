puzzle_runner::register_chapter!(book = 2021, title = "Lanternfish");

type State = [u64; 9];

fn parse_input(input: &str) -> State {
    parse!(input => {
        [nums split on ',' as usize]
    } => get_state(nums))
}

fn get_state(input: Vec<usize>) -> State {
    let mut state = [0; 9];
    for num in input {
        state[num] += 1;
    }
    state
}

fn pass_day(state: State) -> State {
    [
        state[1],
        state[2],
        state[3],
        state[4],
        state[5],
        state[6],
        state[7] + state[0],
        state[8],
        state[0],
    ]
}

fn pass_days(state: State, days: u64) -> State {
    let mut state = state;
    for _ in 0..days {
        state = pass_day(state);
    }
    state
}

#[register_part]
fn part1(input: &str) -> u64 {
    let mut state = parse_input(input);
    state = pass_days(state, 80);
    state.iter().sum()
}

#[register_part]
fn part2(input: &str) -> u64 {
    let mut state = parse_input(input);
    state = pass_days(state, 256);
    state.iter().sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 5934, part2 = 26_984_457_539)]
    static EXAMPLE_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn example_pass_days() {
        assert_eq!(
            pass_days([0, 1, 2, 1, 0, 0, 0, 0, 0], 1),
            [1, 2, 1, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            pass_days([0, 1, 2, 1, 0, 0, 0, 0, 0], 2),
            [2, 1, 0, 0, 0, 0, 1, 0, 1]
        );
        assert_eq!(
            pass_days([0, 1, 2, 1, 0, 0, 0, 0, 0], 3),
            [1, 0, 0, 0, 0, 1, 2, 1, 2]
        );
    }
}
