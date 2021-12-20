use aoc::{parse_number_list, runner::*};

type State = [i64; 9];

fn get_state(input: Vec<i32>) -> State {
    let mut state = [0; 9];
    for num in input {
        state[num as usize] += 1;
    }
    return state;
}

fn pass_day(state: State) -> State {
    return [
        state[1],
        state[2],
        state[3],
        state[4],
        state[5],
        state[6],
        state[7] + state[0],
        state[8],
        state[0],
    ];
}

fn pass_days(state: State, days: i64) -> State {
    let mut state = state;
    for _ in 0..days {
        state = pass_day(state);
    }
    return state;
}

fn part1(input: String) -> i64 {
    let mut state = get_state(parse_number_list(input, ","));
    state = pass_days(state, 80);
    return state.iter().sum();
}

fn part2(input: String) -> i64 {
    let mut state = get_state(parse_number_list(input, ","));
    state = pass_days(state, 256);
    return state.iter().sum();
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "3,4,3,1,2";

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

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 5934);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 26984457539);
    }
}
