use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

const INPUT_PATH: &str = "src/day5/input.txt";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Segment {
    origin: Point,
    end: Point,
}

impl Segment {
    fn from_str(s: &str) -> Self {
        let segments: Vec<_> = s
            .split(" -> ")
            .map(|v| {
                let splitted: Vec<_> = v.split(',').map(|x| x.parse().unwrap()).collect();

                Point {
                    x: splitted[0],
                    y: splitted[1],
                }
            })
            .collect();

        Segment {
            origin: segments[0],
            end: segments[1],
        }
    }

    fn vert_or_horiz(&self) -> bool {
        self.origin.x == self.end.x || self.origin.y == self.end.y
    }
}

fn get_increment(x: i32, y: i32) -> i32 {
    match x.cmp(&y) {
        Ordering::Greater => -1,
        Ordering::Less => 1,
        Ordering::Equal => 0,
    }
}

fn score(segments: Vec<Segment>) -> u32 {
    let mut diagram = HashMap::new();

    let mut update_diagram = |p: Point| {
        diagram.entry(p).and_modify(|e| *e += 1).or_insert(1);
    };

    for segment in segments {
        let x_inc = get_increment(segment.origin.x, segment.end.x);
        let y_inc = get_increment(segment.origin.y, segment.end.y);

        let mut current = segment.origin;
        while current != segment.end {
            update_diagram(current);

            current.x += x_inc;
            current.y += y_inc;
        }

        update_diagram(current);
    }

    diagram
        .iter()
        .filter_map(|(_, &v)| if v > 1 { Some(v) } else { None })
        .count() as u32
}

fn parse_input(input: &str) -> Vec<Segment> {
    input.lines().map(|l| Segment::from_str(l)).collect()
}

fn part1(input: &str) -> u32 {
    let segments = parse_input(input)
        .iter()
        .cloned()
        .filter(|s| s.vert_or_horiz())
        .collect();
    score(segments)
}

fn part2(input: &str) -> u32 {
    let segments = parse_input(input);
    score(segments)
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn day5_part1() {
        assert_eq!(part1(INPUT), 5);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 6687);
    }

    #[test]
    fn day5_part2() {
        assert_eq!(part2(INPUT), 12);
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 19851);
    }
}
