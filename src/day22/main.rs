use std::collections::HashSet;
use std::fs;

const INPUT_PATH: &str = "src/day22/input.txt";

struct Range {
    lowest: i32,
    highest: i32,
}

struct Step {
    on: bool,
    x: Range,
    y: Range,
    z: Range,
}

fn parse_range(input: &str) -> Range {
    let separator_idx = input.find("..").unwrap();

    let left = &input[2..separator_idx];
    let right = &input[separator_idx + 2..];

    Range {
        lowest: left.parse().unwrap(),
        highest: right.parse().unwrap(),
    }
}

fn parse_input(input: &str) -> Vec<Step> {
    input
        .lines()
        .map(|l| {
            let s: Vec<_> = l.split(' ').collect();
            let on = s[0] == "on";

            let steps: Vec<_> = s[1].split(',').collect();
            let x = parse_range(steps[0]);
            let y = parse_range(steps[1]);
            let z = parse_range(steps[2]);

            Step { on, x, y, z }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let steps = parse_input(input);
    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();

    let in_range = |a| -50 <= a && a <= 50;

    for step in steps {
        for x in step.x.lowest..step.x.highest + 1 {
            if !in_range(x) {
                continue;
            }

            for y in step.y.lowest..step.y.highest + 1 {
                if !in_range(y) {
                    continue;
                }

                for z in step.z.lowest..step.z.highest + 1 {
                    if !in_range(z) {
                        continue;
                    }

                    if step.on {
                        cubes.insert((x, y, z));
                    } else {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    cubes.len()
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    #[test]
    fn day22_part1() {
        assert_eq!(part1(INPUT), 590784);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 582644);
    }
}
