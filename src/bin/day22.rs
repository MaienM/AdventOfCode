use std::fmt::Debug;
use std::ops::RangeInclusive;

use aoc::runner::*;

fn ranges_overlap(lhs: &RangeInclusive<i64>, rhs: &RangeInclusive<i64>) -> bool {
    return lhs.contains(rhs.start())
        || lhs.contains(rhs.end())
        || rhs.contains(lhs.start())
        || rhs.contains(lhs.end());
}

#[derive(Eq, PartialEq)]
struct Region(
    RangeInclusive<i64>,
    RangeInclusive<i64>,
    RangeInclusive<i64>,
);
impl Region {
    fn size(&self) -> u64 {
        return (self.0.end() - self.0.start() + 1) as u64
            * (self.1.end() - self.1.start() + 1) as u64
            * (self.2.end() - self.2.start() + 1) as u64;
    }

    fn overlaps(&self, other: &Region) -> bool {
        return ranges_overlap(&self.0, &other.0)
            && ranges_overlap(&self.1, &other.1)
            && ranges_overlap(&self.2, &other.2);
    }

    fn without(&self, other: &Region) -> Option<Vec<Region>> {
        if !self.overlaps(other) {
            return None;
        }

        let mut parts = Vec::new();

        if self.0.contains(&(other.0.start() - 1)) {
            // Left
            parts.push(Region(
                (*self.0.start())..=(other.0.start() - 1),
                self.1.clone(),
                self.2.clone(),
            ));
        }
        if self.0.contains(&(other.0.end() + 1)) {
            // Right
            parts.push(Region(
                (other.0.end() + 1)..=(*self.0.end()),
                self.1.clone(),
                self.2.clone(),
            ));
        }
        let range0 = (*self.0.start().max(other.0.start()))..=(*self.0.end().min(other.0.end()));

        if self.1.contains(&(other.1.start() - 1)) {
            // Above
            parts.push(Region(
                range0.clone(),
                (*self.1.start())..=(other.1.start() - 1),
                self.2.clone(),
            ));
        }
        if self.1.contains(&(other.1.end() + 1)) {
            // Below
            parts.push(Region(
                range0.clone(),
                (other.1.end() + 1)..=(*self.1.end()),
                self.2.clone(),
            ));
        }
        let range1 = (*self.1.start().max(other.1.start()))..=(*self.1.end().min(other.1.end()));

        if self.2.contains(&(other.2.start() - 1)) {
            // In front of
            parts.push(Region(
                range0.clone(),
                range1.clone(),
                (*self.2.start())..=(other.2.start() - 1),
            ));
        }
        if self.2.contains(&(other.2.end() + 1)) {
            // Behind
            parts.push(Region(
                range0.clone(),
                range1.clone(),
                (other.2.end() + 1)..=(*self.2.end()),
            ));
        }

        return Some(parts);
    }
}
impl Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Region")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .field(&self.size())
            .finish()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Step(Region, bool);

struct RegionalManager(Vec<Region>);
impl RegionalManager {
    fn turn_off(&mut self, region: &Region) {
        loop {
            match self.0.iter().position(|r| r.overlaps(region)) {
                Some(pos) => {
                    let overlapping = self.0.swap_remove(pos);
                    self.0.append(&mut overlapping.without(region).unwrap());
                }
                None => break,
            }
        }
    }

    fn turn_on(&mut self, region: Region) {
        self.turn_off(&region);
        self.0.push(region);
    }

    fn count_enabled(&self) -> u64 {
        return self.0.iter().map(Region::size).sum();
    }
}

fn parse_input(input: String) -> Vec<Step> {
    return input
        .trim()
        .split("\n")
        .map(str::trim)
        .map(|line| {
            let mut parts = line.splitn(2, " ");
            let status = match parts.next() {
                Some("on") => true,
                Some("off") => false,
                _ => panic!("Bad input, no cookie."),
            };
            let mut ranges = parts.next().unwrap().splitn(3, ",").map(|range| {
                let mut parts = range[2..].splitn(2, "..").map(|p| p.parse().unwrap());
                return (parts.next().unwrap())..=(parts.next().unwrap());
            });
            return Step(
                Region(
                    ranges.next().unwrap(),
                    ranges.next().unwrap(),
                    ranges.next().unwrap(),
                ),
                status,
            );
        })
        .collect();
}

