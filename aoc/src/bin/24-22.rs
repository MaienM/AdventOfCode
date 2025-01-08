puzzle_lib::setup!(title = "Monkey Market");

use std::collections::HashMap;

const PRUNE: usize = 16_777_216;

fn parse_input(input: &str) -> Vec<usize> {
    parse!(input => {
        [num split on '\n' as usize]
    } => num)
}

fn next_num(mut num: usize) -> usize {
    num = (num ^ (num << 6)) % PRUNE;
    num = (num ^ (num >> 5)) % PRUNE;
    num = (num ^ (num << 11)) % PRUNE;
    num
}

pub fn part1(input: &str) -> usize {
    let nums = parse_input(input);
    nums.into_par_iter()
        .map(|mut num| {
            for _ in 0..2000 {
                num = next_num(num);
            }
            num
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let nums = parse_input(input);
    let price_by_deltas: Vec<_> = nums
        .into_par_iter()
        .map(|mut num| {
            let mut prices = Vec::with_capacity(2001);
            let mut deltas = Vec::with_capacity(2000);
            let mut price = (num % 10) as i32;
            prices.push(price);
            for _ in 0..2000 {
                num = next_num(num);
                let next_price = (num % 10) as i32;
                deltas.push(next_price - price);
                prices.push(next_price);
                price = next_price;
            }

            let mut price_by_delta = HashMap::new();
            for i in 0..(deltas.len() - 3) {
                let delta =
                    deltas[i] + (deltas[i + 1] << 4) + (deltas[i + 2] << 8) + (deltas[i + 3] << 12);
                price_by_delta
                    .entry(delta)
                    .or_insert(prices[i + 4] as usize);
            }
            price_by_delta
        })
        .collect();
    let mut sum_by_delta = HashMap::new();
    for price_by_delta in price_by_deltas {
        for (delta, price) in price_by_delta {
            sum_by_delta
                .entry(delta)
                .and_modify(|v| *v += price)
                .or_insert(price);
        }
    }
    sum_by_delta.into_values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 37_327_623)]
    static EXAMPLE_INPUT_1: &str = "
        1
        10
        100
        2024
    ";

    #[example_input(part2 = 23)]
    static EXAMPLE_INPUT_2: &str = "
        1
        2
        3
        2024
    ";

    #[test]
    fn example_next_num() {
        let pairs = [
            (123, 15_887_950),
            (15_887_950, 16_495_136),
            (16_495_136, 527_345),
            (527_345, 704_524),
            (704_524, 1_553_684),
            (1_553_684, 12_683_156),
            (12_683_156, 11_100_544),
            (11_100_544, 12_249_484),
            (12_249_484, 7_753_432),
            (7_753_432, 5_908_254),
        ];
        for (start, expected) in pairs {
            let actual = next_num(start);
            assert_eq!(actual, expected);
        }
    }
}
