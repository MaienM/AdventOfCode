puzzle_runner::register_chapter!(book = "2015", title = "Aunt Sue");

use std::collections::HashMap;

use common_macros::hash_map;

struct Aunt<'a> {
    num: u16,
    compounds: HashMap<&'a str, u8>,
}

fn parse_input(input: &str) -> Vec<Aunt<'_>> {
    parse!(input => {
        [aunts split on '\n' with
            {
                "Sue "
                [num as u16]
                ": "
                [compounds split on ", " into (HashMap<_, _>) with
                    {
                        name
                        ": "
                        [count as u8]
                    }
                    => (name, count)
                ]
            }
            => Aunt { num, compounds }
        ]
    } => aunts)
}

pub fn part1(input: &str) -> u16 {
    let aunts = parse_input(input);
    let wanted = hash_map![
        "children" => 3,
        "cats" => 7,
        "samoyeds" => 2,
        "pomeranians" => 3,
        "akitas" => 0,
        "vizslas" => 0,
        "goldfish" => 5,
        "trees" => 3,
        "cars" => 2,
        "perfumes" => 1,
    ];
    'aunt: for aunt in aunts {
        for (compound, count) in aunt.compounds {
            if wanted[compound] != count {
                continue 'aunt;
            }
        }
        return aunt.num;
    }
    panic!("Should never happen.");
}