pub fn part1(input: String) -> u64 {
    let steps = parse_input(input);
    let mut manager = RegionalManager(Vec::new());

    let relevant_region = Region(-50..=50, -50..=50, -50..=50);
    for Step(region, status) in steps {
        if !relevant_region.overlaps(&region) {
            continue;
        }

        if status {
            manager.turn_on(region);
        } else {
            manager.turn_off(&region);
        }
    }

    return manager.count_enabled();
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        on x=-20..26,y=-36..17,z=-47..7
        on x=-20..33,y=-21..23,z=-26..28
        on x=-22..28,y=-29..23,z=-38..16
        on x=-46..7,y=-6..46,z=-50..-1
        on x=-49..1,y=-3..46,z=-24..28
        on x=2..47,y=-22..22,z=-23..27
        on x=-27..23,y=-28..26,z=-21..29
        on x=-39..5,y=-6..47,z=-3..44
        on x=-30..21,y=-8..43,z=-13..34
        on x=-22..26,y=-27..20,z=-29..19
        off x=-48..-32,y=26..41,z=-47..-37
        on x=-12..35,y=6..50,z=-50..-2
        off x=-48..-32,y=-32..-16,z=-15..-5
        on x=-18..26,y=-33..15,z=-7..46
        off x=-40..-22,y=-38..-28,z=23..41
        on x=-16..35,y=-41..10,z=-47..6
        off x=-32..-23,y=11..30,z=-14..3
        on x=-49..-5,y=-3..45,z=-29..18
        off x=18..30,y=-20..-8,z=-3..13
        on x=-41..9,y=-7..43,z=-33..15
        on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
        on x=967..23432,y=45373..81175,z=27513..53682
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = vec![
            Step(Region(-20..=26, -36..=17, -47..=7), true),
            Step(Region(-20..=33, -21..=23, -26..=28), true),
            Step(Region(-22..=28, -29..=23, -38..=16), true),
            Step(Region(-46..=7, -6..=46, -50..=-1), true),
            Step(Region(-49..=1, -3..=46, -24..=28), true),
            Step(Region(2..=47, -22..=22, -23..=27), true),
            Step(Region(-27..=23, -28..=26, -21..=29), true),
            Step(Region(-39..=5, -6..=47, -3..=44), true),
            Step(Region(-30..=21, -8..=43, -13..=34), true),
            Step(Region(-22..=26, -27..=20, -29..=19), true),
            Step(Region(-48..=-32, 26..=41, -47..=-37), false),
            Step(Region(-12..=35, 6..=50, -50..=-2), true),
            Step(Region(-48..=-32, -32..=-16, -15..=-5), false),
            Step(Region(-18..=26, -33..=15, -7..=46), true),
            Step(Region(-40..=-22, -38..=-28, 23..=41), false),
            Step(Region(-16..=35, -41..=10, -47..=6), true),
            Step(Region(-32..=-23, 11..=30, -14..=3), false),
            Step(Region(-49..=-5, -3..=45, -29..=18), true),
            Step(Region(18..=30, -20..=-8, -3..=13), false),
            Step(Region(-41..=9, -7..=43, -33..=15), true),
            Step(
                Region(-54112..=-39298, -85059..=-49293, -27449..=7877),
                true,
            ),
            Step(Region(967..=23432, 45373..=81175, 27513..=53682), true),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_region_size() {
        let region = Region(0..=9, 0..=9, 0..=9);
        assert_eq!(region.size(), 1000);
    }

    #[test]
    fn test_region_without() {
        let region = Region(0..=10, 0..=10, 0..=10);

        let cutout = Region(5..=10, 2..=8, -5..=5);
        let without = region.without(&cutout).unwrap();

        assert_eq!(
            without,
            vec![
                Region(0..=4, 0..=10, 0..=10),
                Region(5..=10, 0..=1, 0..=10),
                Region(5..=10, 9..=10, 0..=10),
                Region(5..=10, 2..=8, 6..=10),
            ],
        );
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 590784);
    }
}
