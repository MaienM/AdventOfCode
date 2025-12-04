puzzle_runner::register_chapter!(book = "2024", title = "Garden Groups");

use std::collections::HashSet;

use puzzle_lib::{
    grid::FullGrid,
    point::{Direction2X, Point2},
};

type Grid = FullGrid<char>;
type Region = HashSet<Point2>;

fn parse_input(input: &str) -> Grid {
    parse!(input => { [grid cells] } => grid)
}

fn find_region(grid: &Grid, members: &mut HashSet<Point2>, current: &Point2, chr: char) {
    for neighbour in current.neighbours_ortho() {
        if members.contains(&neighbour) {
            continue;
        }
        if let Some(c) = grid.get(&neighbour)
            && *c == chr
        {
            members.insert(neighbour);
            find_region(grid, members, &neighbour, chr);
        }
    }
}

fn find_regions(grid: &Grid) -> Vec<Region> {
    let bounds = Point2::new(grid.width(), grid.height());
    let mut visited = HashSet::new();
    let mut regions = Vec::new();
    for x in 0..bounds.x {
        for y in 0..bounds.y {
            let point = Point2::new(x, y);
            if !visited.contains(&point) {
                let mut members = HashSet::new();
                members.insert(point);
                find_region(grid, &mut members, &point, grid[point]);
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

fn has_neighbour(region: &Region, point: &Point2, direction: Direction2X) -> bool {
    point
        .checked_add_direction2x(direction)
        .is_some_and(|p| region.contains(&p))
}

fn price_bulk(region: &Region) -> usize {
    // We're just counting the corners here, because the amount of sides equals the amount of corners.
    let sides: usize = region
        .iter()
        .map(|member| {
            let n = has_neighbour(region, member, Direction2X::North);
            let e = has_neighbour(region, member, Direction2X::East);
            let s = has_neighbour(region, member, Direction2X::South);
            let w = has_neighbour(region, member, Direction2X::West);
            let ne = has_neighbour(region, member, Direction2X::NorthEast);
            let nw = has_neighbour(region, member, Direction2X::NorthWest);
            let se = has_neighbour(region, member, Direction2X::SouthEast);
            let sw = has_neighbour(region, member, Direction2X::SouthWest);
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
    let grid = parse_input(input);
    let regions = find_regions(&grid);
    regions.iter().map(price_normal).sum()
}

pub fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let regions = find_regions(&grid);
    regions.iter().map(price_bulk).sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

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
