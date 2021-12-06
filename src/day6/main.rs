use std::fs;

const INPUT_PATH: &str = "src/day6/input.txt";

fn compute_input(input: &str, days: u32) -> u64 {
    let mut fishes: [u64; 9] = [0; 9];
    for fish in input.split(',').map(|x| x.parse::<usize>().unwrap()) {
        fishes[fish] += 1;
    }

    let mut idx: usize = 0;

    for _ in 0..days {
        let front = fishes[idx];
        idx = (idx + 1) % 9;

        fishes[(idx + 6) % 9] += front;
        fishes[(idx + 8) % 9] = front;
    }

    fishes.iter().sum()
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", compute_input(&input, 80));
    println!("Part 2: {}", compute_input(&input, 256));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "3,4,3,1,2";

    #[test]
    fn day6_part1() {
        assert_eq!(compute_input(INPUT, 18), 26);
        assert_eq!(compute_input(INPUT, 80), 5934);
        assert_eq!(
            compute_input(&fs::read_to_string(INPUT_PATH).unwrap(), 80),
            356190
        );
    }

    #[test]
    fn day6_part2() {
        assert_eq!(compute_input(INPUT, 256), 26984457539);
        assert_eq!(
            compute_input(&fs::read_to_string(INPUT_PATH).unwrap(), 256),
            1617359101538
        );
    }
}
