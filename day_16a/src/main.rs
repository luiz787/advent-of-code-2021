use std::ops::Add;

#[derive(Debug)]
struct LiteralPacket {
    version: u64,
    _type_id: u64,
    _value: u64,
    size: usize,
}

#[derive(Debug)]
struct OperatorPacket {
    version: u64,
    _type_id: u64,
    children: Vec<Packet>,
    size: usize
}

#[derive(Debug)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

fn main() {
    let input = include_str!("../input").trim();
    let decoded = parse_input(input);

    let version = to_number(&decoded[0..3]);
    let type_id = to_number(&decoded[3..6]);

    let root_packet: Packet = match type_id {
        4 => decode_literal(version, type_id, &decoded[6..]),
        _ => decode_operator(version, type_id, &decoded[6..]),
    };

    println!("{:?}", sum_version(&root_packet));
}

fn sum_version(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(lp) => lp.version,
        Packet::Operator(op) => op.version + op.children.iter().map(sum_version).sum::<u64>()
    }
}

fn decode_operator(version: u64, type_id: u64, val: &str) -> Packet {
    let length_type_id = to_number(&val[0..1]);

    let (rest_size, children) = match length_type_id {
        0 => decode_children_by_length(&val[1..]),
        1 => decode_children_by_count(&val[1..]),
        _ => unreachable!(),
    };

    let children_size: usize = children.iter().map(|c| match c {
        Packet::Literal(p) => p.size,
        Packet::Operator(o) => o.size
    }).sum();

    Packet::Operator(OperatorPacket {
        version,
        _type_id: type_id,
        children,
        size: rest_size + children_size + 7
    })
}

fn decode_children_by_length(val: &str) -> (usize, Vec<Packet>) {
    let length = to_number(&val[..15]);

    let mut children = Vec::new();

    let mut rest = &val[15..(15 + length) as usize];

    loop {
        let version = to_number(&rest[0..3]);
        let type_id = to_number(&rest[3..6]);

        let packet: Packet = match type_id {
            4 => decode_literal(version, type_id, &rest[6..]),
            _ => decode_operator(version, type_id, &rest[6..]),
        };
        let child_size = match &packet {
            Packet::Literal(p) => p.size,
            Packet::Operator(o) => o.size
        };

        children.push(packet);
        if child_size >= rest.len() {
            break;
        }
        rest = &rest[child_size..];
    }

    (15, children)
}

fn decode_children_by_count(val: &str) -> (usize, Vec<Packet>) {
    let count = to_number(&val[..11]);
    let mut children = Vec::new();

    let mut offset = 11;

    for _ in 0..count {
        let version = to_number(&val[offset..offset + 3]);
        let type_id = to_number(&val[offset + 3..offset + 6]);

        let packet: Packet = match type_id {
            4 => decode_literal(version, type_id, &val[offset + 6..]),
            _ => decode_operator(version, type_id, &val[offset + 6..]),
        };

        let child_size = match &packet {
            Packet::Literal(p) => p.size,
            Packet::Operator(o) => o.size
        };

        children.push(packet);

        offset += child_size;
    }

    (11, children)
}

fn decode_literal(version: u64, type_id: u64, val: &str) -> Packet {
    let mut pos = 0;
    let mut accumulator = String::new();
    let mut size = 6;

    let mut current = &val[pos..pos + 5];
    while let Some('1') = current.chars().next() {
        accumulator += &current[1..];

        pos += 5;
        current = &val[pos..pos + 5];
        size += 5;
    }
    accumulator += &current[1..];
    size += 5;

    Packet::Literal(LiteralPacket {
        version,
        _type_id: type_id,
        _value: to_number(&accumulator),
        size,
    })
}

fn to_number(s: &str) -> u64 {
    u64::from_str_radix(s, 2).unwrap()
}

fn parse_input(input: &str) -> String {
    input
        .trim()
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .fold("".to_owned(), |a, b| a.add(&b))
}
