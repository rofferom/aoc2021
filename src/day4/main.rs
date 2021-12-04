use std::fs;

use colored::*;

const GRID_SIZE: u32 = 5;

#[derive(Clone, Debug)]
struct GridItem {
    value: u32,
    marked: bool,
}

#[derive(Clone, Debug)]
struct Grid {
    items: Vec<GridItem>,
    last_marked: Option<u32>,
}

impl Grid {
    fn mark(&mut self, b: u32) {
        let it = self.items.iter_mut().find(|x| x.value == b);
        if let Some(mut item) = it {
            item.marked = true;
            self.last_marked = Some(b);
        }
    }

    fn wins(&self) -> (bool, u32) {
        for i in 0..GRID_SIZE {
            let mut line_completed = true;
            let mut row_completed = true;

            for j in 0..GRID_SIZE {
                if !self.items[(i * GRID_SIZE + j) as usize].marked {
                    line_completed = false;
                }

                if !self.items[(j * GRID_SIZE + i) as usize].marked {
                    row_completed = false;
                }
            }

            if line_completed || row_completed {
                return (true, self.score());
            }
        }

        (false, 0)
    }

    fn score(&self) -> u32 {
        self.items
            .iter()
            .filter(|x| !x.marked)
            .map(|x| x.value)
            .sum()
    }

    fn last_marked(&self) -> Option<u32> {
        self.last_marked
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let item = &self.items[(row * GRID_SIZE + col) as usize];

                if item.marked {
                    write!(f, "{} ", format!("{:2}", item.value).green()).unwrap();
                } else {
                    write!(f, "{:2} ", item.value).unwrap();
                }
            }

            if row != GRID_SIZE - 1 {
                writeln!(f, "").unwrap();
            }
        }

        Ok(())
    }
}

fn str_it_to_u32<'a, T: Iterator<Item = &'a str>>(it: T) -> Vec<u32> {
    it.map(|v| v.parse::<u32>().unwrap()).collect()
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Grid>) {
    let mut lines = input.lines();

    // Get numbers
    let numbers: Vec<u32> = str_it_to_u32(lines.next().unwrap().split(','));
    lines.next();

    // Get grids
    let mut grids: Vec<Grid> = vec![];

    loop {
        // Parse grid
        let mut grid: Vec<u32> = vec![];

        for l in (&mut lines).take(GRID_SIZE as usize) {
            let mut parsed_lined = str_it_to_u32(l.split_whitespace());
            grid.append(&mut parsed_lined);
        }

        if grid.is_empty() {
            break;
        }

        lines.next();

        // Craft final grid
        grids.push(Grid {
            items: grid
                .iter()
                .map(|v| GridItem {
                    value: *v,
                    marked: false,
                })
                .collect(),
            last_marked: None,
        });
    }

    (numbers, grids)
}

fn part1(input: &str) -> u32 {
    let (numbers, mut grids) = parse_input(input);

    for n in numbers {
        println!("Got number {}", n);

        for grid in &mut grids {
            grid.mark(n);

            let (wins, score) = grid.wins();
            if wins {
                println!("{}", grid);
                return score * n;
            }
        }
    }

    0
}

fn part2(input: &str) -> u32 {
    let (numbers, mut grids) = parse_input(input);
    let mut winner: Option<Grid> = None;

    for n in numbers {
        let mut winners_idx = vec![];

        for (idx, grid) in grids.iter_mut().enumerate() {
            grid.mark(n);

            let (wins, _score) = grid.wins();
            if wins {
                winner = Some(grid.clone());
                winners_idx.push(idx);
            }
        }

        for idx in winners_idx.iter().rev() {
            grids.remove(*idx);
        }
    }

    if let Some(winner) = winner {
        println!("Winner:");
        println!("{}", winner);
        return winner.score() * winner.last_marked().unwrap();
    }

    0
}

fn main() {
    let input = fs::read_to_string("src/day4/input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
    8   2 23  4 24
    21  9 14 16  7
    6  10  3 18  5
    1  12 20 15 19

    3  15  0  2 22
    9  18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
    2   0 12  3  7";

    #[test]
    fn day4_part1() {
        assert_eq!(part1(INPUT), 4512);
    }

    #[test]
    fn day4_part2() {
        assert_eq!(part2(INPUT), 1924);
    }
}
