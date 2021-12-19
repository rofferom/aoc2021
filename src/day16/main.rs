use std::cmp::min;
use std::fs;

const INPUT_PATH: &str = "src/day16/input.txt";

#[derive(Debug)]
struct BitReader {
    data: Vec<u8>,
    read_idx: usize,

    current_byte: u8,
    current_len: usize,
}

impl BitReader {
    fn new(input: &str) -> Self {
        let current_byte: Vec<u8> = input
            .chars()
            .take(1)
            .map(|c| c.to_digit(16).unwrap() as u8)
            .collect();

        let data: Vec<u8> = input
            .chars()
            .skip(1)
            .map(|c| c.to_digit(16).unwrap() as u8)
            .collect();

        Self {
            data,
            read_idx: 0,
            current_byte: current_byte[0],
            current_len: 4,
        }
    }

    fn read(&mut self, mut count: usize) -> u64 {
        let mut v: u64 = 0;

        while count > 0 {
            let chunk_size = min(count, self.current_len);

            v <<= chunk_size;
            v |= (self.current_byte >> (4 - chunk_size)) as u64;

            self.current_byte = (self.current_byte << chunk_size) & 0b1111;
            self.current_len -= chunk_size;
            count -= chunk_size;

            if self.current_len == 0 && self.read_idx < self.data.len() {
                assert!(self.read_idx < self.data.len());

                self.current_byte = self.data[self.read_idx];
                self.current_len = 4;
                self.read_idx += 1;
            }
        }

        v
    }
}

struct ReadResult {
    version_sum: u64,
    packet_len: u64,
    value: u64,
}

fn read_packet(reader: &mut BitReader) -> ReadResult {
    let mut version_sum = reader.read(3);
    let type_ = reader.read(3);

    let mut packet_len = 6;

    let value = if type_ == 4 {
        let mut subpacket_value = 0;
        let mut subpacket_len = 0;

        loop {
            let t = reader.read(5);
            subpacket_len += 5;

            subpacket_value = (subpacket_value << 4) | (t & 0b1111);

            if t & 0b10000 == 0 {
                break;
            }
        }

        packet_len += subpacket_len;

        subpacket_value
    } else {
        let len_type = reader.read(1);
        packet_len += 1;

        let mut subpackets = vec![];

        if len_type == 0 {
            let mut remaining = reader.read(15);
            packet_len += 15;

            while remaining > 0 {
                let subpacket = read_packet(reader);
                subpackets.push(subpacket.value);

                remaining -= subpacket.packet_len;
                packet_len += subpacket.packet_len;
                version_sum += subpacket.version_sum;
            }
        } else {
            let count = reader.read(11);
            packet_len += 11;

            for _ in 0..count {
                let subpacket = read_packet(reader);
                subpackets.push(subpacket.value);

                packet_len += subpacket.packet_len;
                version_sum += subpacket.version_sum;
            }
        }

        match type_ {
            0 => subpackets.iter().sum(),
            1 => subpackets.iter().product(),
            2 => subpackets.iter().copied().min().unwrap(),
            3 => subpackets.iter().copied().max().unwrap(),
            5 => (subpackets[0] > subpackets[1]) as u64,
            6 => (subpackets[0] < subpackets[1]) as u64,
            7 => (subpackets[0] == subpackets[1]) as u64,
            _ => {
                panic!("Unexpected type {}", type_);
            }
        }
    };

    ReadResult {
        version_sum,
        packet_len,
        value,
    }
}

fn part1(input: &str) -> u64 {
    let mut reader = BitReader::new(input);

    let result = read_packet(&mut reader);
    result.version_sum
}

fn part2(input: &str) -> u64 {
    let mut reader = BitReader::new(input);

    let result = read_packet(&mut reader);
    result.value
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_part1() {
        assert_eq!(part1("8A004A801A8002F478"), 16);
        assert_eq!(part1("620080001611562C8802118E34"), 12);
        assert_eq!(part1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 957);
    }

    #[test]
    fn day16_part2() {
        assert_eq!(part2("C200B40A82"), 3);
        assert_eq!(part2("04005AC33890"), 54);
        assert_eq!(part2("880086C3E88112"), 7);
        assert_eq!(part2("CE00C43D881120"), 9);
        assert_eq!(part2("D8005AC2A8F0"), 1);
        assert_eq!(part2("F600BC2D8F"), 0);
        assert_eq!(part2("9C005AC2F8F0"), 0);
        assert_eq!(part2("9C0141080250320F1802104A08"), 1);
        assert_eq!(
            part2(&fs::read_to_string(INPUT_PATH).unwrap()),
            744953223228
        );
    }
}
