use std::collections::{HashMap, HashSet};
use std::fs;

const INPUT_PATH: &str = "src/day8/input.txt";

fn parse_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .lines()
        .map(|l| {
            let parsed: Vec<Vec<_>> = l
                .split(" | ")
                .map(|x| {
                    x.split(' ')
                        .map(|s| {
                            let mut v: Vec<_> = s.chars().collect();
                            v.sort_unstable();
                            String::from_iter(v)
                        })
                        .collect()
                })
                .collect();

            (parsed[0].clone(), parsed[1].clone())
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    let v = parse_input(input);

    v.iter()
        .map(|(_, output)| output)
        .flatten()
        .filter(|&x| {
            let l = x.len();
            l == 2 || l == 3 || l == 4 || l == 7
        })
        .count() as u32
}

fn solve_problem(str_input: &[String], output: &[String]) -> u32 {
    let str_to_hashset = |input: &str| -> HashSet<char> { input.chars().collect() };
    let input: Vec<_> = str_input.iter().map(|x| str_to_hashset(x)).collect();

    // Find easy ones
    let one = input.iter().find(|&x| x.len() == 2).unwrap();
    let four = input.iter().find(|&x| x.len() == 4).unwrap();
    let seven = input.iter().find(|&x| x.len() == 3).unwrap();
    let eight = input.iter().find(|&x| x.len() == 7).unwrap();

    // Find nine
    let nine = input
        .iter()
        .find(|&x| {
            let merged: HashSet<_> = four.union(seven).copied().collect();
            x.len() == 6 && x.difference(&merged).count() == 1
        })
        .unwrap();

    // Find two
    let two = input
        .iter()
        .find(|&x| x.len() == 5 && x.difference(nine).count() == 1)
        .unwrap();

    // Find zero
    let zero = input
        .iter()
        .find(|&x| x.len() == 6 && x != nine && x.intersection(one).count() == 2)
        .unwrap();

    // Find six
    let six = input
        .iter()
        .find(|&x| x.len() == 6 && x != nine && x != zero)
        .unwrap();

    // Find five
    let five = input
        .iter()
        .find(|&x| x.len() == 5 && x != two && x.intersection(six).count() == 5)
        .unwrap();

    // Find three
    let three = input
        .iter()
        .find(|&x| x.len() == 5 && x != two && x.intersection(six).count() == 4)
        .unwrap();

    // Build final map from results
    let map: HashMap<_, _> = [zero, one, two, three, four, five, six, seven, eight, nine]
        .iter()
        .enumerate()
        .map(|(digit, &r)| {
            // Find HashSet position in the input to get the original form
            let result_idx = str_input
                .iter()
                .enumerate()
                .find_map(|(i, x)| {
                    if str_to_hashset(x) == *r {
                        Some(i)
                    } else {
                        None
                    }
                })
                .unwrap();

            (str_input[result_idx].clone(), digit)
        })
        .collect();

    // Compute result
    output
        .iter()
        .fold(0, |acc, x| acc * 10 + *map.get(x).unwrap() as u32)
}

fn part2(input: &str) -> u32 {
    let v = parse_input(input);

    v.iter().map(|(x, y)| solve_problem(x, y)).sum()
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn day8_part1() {
        assert_eq!(part1(INPUT), 26);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 521);
    }

    #[test]
    fn day8_part2() {
        assert_eq!(part2(INPUT), 61229);
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 1016804);
    }
}
