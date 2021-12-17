use std::io::{stdin, BufRead};
use std::cmp::{min, max};

#[derive(Debug)]
struct Packet {
    version: u32,
    type_id: u32,
    literal: Option<u64>,
    sub_packets: Vec<Packet>
}

impl Packet {
    fn version_sum(&self) -> u32 {
        self.version + self.sub_packets.iter().map(|p| p.version_sum()).sum::<u32>()
    }

    fn value(&self) -> u64 {
        match self.type_id {
            4 => self.literal.unwrap(),
            0 => self.sub_packets.iter().map(|p| p.value()).sum::<u64>(),
            1 => self.sub_packets.iter().map(|p| p.value()).fold(1, |p, v| p * v),
            2 => self.sub_packets.iter().map(|p| p.value()).fold(u64::MAX, |p, v| min(p, v)),
            3 => self.sub_packets.iter().map(|p| p.value()).fold(0, |p, v| max(p, v)),
            5 => if self.sub_packets[0].value() > self.sub_packets[1].value() { 1 } else { 0 },
            6 => if self.sub_packets[0].value() < self.sub_packets[1].value() { 1 } else { 0 },
            7 => if self.sub_packets[0].value() == self.sub_packets[1].value() { 1 } else { 0 },
            _ => panic!("Invalid type id: {}", self.type_id)
        }
    }
}

fn hex_to_bin(c: char) -> Vec<u8> {
    let binary_str = format!("{:b}", c.to_digit(16).unwrap());

    let mut result: Vec<u8> = binary_str.chars().map(|b| b.to_digit(2).unwrap()).map(|v| v as u8).collect();
    while result.len() < 4 {
        result.insert(0, 0);
    }
    result
}

fn to_decimal(binary: &[u8]) -> u64
{
    binary.iter().fold(0, |s, d| (s << 1) + (*d as u64))
}

fn to_decimal_u32(binary: &[u8]) -> u32
{
    to_decimal(binary) as u32
}

fn parse_packet(data: &[u8]) -> (Packet, usize)
{
    //println!("parse_packet: {:?} ({} bits)", data, data.len());
    let version = to_decimal_u32(&data[0..3]);
    let type_id = to_decimal_u32(&data[3..6]);

    //println!("version: {}, type id: {}", version, type_id);
    let mut literal: Option<u64> = None;
    let mut sub_packets: Vec<Packet> = Vec::new();
    let mut idx = 6;
    if type_id == 4 {
        let mut literal_value = 0;
        loop {
            literal_value <<=4;
            let nibble = &data[idx..idx+5];
            literal_value += to_decimal(&nibble[1..]);
            idx += 5;
            if nibble[0] == 0 {
                break;
            }
        }
        literal = Some(literal_value)
    } else {
        let length_type_id = data[6];
        idx = 7;
        let data_len: usize;
        let mut num_packets = usize::MAX;
        if length_type_id == 0 {
            data_len = to_decimal(&data[idx..idx+15]) as usize;
            idx += 15;
            //println!("data_len from packet: {}", data_len);
        } else {
            num_packets = to_decimal(&data[idx..idx+11]) as usize;
            idx += 11;
            data_len = (data.len() - idx) as usize;
            //println!("data_len remaining: {}", data_len);
        }
        let start_data = idx;
        while (idx - start_data) < data_len && sub_packets.len() < num_packets {
            let (packet, read_len) = parse_packet(&data[idx..start_data + data_len]);
            sub_packets.push(packet);
            idx += read_len;
        }
    }

    //println!("Read {} bits", idx);
    (Packet{version, type_id, literal, sub_packets}, idx)
}

fn main() {
   let transmissions: Vec<Vec<u8>> = stdin().lock().lines()
    .map(|l| l.unwrap())
    .filter(|l| l.len() > 0)
    .map(|l| l
        .chars()
        .flat_map(hex_to_bin)
        .collect::<Vec<u8>>())
    .collect();

    for transmission in transmissions {
        let (packet, _len) = parse_packet(&transmission);
        println!("{:?}, version_sum: {}, value: {}", packet, packet.version_sum(), packet.value());
    }
}
