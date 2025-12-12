puzzle_runner::register_chapter!(book = 2025, title = "Christmas Tree Farm");

type Shape = [[bool; 3]; 3];

#[derive(Debug, Eq, PartialEq)]
struct Region {
    width: usize,
    height: usize,
    shapes: Vec<usize>,
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let (shapes, regions) = input.rsplit_once("\n\n").unwrap();
    parse!(shapes =>
        [shapes split on "\n\n" with
            {
                _
                ":\n"
                [cells split on '\n' try into (Shape) with [chars try into ([bool; 3]) with |c| c == '#']]
            }
            => cells
        ]
    );
    parse!(regions =>
        [regions split on '\n' with
            {
                [width as usize]
                'x'
                [height as usize]
                ": "
                [shapes split as usize]
            }
            => Region { width, height, shapes }
        ]
    );
    (shapes, regions)
}

#[register_part]
fn part1(input: &str) -> usize {
    let (shapes, regions) = parse_input(input);
    let shapes: Vec<_> = shapes
        .into_iter()
        .map(|shape| shape.into_iter().flatten().filter(|v| *v).count())
        .collect();
    regions
        .into_iter()
        .filter(|region| {
            let size = region.width * region.height;
            let needed = region
                .shapes
                .iter()
                .enumerate()
                .map(|(shape, count)| shapes[shape] * count)
                .sum();
            size >= needed
        })
        .count()
}

#[register_part]
fn part2(_input: &str) -> &'static str {
    "I did it!"
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 2, notest)]
    static EXAMPLE_INPUT: &str = "
        0:
        ###
        ##.
        ##.

        1:
        ###
        ##.
        .##

        2:
        .##
        ###
        ##.

        3:
        ##.
        ###
        ##.

        4:
        ###
        #..
        ###

        5:
        ###
        .#.
        ###

        4x4: 0 0 0 0 2 0
        12x5: 1 0 1 0 2 2
        12x5: 1 0 1 0 3 2
    ";

    #[test]
    fn example_parse() {
        let (shapes, regions) = parse_input(&EXAMPLE_INPUT);
        assert_eq!(
            shapes,
            vec![
                [[true, true, true], [true, true, false], [true, true, false]],
                [[true, true, true], [true, true, false], [false, true, true]],
                [[false, true, true], [true, true, true], [true, true, false]],
                [[true, true, false], [true, true, true], [true, true, false]],
                [[true, true, true], [true, false, false], [true, true, true]],
                [[true, true, true], [false, true, false], [true, true, true]],
            ],
        );
        assert_eq!(
            regions,
            vec![
                Region {
                    width: 4,
                    height: 4,
                    shapes: vec![0, 0, 0, 0, 2, 0]
                },
                Region {
                    width: 12,
                    height: 5,
                    shapes: vec![1, 0, 1, 0, 2, 2]
                },
                Region {
                    width: 12,
                    height: 5,
                    shapes: vec![1, 0, 1, 0, 3, 2]
                },
            ],
        );
    }
}
