use std::fs;

const INPUT_PATH: &str = "src/day25/input.txt";

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Empty,
    East,
    South,
}

fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '>' => Cell::East,
                    'v' => Cell::South,
                    _ => panic!("Unexpected char {}", c),
                })
                .collect()
        })
        .collect()
}

fn run(mut input: Vec<Vec<Cell>>) -> (Vec<Vec<Cell>>, bool) {
    let mut updated = false;
    let height = input.len();
    let width = input[0].len();

    let mut next_input = input.clone();

    // Move east
    for y in 0..height {
        let mut next_is_moved = false;

        for x in 0..width {
            if next_is_moved {
                next_is_moved = false;
                continue;
            }

            let next_idx = (x + 1) % width;

            if input[y][x] == Cell::East && input[y][next_idx] == Cell::Empty {
                next_input[y][x] = Cell::Empty;
                next_input[y][next_idx] = Cell::East;

                next_is_moved = true;
                updated = true;
            } else {
                next_is_moved = false;
            }
        }
    }

    // Move south
    input = next_input;
    next_input = input.clone();

    for x in 0..width {
        let mut next_is_moved = false;

        for y in 0..height {
            if next_is_moved {
                next_is_moved = false;
                continue;
            }

            let next_idx = (y + 1) % height;

            if input[y][x] == Cell::South && input[next_idx][x] == Cell::Empty {
                next_input[y][x] = Cell::Empty;
                next_input[next_idx][x] = Cell::South;

                next_is_moved = true;
                updated = true;
            } else {
                next_is_moved = false;
            }
        }
    }

    (next_input, updated)
}

fn part1(input: &str) -> u32 {
    let mut input = parse_input(input);

    let mut steps = 0;
    loop {
        steps += 1;

        let (next_input, updated) = run(input);
        if !updated {
            break;
        }

        input = next_input;
    }

    steps
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn day25() {
        assert_eq!(part1(INPUT), 58);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 419);
    }
}
