puzzle_runner::register_chapter!(book = "2025", title = "Playground");

use std::{cmp::Ordering, collections::HashSet};

use common_macros::hash_set;
use num::integer::sqrt;
use puzzle_lib::point::Point3;

fn parse_input(input: &str) -> Vec<Point3> {
    parse!(input => {
        [boxes split on '\n' with
            {
                [x as usize]
                ','
                [y as usize]
                ','
                [z as usize]
            }
            => Point3::new(x, y, z)
        ]
    } => boxes)
}

fn distance(a: &Point3, b: &Point3) -> usize {
    sqrt(a.x.abs_diff(b.x).pow(2) + a.y.abs_diff(b.y).pow(2) + a.z.abs_diff(b.z).pow(2))
}

fn add_connection(circuits: &mut Vec<HashSet<Point3>>, a: &Point3, b: &Point3) {
    let circuit_a = circuits.iter().find_position(|c| c.contains(a));
    let circuit_b = circuits.iter().find_position(|c| c.contains(b));
    match (circuit_a, circuit_b) {
        (None, None) => {
            circuits.push(hash_set![*a, *b]);
        }
        (None, Some((idx, _))) => {
            circuits[idx].insert(*a);
        }
        (Some((idx, _)), None) => {
            circuits[idx].insert(*b);
        }
        (Some((ia, _)), Some((ib, _))) => match ia.cmp(&ib) {
            Ordering::Less => {
                let to_merge = circuits.swap_remove(ib);
                circuits[ia].extend(to_merge);
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                let to_merge = circuits.swap_remove(ia);
                circuits[ib].extend(to_merge);
            }
        },
    }
}

#[register_part(arg = 1000)]
fn part1(input: &str, connections: usize) -> usize {
    let boxes = parse_input(input);
    let pairs = boxes
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (distance(a, b), a, b))
        .sorted_unstable()
        .take(connections);
    let mut circuits: Vec<HashSet<Point3>> = Vec::new();
    for (_, a, b) in pairs {
        add_connection(&mut circuits, a, b);
    }
    circuits
        .into_iter()
        .map(|c| c.len())
        .sorted_unstable()
        .rev()
        .take(3)
        .reduce(|a, b| a * b)
        .unwrap()
}

#[register_part]
fn part2(input: &str) -> usize {
    let boxes = parse_input(input);
    let all = boxes.len();
    let pairs = boxes
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (distance(a, b), a, b))
        .sorted_unstable();
    let mut circuits: Vec<HashSet<Point3>> = Vec::new();
    for (_, a, b) in pairs {
        add_connection(&mut circuits, a, b);
        if circuits.len() == 1 && circuits.first().unwrap().len() == all {
            return a.x * b.x;
        }
    }
    panic!("Should never happen.");
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 40, part1::arg = 10, part2 = 25_272)]
    static EXAMPLE_INPUT: &str = "
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
    ";
}
