use std::collections::HashSet;
use std::fs;

const INPUT_PATH: &str = "src/day11/input.txt";

fn parse_grid(input: &str) -> (Vec<Vec<u32>>, usize, usize) {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    let columns = grid[0].len();
    let rows = grid.len();

    (grid, columns, rows)
}

fn step_grid(grid: &mut Vec<Vec<u32>>, columns: usize, rows: usize) -> usize {
    let is_valid_pos = |row: i32, col: i32| -> bool {
        0 <= row && row < rows as i32 && 0 <= col && col < columns as i32
    };

    let mut hitmap: HashSet<(usize, usize)> = HashSet::new();

    for (row_idx, row) in grid.iter_mut().enumerate() {
        for (column_idx, octopus) in row.iter_mut().enumerate() {
            *octopus += 1;

            if *octopus == 10 {
                hitmap.insert((row_idx, column_idx));
            }
        }
    }

    while !hitmap.is_empty() {
        let mut round_hitmap: HashSet<(usize, usize)> = HashSet::new();

        for (row_idx, column_idx) in &hitmap {
            let directions = [
                (-1, 0),
                (1, 0),
                (0, 1),
                (0, -1),
                (-1, 1),
                (-1, -1),
                (1, 1),
                (1, -1),
            ];

            for (x, y) in directions {
                let next_row = *row_idx as i32 + y;
                let next_col = *column_idx as i32 + x;

                if !is_valid_pos(next_row, next_col) {
                    continue;
                }

                let octopus = &mut grid[next_row as usize][next_col as usize];
                if *octopus == 10 {
                    continue;
                } else if *octopus == 9 {
                    round_hitmap.insert((next_row as usize, next_col as usize));
                }

                *octopus += 1;
            }
        }

        hitmap = round_hitmap;
    }

    let mut flashes = 0;

    for (_, row) in grid.iter_mut().enumerate() {
        for (_, octopus) in row.iter_mut().enumerate() {
            if *octopus == 10 {
                *octopus = 0;
                flashes += 1;
            }
        }
    }

    flashes
}

fn part1(input: &str) -> usize {
    let (mut grid, columns, rows) = parse_grid(input);

    let mut flashes = 0;

    for _ in 0..100 {
        flashes += step_grid(&mut grid, columns, rows);
    }

    flashes
}

fn part2(input: &str) -> u32 {
    let (mut grid, columns, rows) = parse_grid(input);

    let mut i = 1;

    loop {
        let flashes = step_grid(&mut grid, columns, rows);
        if flashes == rows * columns {
            return i;
        }

        i += 1;
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn day11_part1() {
        assert_eq!(part1(INPUT), 1656);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 1603);
    }

    #[test]
    fn day11_part2() {
        assert_eq!(part2(INPUT), 195);
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 222);
    }
}
