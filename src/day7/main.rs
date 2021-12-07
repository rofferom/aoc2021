use std::fs;

const INPUT_PATH: &str = "src/day7/input.txt";

fn abs_diff(x: u32, y: u32) -> u32 {
    if x > y {
        x - y
    } else {
        y - x
    }
}

fn find_shortest_path(input: &str, distance_cb: fn(pos: u32, target: u32) -> u32) -> u32 {
    let crabs: Vec<_> = input.split(',').map(|x| x.parse().unwrap()).collect();

    let min_crab = *crabs.iter().min().unwrap();
    let max_crab = *crabs.iter().max().unwrap();

    (min_crab..(max_crab + 1))
        .map(|target| crabs.iter().map(|&x| distance_cb(x, target)).sum())
        .min()
        .unwrap()
}

fn part1(input: &str) -> u32 {
    find_shortest_path(input, |pos, target| abs_diff(pos, target))
}

fn part2(input: &str) -> u32 {
    find_shortest_path(input, |pos, target| {
        let dist = abs_diff(pos, target);
        (dist * (dist + 1)) / 2
    })
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn day7_part1() {
        assert_eq!(part1(INPUT), 37);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 344138);
    }

    #[test]
    fn day7_part2() {
        assert_eq!(part2(INPUT), 168);
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 94862124);
    }
}
