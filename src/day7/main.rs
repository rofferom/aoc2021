use std::fs;

const INPUT_PATH: &str = "src/day7/input.txt";

fn abs_diff(x: u32, y: u32) -> u32 {
    if x > y {
        x - y
    } else {
        y - x
    }
}

fn part1(input: &str) -> u32 {
    let mut crabs: Vec<u32> = input.split(',').map(|x| x.parse().unwrap()).collect();
    crabs.sort_unstable();

    let median = crabs[crabs.len() / 2];
    crabs.iter().map(|&x| abs_diff(median, x)).sum()
}

fn part2(input: &str) -> u32 {
    let crabs: Vec<u32> = input.split(',').map(|x| x.parse().unwrap()).collect();
    let mean = crabs.iter().sum::<u32>() / (crabs.len() as u32);
    let mut score = u32::MAX;

    for target in (mean - 1)..(mean + 2) {
        let local_score = crabs
            .iter()
            .map(|&x| {
                let dist = abs_diff(target, x);
                (dist * (dist + 1)) / 2
            })
            .sum();

        score = std::cmp::min(score, local_score);
    }

    score
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
