use std::fs;

const INPUT_PATH: &str = "src/day20/input.txt";

fn apply(grid: &[Vec<char>], enhancement: &[char], round: usize) -> Vec<Vec<char>> {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut next_grid: Vec<Vec<char>> = vec![];

    for row_idx in -1..rows + 1 {
        next_grid.push(vec![]);

        for col_idx in -1..cols + 1 {
            let moves: &[(i32, i32)] = &[
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (0, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ];

            let mut value = 0;

            for (delta_x, delta_y) in moves {
                let x = col_idx as i32 + delta_x;
                let y = row_idx as i32 + delta_y;

                value <<= 1;

                if 0 <= x && x < cols as i32 && 0 <= y && y < rows as i32 {
                    value |= (grid[y as usize][x as usize] == '#') as usize;
                } else if enhancement[0] != '.' {
                    value |= (round % 2 == 1) as usize;
                }
            }

            next_grid[(row_idx + 1) as usize].push(enhancement[value]);
        }
    }

    next_grid
}

fn solve(input: &str, rounds: usize) -> usize {
    let enhancement: Vec<char> = input.lines().next().unwrap().chars().collect();
    let mut grid: Vec<Vec<char>> = input.lines().skip(2).map(|l| l.chars().collect()).collect();

    for i in 0..rounds {
        grid = apply(&grid, &enhancement, i);
    }

    grid.iter().flatten().filter(|&c| *c == '#').count()
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", solve(&input, 2));
    println!("Part 2: {}", solve(&input, 50));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn day20_part1() {
        assert_eq!(solve(INPUT, 2), 35);
        assert_eq!(solve(&fs::read_to_string(INPUT_PATH).unwrap(), 2), 5306);
    }

    #[test]
    fn day20_part2() {
        assert_eq!(solve(INPUT, 50), 3351);
        assert_eq!(solve(&fs::read_to_string(INPUT_PATH).unwrap(), 50), 17497);
    }
}
