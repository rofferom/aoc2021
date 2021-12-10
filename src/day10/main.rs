use std::collections::HashMap;
use std::fs;

const INPUT_PATH: &str = "src/day10/input.txt";

enum State {
    Ok,
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn line_state(l: &str) -> State {
    let delimiters = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut stack: Vec<char> = vec![];

    for c in l.chars() {
        if delimiters.contains_key(&c) {
            stack.push(c);
        } else if let Some(open) = stack.pop() {
            let expected_close = delimiters.get(&open).unwrap();
            if *expected_close != c {
                return State::Corrupted(c);
            }
        }
    }

    if stack.is_empty() {
        State::Ok
    } else {
        State::Incomplete(stack)
    }
}

fn part1(input: &str) -> u64 {
    let scores_map = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    input
        .lines()
        .filter_map(|l| {
            if let State::Corrupted(invalid_char) = line_state(l) {
                Some(scores_map.get(&invalid_char).unwrap())
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let scores_map = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let mut scores: Vec<_> = input
        .lines()
        .filter_map(|l| {
            if let State::Incomplete(stack) = line_state(l) {
                Some(
                    stack
                        .iter()
                        .rev()
                        .fold(0, |score, c| score * 5 + scores_map.get(c).unwrap()),
                )
            } else {
                None
            }
        })
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn day10_part1() {
        assert_eq!(part1(INPUT), 26397);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 321237);
    }

    #[test]
    fn day10_part2() {
        assert_eq!(part2(INPUT), 288957);
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 2360030859);
    }
}
