puzzle_runner::register_chapter!(book = 2015, title = "All in a Single Night");

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
};

fn parse_input(input: &str) -> Vec<(&str, &str, usize)> {
    parse!(input => {
        [edges split on '\n' with
            { left " to " right " = " [distance as usize] }
            => (left, right, distance)
        ]
    } => edges)
}

struct Graph {
    /// The available locations as bit flags.
    locations: Vec<usize>,
    /// The distances, with index (flag a + flag b) being the distance between those two locations.
    edges: Vec<usize>,
    /// The bit mask containing all locations.
    mask: usize,
}

/// Convert each of the locations to a bit flag so that we can track the visited locations using a
/// single number instead of a whole collection.
fn optimize_locations(edges: Vec<(&str, &str, usize)>) -> Graph {
    let locations: HashMap<_, _> = edges
        .iter()
        .flat_map(|(l, r, _)| [l, r])
        .unique()
        .enumerate()
        .map(|(i, name)| (*name, 1 << i))
        .collect();
    let mask = (1 << locations.len()) - 1;
    let mut new_edges: Vec<usize> = (0..=mask).map(|_| 0).collect();
    for (l, r, d) in edges {
        new_edges[locations[l] + locations[r]] = d;
    }
    Graph {
        edges: new_edges,
        locations: locations.into_values().collect(),
        mask,
    }
}

#[register_part]
fn part1(input: &str) -> usize {
    let edges = parse_input(input);
    let Graph {
        edges,
        locations,
        mask,
    } = optimize_locations(edges);

    let mut paths = locations
        .iter()
        .map(|l| (Reverse(0usize), *l, *l))
        .collect::<BinaryHeap<_>>();
    while !paths.is_empty() {
        let (Reverse(distance), visited, current) = paths.pop().unwrap();
        if visited == mask {
            return distance;
        }
        for next in &locations {
            if visited & next == 0 {
                paths.push((
                    Reverse(distance + edges[current + next]),
                    visited | next,
                    *next,
                ));
            }
        }
    }
    panic!("Should never happen");
}

#[register_part]
fn part2(input: &str) -> usize {
    let edges = parse_input(input);
    let Graph {
        edges,
        locations,
        mask,
    } = optimize_locations(edges);

    let mut paths = locations
        .iter()
        .map(|l| (0usize, *l, *l))
        .collect::<VecDeque<_>>();
    let mut max = 0;
    while let Some((distance, visited, current)) = paths.pop_front() {
        if visited == mask {
            max = usize::max(max, distance);
        }
        for next in &locations {
            if visited & next == 0 {
                paths.push_back((distance + edges[current + next], visited | next, *next));
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 605, part2 = 982)]
    static EXAMPLE_INPUT: &str = "
        London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            ("London", "Dublin", 464),
            ("London", "Belfast", 518),
            ("Dublin", "Belfast", 141),
        ];
        assert_eq!(actual, expected);
    }
}
