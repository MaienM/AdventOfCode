use std::collections::VecDeque;

use aoc::runner::*;

#[macro_use]
extern crate derive_new;

type Bits = VecDeque<u8>;

trait BitHelper {
    fn take(&mut self, amount: usize) -> Bits;
    fn to_decimal(&self) -> u128;
}
impl BitHelper for Bits {
    fn take(&mut self, amount: usize) -> Bits {
        let mut result = Bits::new();
        for _ in 0..amount {
            result.push_back(self.pop_front().unwrap());
        }
        return result;
    }

    fn to_decimal(&self) -> u128 {
        let mut result = 0u128;
        for bit in self {
            result = result << 1;
            result += *bit as u128;
        }
        return result;
    }
}

#[derive(Debug, PartialEq, new)]
struct LiteralPacket {
    pub version: u8,
    pub value: u128,
}

#[derive(Debug, PartialEq, new)]
struct OperatorPacket {
    pub version: u8,
    pub type_id: u8,
    pub subpackets: Vec<Packet>,
}

#[derive(Debug, PartialEq)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

fn parse_input(input: String) -> Bits {
    return input
        .trim()
        .chars()
        .flat_map(|c| {
            let val = c.to_digit(16).unwrap();
            return vec![
                (val & (2u32.pow(3)) != 0) as u8,
                (val & (2u32.pow(2)) != 0) as u8,
                (val & (2u32.pow(1)) != 0) as u8,
                (val & (2u32.pow(0)) != 0) as u8,
            ];
        })
        .collect::<Bits>();
}

fn parse_packet(bits: &mut Bits) -> Packet {
    let version = bits.take(3).to_decimal() as u8;
    let type_id = bits.take(3).to_decimal() as u8;
    match type_id {
        4 => {
            let mut litbits = Bits::new();
            loop {
                let cont = bits.pop_front().unwrap();
                litbits.append(&mut bits.take(4));
                if cont == 0 {
                    break;
                }
            }
            return Packet::Literal(LiteralPacket::new(version, litbits.to_decimal()));
        }
        _ => {
            let length_type_id = bits.pop_front().unwrap();
            let mut subpackets: Vec<Packet> = Vec::new();
            if length_type_id == 0 {
                let length_in_bits = bits.take(15).to_decimal();
                let mut subpacket_data = bits.take(length_in_bits as usize);
                while !subpacket_data.is_empty() {
                    subpackets.push(parse_packet(&mut subpacket_data));
                }
            } else {
                let subpacket_count = bits.take(11).to_decimal();
                for _ in 0..subpacket_count {
                    subpackets.push(parse_packet(bits));
                }
            }
            return Packet::Operator(OperatorPacket::new(version, type_id, subpackets));
        }
    }
}

fn resolve(packet: Packet) -> u128 {
    match packet {
        Packet::Literal(p) => {
            return p.value;
        }
        Packet::Operator(p) => {
            let mut values = p
                .subpackets
                .into_iter()
                .map(|sp| resolve(sp))
                .collect::<Vec<u128>>();
            return match p.type_id {
                0 => values.iter().sum::<u128>(),
                1 => {
                    // product
                    let mut result = values.pop().unwrap();
                    while !values.is_empty() {
                        result *= values.pop().unwrap();
                    }
                    return result;
                }
                2 => *values.iter().min().unwrap(),
                3 => *values.iter().max().unwrap(),
                5 => (values[0] > values[1]) as u128,
                6 => (values[0] < values[1]) as u128,
                7 => (values[0] == values[1]) as u128,
                _ => 0,
            };
        }
    }
}

fn part1(input: String) -> u128 {
    let mut bits = parse_input(input);
    let mut remaining = vec![parse_packet(&mut bits)];
    let mut result = 0u128;
    while !remaining.is_empty() {
        let packet = remaining.pop().unwrap();
        match packet {
            Packet::Literal(p) => {
                result += p.version as u128;
            }
            Packet::Operator(mut p) => {
                result += p.version as u128;
                remaining.append(&mut p.subpackets);
            }
        }
    }
    return result;
}

fn part2(input: String) -> u128 {
    let mut bits = parse_input(input);
    let packet = parse_packet(&mut bits);
    return resolve(packet);
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn example_parse() {
        assert_eq!(
            parse_input("D2FE28".to_string()),
            vec![1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]
        );
    }

    #[test]
    fn example_parse_packet_literal() {
        let mut bits: Bits = vec![
            1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0,
        ]
        .into();
        assert_eq!(
            parse_packet(&mut bits),
            Packet::Literal(LiteralPacket::new(6, 2021))
        );
    }

    #[test]
    fn example_parse_packet_operator_length_type_0() {
        let mut bits: Bits = vec![
            0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0,
            1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
        .into();
        assert_eq!(
            parse_packet(&mut bits),
            Packet::Operator(OperatorPacket::new(
                1,
                6,
                vec![
                    Packet::Literal(LiteralPacket::new(6, 10)),
                    Packet::Literal(LiteralPacket::new(2, 20)),
                ]
            ))
        );
    }

    #[test]
    fn example_parse_packet_operator_length_type_2() {
        let mut bits: Bits = vec![
            1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        ]
        .into();
        assert_eq!(
            parse_packet(&mut bits),
            Packet::Operator(OperatorPacket::new(
                7,
                3,
                vec![
                    Packet::Literal(LiteralPacket::new(2, 1)),
                    Packet::Literal(LiteralPacket::new(4, 2)),
                    Packet::Literal(LiteralPacket::new(1, 3)),
                ]
            ))
        );
    }

    #[test]
    fn example1_part1() {
        assert_eq!(part1("8A004A801A8002F478".to_string()), 16);
    }

    #[test]
    fn example2_part1() {
        assert_eq!(part1("620080001611562C8802118E34".to_string()), 12);
    }

    #[test]
    fn example3_part1() {
        assert_eq!(part1("C0015000016115A2E0802F182340".to_string()), 23);
    }

    #[test]
    fn example4_part1() {
        assert_eq!(part1("A0016C880162017C3686B18A3D4780".to_string()), 31);
    }

    #[test]
    fn example1_part2() {
        assert_eq!(part2("C200B40A82".to_string()), 3);
    }

    #[test]
    fn example2_part2() {
        assert_eq!(part2("04005AC33890".to_string()), 54);
    }

    #[test]
    fn example3_part2() {
        assert_eq!(part2("880086C3E88112".to_string()), 7);
    }

    #[test]
    fn example4_part2() {
        assert_eq!(part2("CE00C43D881120".to_string()), 9);
    }

    #[test]
    fn example5_part2() {
        assert_eq!(part2("D8005AC2A8F0".to_string()), 1);
    }

    #[test]
    fn example6_part2() {
        assert_eq!(part2("F600BC2D8F".to_string()), 0);
    }

    #[test]
    fn example7_part2() {
        assert_eq!(part2("9C005AC2F8F0".to_string()), 0);
    }

    #[test]
    fn example8_part2() {
        assert_eq!(part2("9C0141080250320F1802104A08".to_string()), 1);
    }
}
