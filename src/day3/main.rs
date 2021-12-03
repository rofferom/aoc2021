use std::io::{BufRead, BufReader};
use std::fs::File;

fn insert_bit(value: u32, b: u32) -> u32 {
    (value << 1) | b
}

fn part1() -> u32 {
    let file = File::open("src/day3/input.txt").unwrap();

    let mut line_count = 0;
    let mut bits: Vec<u32> = vec![];

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

fn extract_value(mut values: Vec<String>, filter_cb: fn(u32, u32) -> char) -> u32 {
    let value_len = values[0].len();

    for pos in 0..value_len {
        let ones = values.iter().map(|v| v.chars().nth(pos).unwrap().to_digit(10).unwrap()).sum();
        let zeroes = values.len() as u32 - ones;

        let criteria = filter_cb(ones, zeroes);
        values = values.iter().filter(|v| v.chars().nth(pos).unwrap() == criteria).cloned().collect();

        if values.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(&values[0], 2).unwrap()
}

fn part2() -> u32 {
    let file = File::open("src/day3/input.txt").unwrap();
    let values: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    // Oxygen
    let oxygen_filter_cb = |ones: u32, zeroes: u32| {
        if ones >= zeroes {
            '1'
        } else {
            '0'
        }
    };

    let oxygen = extract_value(values.clone(), oxygen_filter_cb);

    // CO2
    let co2_filter_cb = |ones: u32, zeroes: u32| {
        if zeroes <= ones {
            '0'
        } else {
            '1'
        }
    };

    let co2 = extract_value(values, co2_filter_cb);

    oxygen * co2
}

fn main() {
    println!("Part1: {}", part1());
    println!("Part2: {}", part2());
}