use std::collections::HashSet;

use aoc::utils::{parse, point::Point2};

#[derive(Debug, Eq, PartialEq)]
struct Region {
    size: usize,
    perimeter: usize,
}
impl Region {
    fn price(&self) -> usize {
        self.size * self.perimeter
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    parse!(input => { [tiles split on '\n' with [chars]] } => tiles)
}

fn find_region(tiles: &[Vec<char>], members: &mut HashSet<Point2>, current: &Point2, chr: char) {
    for neighbour in current.neighbours_ortho() {
        if members.contains(&neighbour) {
            continue;
        }
        if let Some(c) = tiles.get(neighbour.y).and_then(|row| row.get(neighbour.x)) {
            if *c == chr {
                members.insert(neighbour);
                find_region(tiles, members, &neighbour, chr);
            }
        }
    }
}

fn find_regions(tiles: &[Vec<char>]) -> Vec<Region> {
    let bounds = Point2::new(tiles[0].len(), tiles.len());
    let mut visited = HashSet::new();
    let mut regions = Vec::new();
    for x in 0..bounds.x {
        for y in 0..bounds.y {
            let point = Point2::new(x, y);
            if !visited.contains(&point) {
                let mut members = HashSet::new();
                members.insert(point);
                let chr = tiles[point.y][point.x];
                find_region(tiles, &mut members, &point, chr);

                visited.extend(members.clone());

                let perimeter = members
                    .iter()
                    .map(|member| {
                        4 - member
                            .neighbours_ortho()
                            .into_iter()
                            .filter(|n| members.contains(n))
                            .count()
                    })
                    .sum();

                regions.push(Region {
                    size: members.len(),
                    perimeter,
                });
            }
        }
    }
    regions
}

pub fn part1(input: &str) -> usize {
    let tiles = parse_input(input);
    let regions = find_regions(&tiles);
    regions.into_iter().map(|r| r.price()).sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 140)]
    static EXAMPLE_INPUT_1: &str = "
        AAAA
        BBCD
        BBCC
        EEEC
    ";

    #[example_input(part1 = 772)]
    static EXAMPLE_INPUT_2: &str = "
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
    ";

    #[example_input(part1 = 1930)]
    static EXAMPLE_INPUT_3: &str = "
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    ";

    // #[test]
    // fn example_parse() {
    //     let actual = parse_input(&EXAMPLE_INPUT_1);
    //     let expected = vec![1, 2];
    //     assert_eq!(actual, expected);
    // }
}
