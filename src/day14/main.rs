use std::collections::HashMap;
use std::fs;

const INPUT_PATH: &str = "src/day14/input.txt";

fn solve(input: &str, steps: u32) -> u64 {
    // Parse input
    let parts: Vec<&str> = input.split("\n\n").collect();

    let template = parts[0].to_string();

    let rules: HashMap<&str, &str> = parts[1]
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split(" -> ").collect();
            (parts[0], parts[1])
        })
        .collect();

    // List initial pairs
    let mut pairs: HashMap<String, u64> = HashMap::new();

    for i in 0..template.len() - 1 {
        let pair = &template[i..i + 2];

        let entry = pairs.entry(pair.to_string()).or_insert(0);
        *entry += 1;
    }

    // Do steps
    for _ in 0..steps {
        let mut new_pairs: HashMap<String, u64> = HashMap::new();

        let mut inject_pair = |pair: String, count: u64| {
            let entry = new_pairs.entry(pair).or_insert(0);
            *entry += count;
        };

        for (pair, count) in pairs.iter() {
            let first_char = pair.chars().next().unwrap();
            let second_char = pair.chars().nth(1).unwrap();
            let &target = rules.get(&pair[..]).unwrap();

            inject_pair(format!("{}{}", first_char, target), *count);
            inject_pair(format!("{}{}", target, second_char), *count);
        }

        pairs = new_pairs;
    }

    // Compute frequency
    let mut frequency: HashMap<char, u64> = HashMap::new();
    for (pair, count) in &pairs {
        let c = pair.chars().next().unwrap();

        let entry = frequency.entry(c).or_insert(0);
        *entry += count;
    }

    // Let char isn't include in the previous step
    let last_char = template.chars().last().unwrap();
    let entry = frequency.entry(last_char).or_insert(0);
    *entry += 1;

    frequency.values().max().unwrap() - frequency.values().min().unwrap()
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", solve(&input, 10));
    println!("Part 2: {}", solve(&input, 40));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn day14_part1() {
        assert_eq!(solve(INPUT, 10), 1588);
        assert_eq!(solve(&fs::read_to_string(INPUT_PATH).unwrap(), 10), 2010);
    }

    #[test]
    fn day14_part2() {
        assert_eq!(solve(INPUT, 10), 1588);
        assert_eq!(solve(INPUT, 40), 2188189693529);
        assert_eq!(
            solve(&fs::read_to_string(INPUT_PATH).unwrap(), 40),
            2437698971143
        );
    }
}
