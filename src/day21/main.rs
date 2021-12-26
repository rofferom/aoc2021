use std::cmp::min;
use std::collections::HashMap;
use std::fs;

const INPUT_PATH: &str = "src/day21/input.txt";

#[derive(Clone, Copy, Debug)]
struct Player {
    pos: u64,
    score: u64,
}

impl Player {
    fn new(pos: u64) -> Self {
        Self { pos, score: 0 }
    }
}

#[derive(Clone, Copy, Debug)]
struct Universe {
    players: [Player; 2],
    player_idx: usize,
    rolls: u64,
}

impl Universe {
    fn new(players: [Player; 2]) -> Self {
        Self {
            players,
            player_idx: 0,
            rolls: 1,
        }
    }
}

fn parse_players(input: &str) -> [Player; 2] {
    let lines: Vec<_> = input.lines().collect();

    [
        Player::new(lines[0][28..].parse::<u64>().unwrap() - 1),
        Player::new(lines[1][28..].parse::<u64>().unwrap() - 1),
    ]
}

pub fn part1(input: &str) -> u64 {
    let mut players = parse_players(input);

    let mut player_idx = 0;
    let mut die_rolled = 0;

    while players[0].score < 1000 && players[1].score < 1000 {
        let mut distance = 0;

        distance += die_rolled * 3 + 6;
        die_rolled += 3;

        let mut player = &mut players[player_idx];
        player.pos = (player.pos + distance) % 10;
        player.score += player.pos + 1;

        player_idx = (player_idx + 1) % players.len();
    }

    min(players[0].score, players[1].score) * die_rolled
}

fn compute_rolls() -> Vec<(u64, u64)> {
    let mut rolls = HashMap::new();

    for i in 1..4 {
        for j in 1..4 {
            for k in 1..4 {
                let r = i + j + k;
                let e = rolls.entry(r).or_insert(0);

                *e += 1;
            }
        }
    }

    let mut v: Vec<(u64, u64)> = rolls.iter().map(|(&k, &v)| (k, v)).collect();
    v.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    v
}

pub fn part2(input: &str) -> u64 {
    let rolls = compute_rolls();

    let mut universes = vec![Universe::new(parse_players(input))];
    let mut player_wins = vec![0; 2];

    while let Some(universe) = universes.pop() {
        for (distance, count) in &rolls {
            let mut new_universe = universe;
            new_universe.rolls *= count;

            let player = &mut new_universe.players[universe.player_idx];
            player.pos = (player.pos + distance) % 10;
            player.score += player.pos + 1;

            if player.score >= 21 {
                player_wins[universe.player_idx] += new_universe.rolls;
            } else {
                new_universe.player_idx = (new_universe.player_idx + 1) % 2;
                universes.push(new_universe);
            }
        }
    }

    *player_wins.iter().max().unwrap()
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
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

    #[test]
    fn day21_part2() {
        assert_eq!(part2(INPUT), 444356092776315);
        assert_eq!(
            part2(&fs::read_to_string(INPUT_PATH).unwrap()),
            27464148626406
        );
    }
}
