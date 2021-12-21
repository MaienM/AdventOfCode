use aoc::{parse_number_list, runner::*};

fn get_cost_linear(numbers: &Vec<i32>, target: i32) -> i32 {
    return numbers
        .iter()
        .map(|p| {
            return (target - p).abs();
        })
        .sum::<i32>()
        .into();
}

fn get_cost_exponential(numbers: &Vec<i32>, target: i32) -> i32 {
    return numbers
        .iter()
        .map(|p| {
            let steps = (target - p).abs();
            return (0..=steps).sum::<i32>();
        })
        .sum::<i32>()
        .into();
}

fn find_optimum(positions: &Vec<i32>, get_cost: fn(&Vec<i32>, i32) -> i32) -> i32 {
    // This function assumes a distribution where there is a steady increase in cost when moving away from the optimum result.

    // Start with a (sort of) binary search, to get close to the optimum result as quickly as possible.
    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();
    let size = max - min;
    let mut target = size / 2;
    for level in 1.. {
        let level_size = size / (2 as i32).pow(level);
        if level_size < 2 {
            break;
        }
        let new_targets = (target - level_size, target + level_size);
        let new_costs = (
            get_cost(&positions, new_targets.0),
            get_cost(&positions, new_targets.1),
        );
        if new_costs.0 > new_costs.1 {
            target = new_targets.1;
        } else {
            target = new_targets.0;
        }
    }

    // Target should now be close, but might be not quite there. Figure out if one of the directions is an improvement, and if so keep moving in that direction until results become worse.
    let mut cost = get_cost(&positions, target);
    let direction = if get_cost(&positions, target - 1) < cost {
        -1
    } else {
        1
    };

    loop {
        let new_cost = get_cost(&positions, target + direction);
        if new_cost > cost {
            return cost.into();
        }

        target += direction;
        cost = new_cost;
    }
}

pub fn part1(input: String) -> i32 {
    let positions = parse_number_list(input, ",");
    return find_optimum(&positions, get_cost_linear);
}

pub fn part2(input: String) -> i32 {
    let positions = parse_number_list(input, ",");
    return find_optimum(&positions, get_cost_exponential);
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 37);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 168);
    }
}
