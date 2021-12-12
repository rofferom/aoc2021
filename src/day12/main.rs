use std::collections::{HashMap, HashSet};
use std::fs;

const INPUT_PATH: &str = "src/day12/input.txt";

fn is_small_cave(s: &str) -> bool {
    s.chars().all(char::is_lowercase)
}

fn visit_node(
    graph: &HashMap<&str, Vec<&str>>,
    start_node: &str,
    mut hitmap: HashSet<String>,
    allow_double_visit: bool,
) -> u32 {
    if is_small_cave(start_node) {
        hitmap.insert(start_node.to_string());
    }

    let mut score = 0;

    for &edge in graph
        .get(start_node)
        .unwrap()
        .iter()
        .filter(|&&x| x != "start")
    {
        if edge == "end" {
            score += 1;
        } else if !is_small_cave(edge) || !&hitmap.contains(edge) {
            score += visit_node(graph, edge, hitmap.clone(), allow_double_visit);
        } else if allow_double_visit {
            score += visit_node(graph, edge, hitmap.clone(), false);
        }
    }

    score
}

fn solve(input: &str, allow_double_visit: bool) -> u32 {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for l in input.lines() {
        let l: Vec<_> = l.split('-').collect();
        let start = l[0];
        let end = l[1];

        let e = graph.entry(start).or_default();
        e.push(end);

        let e = graph.entry(end).or_default();
        e.push(start);
    }

    visit_node(&graph, "start", HashSet::new(), allow_double_visit)
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", solve(&input, false));
    println!("Part 2: {}", solve(&input, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &'static str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const INPUT2: &'static str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const INPUT3: &'static str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn day12_part1() {
        assert_eq!(solve(INPUT1, false), 10);
        assert_eq!(solve(INPUT2, false), 19);
        assert_eq!(solve(INPUT3, false), 226);
        assert_eq!(solve(&fs::read_to_string(INPUT_PATH).unwrap(), false), 3495);
    }

    #[test]
    fn day12_part2() {
        assert_eq!(solve(INPUT1, true), 36);
        assert_eq!(solve(INPUT2, true), 103);
        assert_eq!(solve(INPUT3, true), 3509);
        assert_eq!(solve(&fs::read_to_string(INPUT_PATH).unwrap(), true), 94849);
    }
}
