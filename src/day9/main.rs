use std::collections::HashSet;
use std::fs;

const INPUT_PATH: &str = "src/day9/input.txt";

fn part1(input: &str) -> u32 {
    let heightmap: Vec<Vec<u32>> = input
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    let columns = heightmap[0].len() as i32;
    let rows = heightmap.len() as i32;

    let is_valid_pos =
        |row: i32, col: i32| -> bool { 0 <= row && row < rows && 0 <= col && col < columns };

    let mut risk = 0;

    for (row_idx, row) in heightmap.iter().enumerate() {
        for (column_idx, &current) in row.iter().enumerate() {
            let positions = [
                (row_idx as i32, column_idx as i32 - 1),
                (row_idx as i32, column_idx as i32 + 1),
                (row_idx as i32 - 1, column_idx as i32),
                (row_idx as i32 + 1, column_idx as i32),
            ];

            let adj_min = positions
                .iter()
                .filter_map(|(row, column)| {
                    if is_valid_pos(*row, *column) {
                        Some(heightmap[*row as usize][*column as usize])
                    } else {
                        None
                    }
                })
                .min()
                .unwrap();

            if current < adj_min {
                risk += current + 1;
            }
        }
    }

    risk
}

fn part2(input: &str) -> u32 {
    let heightmap: Vec<Vec<u32>> = input
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    let columns = heightmap[0].len();
    let rows = heightmap.len();

    let mut hitmap: HashSet<(usize, usize)> = HashSet::new();

    let is_valid_pos = |row: i32, col: i32| -> bool {
        (0 <= row && row < rows as i32 && 0 <= col && col < columns as i32)
            && (heightmap[row as usize][col as usize] != 9)
    };

    // Mark 9 as already visited
    for row_idx in 0..rows {
        for column_idx in 0..columns {
            if heightmap[row_idx][column_idx] == 9 {
                hitmap.insert((row_idx, column_idx));
            }
        }
    }

    let mut bassins_size: Vec<u32> = vec![];

    for row_idx in 0..rows {
        for column_idx in 0..columns {
            if hitmap.contains(&(row_idx, column_idx)) {
                continue;
            }

            let mut bassin: HashSet<(usize, usize)> = HashSet::new();
            bassin.insert((row_idx, column_idx));

            let mut next_positions: HashSet<(usize, usize)> = HashSet::new();
            next_positions.insert((row_idx, column_idx));

            while !next_positions.is_empty() {
                let mut visited: HashSet<(usize, usize)> = HashSet::new();

                for (row_idx, column_idx) in &next_positions {
                    let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)];

                    for (x, y) in directions {
                        let mut next_row = *row_idx as i32 + y;
                        let mut next_col = *column_idx as i32 + x;

                        while is_valid_pos(next_row, next_col)
                            && !bassin.contains(&(next_row as usize, next_col as usize))
                        {
                            visited.insert((next_row as usize, next_col as usize));

                            next_col += x;
                            next_row += y;
                        }
                    }
                }

                next_positions = visited.difference(&bassin).copied().collect();
                bassin = bassin.union(&visited).copied().collect();
            }

            hitmap = hitmap.union(&bassin).copied().collect();
            bassins_size.push(bassin.len() as u32);
        }
    }

    bassins_size.sort_by(|a, b| b.cmp(a));
    bassins_size.iter().take(3).product()
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn day9_part1() {
        assert_eq!(part1(INPUT), 15);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 548);
    }

    #[test]
    fn day9_part2() {
        assert_eq!(part2(INPUT), 1134);
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 786048);
    }
}
