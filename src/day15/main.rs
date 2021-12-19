use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

const INPUT_PATH: &str = "src/day15/input.txt";

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: u32,
    position: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(input: &str, tile_count: usize) -> u32 {
    // Parse as a two-dimension array at first to get the dimensions
    let graph: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    let tile_columns = graph[0].len();
    let tile_rows = graph.len();

    let columns = tile_columns * tile_count;
    let rows = tile_rows * tile_count;
    let nodes_count = columns * rows;

    // Flatten the graph to use a one-dimension array
    let graph: Vec<u32> = graph.iter().flatten().copied().collect();

    // Helper to get a specific position
    let get_position = |pos: usize, delta_x: i32, delta_y: i32| {
        let x = (pos % columns) as i32;
        let y = (pos / columns) as i32;

        if x + delta_x < 0 || x + delta_x >= columns as i32 {
            return None;
        }

        if y + delta_y < 0 || y + delta_y >= rows as i32 {
            return None;
        }

        Some((y + delta_y) as usize * columns + (x + delta_x) as usize)
    };

    let mut dist = vec![u32::MAX; nodes_count];
    dist[0] = 0;

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    heap.push(Node {
        cost: 0,
        position: 0,
    });

    while let Some(node) = heap.pop() {
        if node.position == nodes_count - 1 {
            return node.cost;
        }

        if node.cost > dist[node.position] {
            continue;
        }

        let moves = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (delta_x, delta_y) in moves.iter() {
            if let Some(target) = get_position(node.position, *delta_x, *delta_y) {
                // Remap in x,y coordinates.
                let x = target % columns;
                let y = target / rows;

                // Compute edge cost. It depends of the tile index.
                let x_tile_idx = (x / tile_columns) as u32;
                let y_tile_idx = (y / tile_rows) as u32;
                let tile_target = (y % tile_rows) * tile_columns + x % tile_columns;

                let edge_cost = ((graph[tile_target] + (x_tile_idx + y_tile_idx)) - 1) % 9 + 1;

                // Update costs if required
                let cost = node.cost + edge_cost;
                if cost < dist[target] {
                    heap.push(Node {
                        position: target,
                        cost,
                    });

                    dist[target] = cost;
                }
            }
        }
    }

    0
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", solve(&input, 1));
    println!("Part 2: {}", solve(&input, 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn day15_part1() {
        assert_eq!(solve(INPUT, 1), 40);
        assert_eq!(solve(&fs::read_to_string(INPUT_PATH).unwrap(), 1), 429);
    }

    #[test]
    fn day15_part2() {
        assert_eq!(solve(INPUT, 5), 315);
        assert_eq!(solve(&fs::read_to_string(INPUT_PATH).unwrap(), 5), 2844);
    }
}
