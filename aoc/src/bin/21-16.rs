puzzle_runner::register_chapter!(book = "2021", title = "Packet Decoder");

use std::collections::VecDeque;

use derive_new::new;

type Bits = VecDeque<u8>;

trait BitHelper {
    fn take(&mut self, amount: usize) -> Bits;
    fn to_decimal(&self) -> u64;
}
impl BitHelper for Bits {
    fn take(&mut self, amount: usize) -> Bits {
        let mut result = Bits::new();
        for _ in 0..amount {
            result.push_back(self.pop_front().unwrap());
        }
        result
    }

    fn to_decimal(&self) -> u64 {
        let mut result = 0u64;
        for bit in self {
            result <<= 1;
            result += u64::from(*bit);
        }
        result
    }
}

#[derive(Debug, PartialEq, new)]
struct LiteralPacket {
    pub version: u8,
    pub value: u64,
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

fn parse_input(input: &str) -> Bits {
    input
        .chars()
        .flat_map(|c| {
            let val = c.to_digit(16).unwrap();
            vec![
                u8::from(val & (2u32.pow(3)) != 0),
                u8::from(val & (2u32.pow(2)) != 0),
                u8::from(val & (2u32.pow(1)) != 0),
                u8::from(val & (2u32.pow(0)) != 0),
            ]
        })
        .collect::<Bits>()
}

fn parse_packet(bits: &mut Bits) -> Packet {
    let version = bits.take(3).to_decimal() as u8;
    let type_id = bits.take(3).to_decimal() as u8;
    if type_id == 4 {
        let mut litbits = Bits::new();
        loop {
            let cont = bits.pop_front().unwrap();
            litbits.append(&mut bits.take(4));
            if cont == 0 {
                break;
            }
        }
        Packet::Literal(LiteralPacket::new(version, litbits.to_decimal()))
    } else {
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
        Packet::Operator(OperatorPacket::new(version, type_id, subpackets))
    }
}

fn resolve(packet: Packet) -> u64 {
    match packet {
        Packet::Literal(p) => p.value,
        Packet::Operator(p) => {
            let mut values = p.subpackets.into_iter().map(resolve).collect::<Vec<u64>>();
            match p.type_id {
                0 => values.iter().sum::<u64>(),
                1 => {
                    // product
                    let mut result = values.pop().unwrap();
                    while !values.is_empty() {
                        result *= values.pop().unwrap();
                    }
                    result
                }
                2 => *values.iter().min().unwrap(),
                3 => *values.iter().max().unwrap(),
                5 => u64::from(values[0] > values[1]),
                6 => u64::from(values[0] < values[1]),
                7 => u64::from(values[0] == values[1]),
                _ => 0,
            }
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut bits = parse_input(input);
    let mut remaining = vec![parse_packet(&mut bits)];
    let mut result = 0u64;
    while let Some(packet) = remaining.pop() {
        match packet {
            Packet::Literal(p) => {
                result += u64::from(p.version);
            }
            Packet::Operator(mut p) => {
                result += u64::from(p.version);
                remaining.append(&mut p.subpackets);
            }
        }
    }
    result
}

pub fn part2(input: &str) -> u64 {
    let mut bits = parse_input(input);
    let packet = parse_packet(&mut bits);
    resolve(packet)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 16)]
    static EXAMPLE_INPUT_1_1: &str = "8A004A801A8002F478";

    #[example_input(part1 = 12)]
    static EXAMPLE_INPUT_1_2: &str = "620080001611562C8802118E34";

    #[example_input(part1 = 23)]
    static EXAMPLE_INPUT_1_3: &str = "C0015000016115A2E0802F182340";

    #[example_input(part1 = 31)]
    static EXAMPLE_INPUT_1_4: &str = "A0016C880162017C3686B18A3D4780";

    #[example_input(part2 = 3)]
    static EXAMPLE_INPUT_2_1: &str = "C200B40A82";

    #[example_input(part2 = 54)]
    static EXAMPLE_INPUT_2_2: &str = "04005AC33890";

    #[example_input(part2 = 7)]
    static EXAMPLE_INPUT_2_3: &str = "880086C3E88112";

    #[example_input(part2 = 9)]
    static EXAMPLE_INPUT_2_4: &str = "CE00C43D881120";

    #[example_input(part2 = 1)]
    static EXAMPLE_INPUT_2_5: &str = "D8005AC2A8F0";

    #[example_input(part2 = 0)]
    static EXAMPLE_INPUT_2_6: &str = "F600BC2D8F";

    #[example_input(part2 = 0)]
    static EXAMPLE_INPUT_2_7: &str = "9C005AC2F8F0";

    #[example_input(part2 = 1)]
    static EXAMPLE_INPUT_2_8: &str = "9C0141080250320F1802104A08";

    #[test]
    fn example_parse() {
        assert_eq!(
            parse_input("D2FE28"),
            vec![
                1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0
            ]
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
}
