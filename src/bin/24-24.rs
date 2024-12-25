use std::{collections::HashMap, fs::File, io::Write as _};

use aoc::utils::parse;
use itertools::Itertools;

#[derive(Eq, PartialEq, Debug)]
enum Operand {
    And,
    Or,
    Xor,
}
impl From<&str> for Operand {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Operand::And,
            "OR" => Operand::Or,
            "XOR" => Operand::Xor,
            _ => panic!(),
        }
    }
}
impl Operand {
    fn calc(&self, lhs: bool, rhs: bool) -> bool {
        match self {
            Operand::And => lhs && rhs,
            Operand::Or => lhs || rhs,
            Operand::Xor => lhs ^ rhs,
        }
    }
}

type Wires<'a> = HashMap<&'a str, bool>;
type Gates<'a> = HashMap<&'a str, (&'a str, Operand, &'a str)>;

fn parse_input(input: &str) -> (Wires, Gates) {
    parse!(input => {
        [wires split on '\n' into (HashMap<_, _>) with
            { name ": " [value as u8] }
            => (name, value == 1)
        ]
        "\n\n"
        [gates split on '\n' into (HashMap<_, _>) with
            { lhs ' ' [op as Operand] ' ' rhs " -> " name }
            => (name, (lhs, op, rhs))
        ]
    } => (wires, gates))
}

fn to_decimal(wires: &Wires, prefix: char) -> usize {
    let mut result = 0;
    let nums = wires
        .iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .sorted_unstable_by_key(|(k, _)| *k)
        .rev();
    for (_, num) in nums {
        result = (result << 1) + usize::from(*num);
    }
    result
}

fn solve<'a>(wires: &mut Wires<'a>, gates: &Gates<'a>, swaps: Vec<Vec<&'a str>>) {
    let swaps: HashMap<_, _> = swaps
        .into_iter()
        .flat_map(|swap| [(swap[0], swap[1]), (swap[1], swap[0])])
        .collect();

    let mut done = false;
    while !done {
        done = true;
        for (mut key, (lhs, op, rhs)) in gates {
            key = swaps.get(key).unwrap_or(key);
            if wires.contains_key(key) {
                continue;
            }
            if let (Some(lhs), Some(rhs)) = (wires.get(lhs), wires.get(rhs)) {
                wires.insert(key, op.calc(*lhs, *rhs));
                done = false;
            }
        }
    }
}

#[allow(dead_code)]
fn dot(gates: &Gates, swaps: Vec<Vec<&str>>) {
    let swaps: HashMap<_, _> = swaps
        .into_iter()
        .flat_map(|swap| [(swap[0], swap[1]), (swap[1], swap[0])])
        .collect();
    let mut file = File::create("dot").unwrap();
    file.write_all(b"digraph {\n").unwrap();
    for (mut key, (lhs, op, rhs)) in gates {
        key = swaps.get(key).unwrap_or(key);
        let color = match op {
            Operand::And => "red",
            Operand::Or => "blue",
            Operand::Xor => "yellow",
        };
        file.write_fmt(format_args!("  {lhs} -> {key} [color={color}]\n"))
            .unwrap();
        file.write_fmt(format_args!("  {rhs} -> {key} [color={color}]\n"))
            .unwrap();
    }
    file.write_all(b"}").unwrap();
}

pub fn part1(input: &str) -> usize {
    let (mut wires, gates) = parse_input(input);
    solve(&mut wires, &gates, Vec::new());
    to_decimal(&wires, 'z')
}

pub fn part2(input: &str) -> String {
    let (mut wires, gates) = parse_input(input);
    let swaps = vec![
        vec!["z15", "qnw"],
        vec!["z20", "cqr"],
        vec!["z37", "vkg"],
        vec!["ncd", "nfj"],
    ];
    solve(&mut wires, &gates, swaps.clone());

    // dot(&gates, swaps.clone());

    let x = to_decimal(&wires, 'x');
    let y = to_decimal(&wires, 'y');
    let z = to_decimal(&wires, 'z');

    // println!("         4         3         2         1          ");
    // println!("    5432109876543210987654321098765432109876543210");
    // println!("x =  {x:b}");
    // println!("y =  {y:b}");
    // println!("z = {z:b}");
    // println!("w = {:b}", x + y);

    for i in 0..=45 {
        let modulo = 1 << i;
        let expected = (x + y) % modulo;
        let actual = z % modulo;
        assert_eq!(actual, expected, "Error at z{i}");
    }

    swaps.into_iter().flatten().sorted_unstable().join(",")
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 4)]
    static EXAMPLE_INPUT_1: &str = "
        x00: 1
        x01: 1
        x02: 1
        y00: 0
        y01: 1
        y02: 0

        x00 AND y00 -> z00
        x01 XOR y01 -> z01
        x02 OR y02 -> z02
    ";

    #[example_input(part1 = 2024)]
    static EXAMPLE_INPUT_2: &str = "
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
    ";

    // #[example_input(part2 = "z00,z01,z02,z05", notest)]
    // static EXAMPLE_INPUT_3: &str = "
    //     x00: 0
    //     x01: 1
    //     x02: 0
    //     x03: 1
    //     x04: 0
    //     x05: 1
    //     y00: 0
    //     y01: 0
    //     y02: 1
    //     y03: 1
    //     y04: 0
    //     y05: 1
    //
    //     x00 AND y00 -> z05
    //     x01 AND y01 -> z02
    //     x02 AND y02 -> z01
    //     x03 AND y03 -> z03
    //     x04 AND y04 -> z04
    //     x05 AND y05 -> z00
    // ";
    //
    // #[test]
    // fn example_parse() {
    //     let actual = parse_input(&EXAMPLE_INPUT);
    //     let expected = vec![1, 2];
    //     assert_eq!(actual, expected);
    // }
}
