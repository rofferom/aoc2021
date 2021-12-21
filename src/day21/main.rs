use std::cmp::min;
use std::fs;

const INPUT_PATH: &str = "src/day21/input.txt";

#[derive(Debug)]
struct Player {
    pos: usize,
    score: usize,
}

fn part1(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();

    let mut players = vec![
        Player {
            pos: lines[0][28..].parse::<usize>().unwrap() - 1,
            score: 0,
        },
        Player {
            pos: lines[1][28..].parse::<usize>().unwrap() - 1,
            score: 0,
        },
    ];

    let mut player_idx = 0;
    let mut die_rolled = 0;

    while players[0].score < 1000 && players[1].score < 1000 {
        let mut distance = 0;

        for _ in 0..3 {
            die_rolled += 1;
            distance += die_rolled;
        }

        let mut player = &mut players[player_idx];

        player.pos = (player.pos + distance) % 10;
        player.score += player.pos + 1;

        player_idx = (player_idx + 1) % players.len();
    }

    min(players[0].score, players[1].score) * die_rolled
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn day21_part1() {
        assert_eq!(part1(INPUT), 739785);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 797160);
    }
}
