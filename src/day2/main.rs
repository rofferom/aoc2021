use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    let file = File::open("src/day2/input.txt").unwrap();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let v: Vec<&str> = line.split(" ").collect();
        let (direction, count) = (v[0], v[1].parse::<i32>().unwrap());

        match direction {
            "forward" => {
                horizontal += count;
                depth += aim * count;
            },
            "down" => {
                aim += count;
            },
            "up" => {
                aim -= count;
            }
            _ => {
            }
        }
    }

    println!("{}", horizontal * depth);
}