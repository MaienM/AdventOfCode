puzzle_runner::register_chapter!(book = "2015", title = "Reindeer Olympics");

use std::iter::repeat_n;

use num::Integer;

#[derive(Debug, Eq, PartialEq)]
struct Reindeer<'a> {
    name: &'a str,
    speed: u16,
    time: u16,
    rest: u16,
}

fn parse_input(input: &str) -> Vec<Reindeer<'_>> {
    parse!(input => {
        [reindeer split on '\n' with
            {
                name
                " can fly "
                [speed as u16]
                " km/s for "
                [time as u16]
                " seconds, but then must rest for "
                [rest as u16]
                " seconds."
            }
            => Reindeer { name, speed, time, rest }
        ]
    } => reindeer)
}

fn part1impl(reindeer: Vec<Reindeer>, elapsed: u16) -> u16 {
    reindeer
        .into_iter()
        .map(|r| {
            let (cycles, remainder) = elapsed.div_rem(&(r.time + r.rest));
            (cycles * r.time + u16::min(r.time, remainder)) * r.speed
        })
        .max()
        .unwrap()
}

#[register_part]
fn part1(input: &str) -> u16 {
    let reindeer = parse_input(input);
    part1impl(reindeer, 2503)
}

fn part2impl(reindeer: Vec<Reindeer>, elapsed: u16) -> u16 {
    let mut cycles: Vec<_> = reindeer
        .into_iter()
        .map(|r| {
            repeat_n(r.speed, r.time as usize)
                .chain(repeat_n(0, r.rest as usize))
                .cycle()
        })
        .collect();
    let mut distances: Vec<_> = cycles.iter().map(|_| 0).collect();
    let mut points: Vec<_> = cycles.iter().map(|_| 0).collect();
    for _ in 0..elapsed {
        let mut max = 0;
        for (i, step) in cycles
            .iter_mut()
            .map(Iterator::next)
            .map(Option::unwrap)
            .enumerate()
        {
            let distance = &mut distances[i];
            *distance += step;
            if *distance > max {
                max = *distance;
            }
        }

        for (i, distance) in distances.iter().enumerate() {
            if *distance == max {
                points[i] += 1;
            }
        }
    }
    points.into_iter().max().unwrap()
}

#[register_part]
fn part2(input: &str) -> u16 {
    let reindeer = parse_input(input);
    part2impl(reindeer, 2503)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input]
    static EXAMPLE_INPUT: &str = "
        Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            Reindeer {
                name: "Comet",
                speed: 14,
                time: 10,
                rest: 127,
            },
            Reindeer {
                name: "Dancer",
                speed: 16,
                time: 11,
                rest: 162,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        let reindeer = parse_input(&EXAMPLE_INPUT);
        assert_eq!(part1impl(reindeer, 1000), 1120);
    }

    #[test]
    fn example_part2() {
        let reindeer = parse_input(&EXAMPLE_INPUT);
        assert_eq!(part2impl(reindeer, 1000), 689);
    }
}
