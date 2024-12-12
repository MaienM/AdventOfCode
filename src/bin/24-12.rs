use std::collections::HashSet;

use aoc::utils::{
    parse,
    point::{Direction2, Point2},
};

type Region = HashSet<Point2>;

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
                find_region(tiles, &mut members, &point, tiles[point.y][point.x]);
                visited.extend(members.clone());
                regions.push(members);
            }
        }
    }
    regions
}

fn price_normal(region: &Region) -> usize {
    let perimeter: usize = region
        .iter()
        .map(|member| {
            4 - member
                .neighbours_ortho()
                .into_iter()
                .filter(|n| region.contains(n))
                .count()
        })
        .sum();
    perimeter * region.len()
}

fn has_neighbour_ortho(region: &Region, point: &Point2, direction: Direction2) -> bool {
    point
        .checked_add_direction(direction, &1)
        .is_some_and(|p| region.contains(&p))
}

fn has_neighbour_diag(region: &Region, point: &Point2, direction: [Direction2; 2]) -> bool {
    point
        .checked_add_direction(direction[0], &1)
        .and_then(|p| p.checked_add_direction(direction[1], &1))
        .is_some_and(|p| region.contains(&p))
}

fn price_bulk(region: &Region) -> usize {
    // We're just counting the corners here, because the amount of sides equals the amount of corners.
    let sides: usize = region
        .iter()
        .map(|member| {
            let n = has_neighbour_ortho(region, member, Direction2::North);
            let e = has_neighbour_ortho(region, member, Direction2::East);
            let s = has_neighbour_ortho(region, member, Direction2::South);
            let w = has_neighbour_ortho(region, member, Direction2::West);
            let ne = has_neighbour_diag(region, member, [Direction2::North, Direction2::East]);
            let nw = has_neighbour_diag(region, member, [Direction2::North, Direction2::West]);
            let se = has_neighbour_diag(region, member, [Direction2::South, Direction2::East]);
            let sw = has_neighbour_diag(region, member, [Direction2::South, Direction2::West]);
            // Outside corner.
            usize::from(!n && !e)
                + usize::from(!e && !s)
                + usize::from(!s && !w)
                + usize::from(!w && !n)
                // Inside corner.
                + usize::from(n && e && !ne)
                + usize::from(n && w && !nw)
                + usize::from(s && e && !se)
                + usize::from(s && w && !sw)
        })
        .sum();
    sides * region.len()
}

pub fn part1(input: &str) -> usize {
    let tiles = parse_input(input);
    let regions = find_regions(&tiles);
    regions.iter().map(price_normal).sum()
}

pub fn part2(input: &str) -> usize {
    let tiles = parse_input(input);
    let regions = find_regions(&tiles);
    regions.iter().map(price_bulk).sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 140, part2 = 80)]
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

    #[example_input(part2 = 236)]
    static EXAMPLE_INPUT_4: &str = "
        EEEEE
        EXXXX
        EEEEE
        EXXXX
        EEEEE
    ";

    #[example_input(part2 = 368)]
    static EXAMPLE_INPUT_5: &str = "
        AAAAAA
        AAABBA
        AAABBA
        ABBAAA
        ABBAAA
        AAAAAA
    ";

    // #[test]
    // fn example_parse() {
    //     let actual = parse_input(&EXAMPLE_INPUT_1);
    //     let expected = vec![1, 2];
    //     assert_eq!(actual, expected);
    // }
}
