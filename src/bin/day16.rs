use std::fs::read_to_string;

#[derive(Debug)]
struct Packet {
    #[allow(dead_code)]
    original_binary: String,
    version: u8,
    packet_id: u8,
    kind: Content,
}

#[derive(Debug)]
enum Content {
    Literal(u64),
    Operator(Vec<Packet>),
}

impl Packet {
    fn version_sum(&self) -> usize {
        self.version as usize
            + match &self.kind {
                Content::Literal(_) => 0,
                Content::Operator(rest) => rest.iter().map(|p| p.version_sum()).sum::<usize>(),
            }
    }

    fn evaluate(&self) -> u64 {
        match &self.kind {
            Content::Literal(n) => *n,
            Content::Operator(packets) => {
                let mut values = packets.iter().map(Self::evaluate);

                match self.packet_id {
                    0 => values.sum(),
                    1 => values.product(),
                    2 => values.min().unwrap(),
                    3 => values.max().unwrap(),
                    5 => {
                        if values.next() > values.next() {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        if values.next() < values.next() {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        if values.next() == values.next() {
                            1
                        } else {
                            0
                        }
                    }
                    x => panic!("unknown packet ID {}", x),
                }
            }
        }
    }
}

/// Decodes n packets
fn decode_multiple(mut binary: &str, count: usize) -> (Vec<Packet>, &str) {
    println!("decode multiple, given {}, {}", binary, count);

    let mut result = vec![];

    for _ in 0..count {
        let (packet, rest) = decode(binary);
        binary = rest;
        result.push(packet);
    }

    (result, binary)
}

/// Decodes all packets until the end
fn decode_all(mut binary: &str) -> Vec<Packet> {
    println!("decode all, given {}", binary);

    let mut result = vec![];

    while !binary.is_empty() {
        let (packet, rest) = decode(binary);
        result.push(packet);
        binary = rest;
    }

    result
}

fn decode_literal(mut binary: &str) -> (String, &str) {
    let mut result = "".to_string();

    loop {
        let group = &binary[..5];
        binary = &binary[5..];
        result.push_str(&group[1..]);

        if group.chars().nth(0).unwrap() == '0' {
            break;
        }
    }

    (result, binary)
}

fn decode(mut binary: &str) -> (Packet, &str) {
    println!("decode, given {}, len {}", binary, binary.len());

    let original_binary = binary.to_string();

    let version = u8::from_str_radix(&binary[..3], 2).unwrap();
    dbg!(version);
    binary = &binary[3..];

    let packet_id = u8::from_str_radix(&binary[..3], 2).unwrap();
    dbg!(packet_id);
    binary = &binary[3..];

    // literal packet
    let content = if packet_id == 4 {
        let (s, rest) = decode_literal(binary);
        binary = rest;
        Content::Literal(u64::from_str_radix(&s, 2).unwrap())
    // operator packet
    } else {
        let mut sub_packets = vec![];
        let length_type = &binary[..1];
        binary = &binary[1..];

        match length_type {
            "0" => {
                // Next 15 bits contain number of bits for sub packets
                let bit_count = usize::from_str_radix(&binary[..15], 2).unwrap();
                binary = &binary[15..];
                // NOTE: decode_all consumes the entire slice, so there's no need for `rest` being
                // returned. The recursion terminates here.
                let mut packets = decode_all(&binary[..bit_count]);
                binary = &binary[bit_count..];
                sub_packets.append(&mut packets);
            }
            "1" => {
                // Next 11 bits represent number of subpackets
                let packet_count = usize::from_str_radix(&binary[..11], 2).unwrap();
                binary = &binary[11..];
                let (mut packets, rest) = decode_multiple(binary, packet_count);
                binary = rest;
                sub_packets.append(&mut packets);
            }
            _ => panic!("should be bit"),
        }
        Content::Operator(sub_packets)
    };

    (
        Packet {
            original_binary,
            version,
            packet_id,
            kind: content,
        },
        binary,
    )
}

fn hex_to_binary(hex: &str) -> String {
    let mut binary: String = String::with_capacity(hex.len() * 4);

    for hex in hex.chars() {
        let n = u8::from_str_radix(&hex.to_string(), 16).unwrap();
        binary.push_str(&format!("{:04b}", n));
    }

    return binary;
}

fn main() {
    let input = read_to_string("inputs/day16.txt").expect("file not found");

    let input = input.trim_end();

    let binary: String = hex_to_binary(&input);

    let (decoded, _rest) = decode(&binary);

    println!("Part 1: {}", decoded.version_sum());

    println!("Part 2: {}", decoded.evaluate());
}
