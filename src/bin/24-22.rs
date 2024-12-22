use std::collections::HashMap;

use aoc::utils::parse;
use rayon::prelude::*;

const PRUNE: usize = 16_777_216;

fn parse_input(input: &str) -> Vec<usize> {
    parse!(input => {
        [num split on '\n' as usize]
    } => num)
}

fn next_num(mut num: usize) -> usize {
    let n2 = num * 64;
    num ^= n2;
    num %= PRUNE;
    let n2 = num / 32;
    num ^= n2;
    num %= PRUNE;
    let n2 = num * 2048;
    num ^= n2;
    num %= PRUNE;
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
    let prices: Vec<_> = nums
        .into_par_iter()
        .map(|mut num| {
            let mut prices = Vec::with_capacity(2001);
            prices.push((num % 10) as isize);
            for _ in 0..2000 {
                num = next_num(num);
                prices.push((num % 10) as isize);
            }
            prices
        })
        .collect();
    let price_by_deltas: Vec<_> = prices
        .into_par_iter()
        .map(|prices| {
            let deltas: Vec<_> = prices
                .iter()
                .zip(prices.iter().skip(1))
                .map(|(n1, n2)| n2 - n1)
                .collect();
            let mut price_by_delta = HashMap::new();
            for i in 0..(deltas.len() - 3) {
                let delta = Vec::from(&deltas[i..i + 4]);
                price_by_delta.entry(delta).or_insert(prices[i + 4]);
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
    sum_by_delta.into_values().max().unwrap() as usize
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

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
    fn example_nest() {
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
