use std::fs;

const INPUT_PATH: &str = "src/day17/input.txt";

#[derive(Debug)]
struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn contains(&self, x: i32) -> bool {
        self.min <= x && x <= self.max
    }
}

#[derive(Debug)]
struct Solution {
    x0: i32,
    y0: i32,
    max_y: i32,
}

fn parse_input(input: &str) -> (Range, Range) {
    // target area: x=20..30, y=-10..-5
    let splitted: Vec<_> = input[13..].split(", ").collect();

    let split_range = |input: &str| -> Range {
        let s: Vec<_> = input.split("..").collect();

        Range {
            min: s[0].parse().unwrap(),
            max: s[1].parse().unwrap(),
        }
    };

    (
        split_range(&splitted[0][2..]),
        split_range(&splitted[1][2..]),
    )
}

fn compute_x(x0: i32, n: i32) -> i32 {
    if n > x0 {
        compute_x(x0, x0)
    } else {
        n * x0 - ((n - 1) * n) / 2
    }
}

fn compute_y(x0: i32, n: i32) -> i32 {
    n * x0 - ((n - 1) * n) / 2
}

fn solve(input: &str) -> Vec<Solution> {
    let (range_x, range_y) = parse_input(input);

    // Get all valid x0
    let valid_x: Vec<i32> = {
        let mut valid_x: Vec<i32> = (range_x.min..range_x.max + 1).collect();

        for x0 in 1..range_x.min {
            for n in 1..x0 + 1 {
                let x = compute_x(x0, n);

                if x < range_x.min {
                    continue;
                } else if x > range_x.max {
                    break;
                } else {
                    valid_x.push(x0);
                    break;
                }
            }
        }

        valid_x
    };

    // Get all valid y0
    let valid_y: Vec<i32> = (range_y.min..range_y.min.abs()).collect();

    let mut solutions = vec![];

    for x0 in valid_x {
        for y0 in &valid_y {
            let mut trajectory_max_y = i32::MIN;

            for n in 1.. {
                // x will always be valid
                let x = compute_x(x0, n);
                if x < range_x.min {
                    continue;
                }

                // y can be out of range
                let y = compute_y(*y0, n);
                if y < range_y.min {
                    break;
                }

                trajectory_max_y = std::cmp::max(trajectory_max_y, y);

                if range_x.contains(x) && range_y.contains(y) {
                    solutions.push(Solution {
                        x0,
                        y0: *y0,
                        max_y: trajectory_max_y,
                    });

                    break;
                }
            }
        }
    }

    solutions
}

fn part1(input: &str) -> i32 {
    let results = solve(input);

    results.iter().map(|x| x.max_y).max().unwrap()
}

fn part2(input: &str) -> usize {
    let results = solve(input);
    results.len()
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn day17_part1() {
        assert_eq!(part1(INPUT), 45);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 3916);
    }

    #[test]
    fn day17_part2() {
        assert_eq!(part2(INPUT), 112);
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 2986);
    }
}
