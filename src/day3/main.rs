use std::fs::File;
use std::io::{BufRead, BufReader};

fn insert_bit(value: u32, b: u32) -> u32 {
    (value << 1) | b
}

fn part1() -> u32 {
    let file = File::open("src/day3/input.txt").unwrap();

    let mut line_count = 0;
    let mut bits = vec![];

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        if bits.is_empty() {
            bits.resize(line.len(), 0);
        }

        for (idx, c) in line.chars().enumerate() {
            if c == '1' {
                bits[idx] += 1;
            }
        }

        line_count += 1;
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    let one_threshold = line_count / 2;

    for b in bits {
        if b > one_threshold {
            gamma = insert_bit(gamma, 1);
            epsilon = insert_bit(epsilon, 0);
        } else {
            gamma = insert_bit(gamma, 0);
            epsilon = insert_bit(epsilon, 1);
        }
    }

    gamma * epsilon
}

fn extract_value(mut values: Vec<u32>, value_len: u32, filter_cb: fn(u32, u32) -> u32) -> u32 {
    for pos in 0..value_len {
        let get_bit = |v, pos| (v >> (value_len - pos - 1)) & 1;

        let ones = values.iter().map(|v| get_bit(v, pos)).sum();
        let zeroes = values.len() as u32 - ones;

        values = values
            .iter()
            .filter(|v| get_bit(v, pos) == filter_cb(ones, zeroes))
            .cloned()
            .collect();
        if values.len() == 1 {
            break;
        }
    }

    values[0]
}

fn part2() -> u32 {
    let file = File::open("src/day3/input.txt").unwrap();

    let str_values: Vec<_> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
    let value_len = str_values[0].len() as u32;

    let values: Vec<_> = str_values
        .iter()
        .map(|v| u32::from_str_radix(v, 2).unwrap())
        .collect();

    // Oxygen
    let oxygen_filter_cb = |ones, zeroes| {
        if ones >= zeroes {
            1
        } else {
            0
        }
    };

    let oxygen = extract_value(values.clone(), value_len, oxygen_filter_cb);

    // CO2
    let co2_filter_cb = |ones, zeroes| {
        if zeroes <= ones {
            0
        } else {
            1
        }
    };

    let co2 = extract_value(values, value_len, co2_filter_cb);

    oxygen * co2
}

fn main() {
    println!("Part1: {}", part1());
    println!("Part2: {}", part2());
}
