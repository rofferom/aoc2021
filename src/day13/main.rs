use std::collections::HashSet;
use std::fs;

const INPUT_PATH: &str = "src/day13/input.txt";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Dot {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Fold {
    axis: char,
    value: u32,
}

fn print_dots(dots: &HashSet<Dot>) {
    let rows = dots.iter().map(|dot| dot.y).max().unwrap() + 1;
    let columns = dots.iter().map(|dot| dot.x).max().unwrap() + 1;

    let mut grid = vec![vec![]; rows as usize];

    for i in 0..rows {
        grid[i as usize] = vec!['.'; columns as usize];
    }

    for p in dots {
        grid[p.y as usize][p.x as usize] = '#';
    }

    for row in grid {
        for cell in row {
            print!("{}", cell);
        }

        println!();
    }
}

fn parse_input(input: &str) -> (HashSet<Dot>, Vec<Fold>) {
    let input_parts: Vec<_> = input.split("\n\n").collect();
    let str_dots = input_parts[0];
    let str_folds = input_parts[1];

    let mut dots: HashSet<Dot> = HashSet::new();
    let mut folds: Vec<Fold> = vec![];

    // Parse dots
    for dot in str_dots.lines() {
        let coords: Vec<u32> = dot.split(',').map(|x| x.parse().unwrap()).collect();

        dots.insert(Dot {
            x: coords[0],
            y: coords[1],
        });
    }

    // Parse folds
    for fold in str_folds.lines() {
        let t: Vec<_> = fold[11..].split('=').collect();
        folds.push(Fold {
            axis: t[0].chars().next().unwrap(),
            value: t[1].parse().unwrap(),
        });
    }

    (dots, folds)
}

fn do_fold(mut dots: HashSet<Dot>, fold: &Fold) -> HashSet<Dot> {
    type FilterCb = Box<dyn Fn(&Dot, u32) -> bool>;
    type MapDotCb = Box<dyn Fn(&Dot, u32) -> Dot>;

    let (filter_cb, map_dot_cb) = if fold.axis == 'y' {
        let filter_cb: FilterCb = Box::new(|dot: &Dot, fold_value: u32| dot.y >= fold_value);

        let map_dot_cb: MapDotCb = Box::new(|dot: &Dot, fold_value: u32| Dot {
            x: dot.x,
            y: fold_value - (dot.y - fold_value),
        });

        (filter_cb, map_dot_cb)
    } else {
        let filter_cb: FilterCb = Box::new(|dot: &Dot, fold_value: u32| dot.x >= fold_value);

        let map_dot_cb: MapDotCb = Box::new(|dot: &Dot, fold_value: u32| Dot {
            x: fold_value - (dot.x - fold_value),
            y: dot.y,
        });

        (filter_cb, map_dot_cb)
    };

    let to_move: HashSet<_> = dots
        .iter()
        .filter(|dot| filter_cb(dot, fold.value))
        .copied()
        .collect();

    dots = dots.difference(&to_move).copied().collect();

    for dot in &to_move {
        let dest = map_dot_cb(dot, fold.value);

        dots.insert(dest);
    }

    dots
}

fn part1(input: &str) -> usize {
    let (mut dots, folds) = parse_input(input);

    dots = do_fold(dots, &folds[0]);
    dots.len()
}

fn part2(input: &str) {
    let (mut dots, folds) = parse_input(input);

    for fold in &folds {
        dots = do_fold(dots, fold);
    }

    print_dots(&dots);
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    part2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn day13_part1() {
        assert_eq!(part1(INPUT), 17);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 712);
    }
}
