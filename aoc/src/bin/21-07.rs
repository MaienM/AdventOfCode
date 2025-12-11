puzzle_runner::register_chapter!(book = "2021", title = "The Treachery of Whales");

fn parse_input(input: &str) -> Vec<i32> {
    parse!(input => {
        [nums split on ',' as i32]
    } => nums)
}

fn get_cost_linear(numbers: &[i32], target: i32) -> i32 {
    numbers.iter().map(|p| (target - p).abs()).sum()
}

fn get_cost_exponential(numbers: &[i32], target: i32) -> i32 {
    numbers
        .iter()
        .map(|p| {
            let steps = (target - p).abs();
            (0..=steps).sum::<i32>()
        })
        .sum()
}

fn find_optimum(positions: &[i32], get_cost: fn(&[i32], i32) -> i32) -> i32 {
    // This function assumes a distribution where there is a steady increase in cost when moving away from the optimum result.

    // Start with a (sort of) binary search, to get close to the optimum result as quickly as possible.
    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();
    let size = max - min;
    let mut target = size / 2;
    for level in 1.. {
        let level_size = size / 2_i32.pow(level);
        if level_size < 2 {
            break;
        }
        let new_targets = (target - level_size, target + level_size);
        let new_costs = (
            get_cost(positions, new_targets.0),
            get_cost(positions, new_targets.1),
        );
        if new_costs.0 > new_costs.1 {
            target = new_targets.1;
        } else {
            target = new_targets.0;
        }
    }

    // Target should now be close, but might be not quite there. Figure out if one of the directions is an improvement, and if so keep moving in that direction until results become worse.
    let mut cost = get_cost(positions, target);
    let direction: i32 = if get_cost(positions, target - 1) < cost {
        -1
    } else {
        1
    };

    loop {
        let new_target = target + direction;
        let new_cost = get_cost(positions, new_target);
        if new_cost > cost {
            return cost;
        }
        target = new_target;
        cost = new_cost;
    }
}

#[register_part]
fn part1(input: &str) -> i32 {
    let positions = parse_input(input);
    find_optimum(&positions, get_cost_linear)
}

#[register_part]
fn part2(input: &str) -> i32 {
    let positions = parse_input(input);
    find_optimum(&positions, get_cost_exponential)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 37, part2 = 168)]
    static EXAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";
}
