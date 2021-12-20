use std::cell::RefCell;
use std::cmp::max;
use std::fs;
use std::rc::Rc;

const INPUT_PATH: &str = "src/day18/input.txt";

#[derive(Debug)]
struct Edges {
    left: Rc<RefCell<Node>>,
    right: Rc<RefCell<Node>>,
}

#[derive(Debug)]
enum Node {
    Number(u32),
    Edges(Edges),
}

impl Node {
    fn parse_input(input: &str) -> (Rc<RefCell<Node>>, usize) {
        assert_eq!(input.chars().next().unwrap(), '[');
        assert_eq!(input.chars().last().unwrap(), ']');

        // Parse left
        let left = &input[1..];
        let first_char = left.chars().next().unwrap();

        let (separator_idx, left_node) = if first_char == '[' {
            let (subnode, subnode_len) = Self::parse_input(left);
            (1 + subnode_len + 1, subnode)
        } else {
            assert!(first_char.is_digit(10));

            let separator_idx = left.find(',').unwrap();
            let value = left[0..separator_idx].parse().unwrap();

            (
                1 + separator_idx,
                Rc::new(RefCell::new(Node::Number(value))),
            )
        };

        // Parse right
        let right_idx = separator_idx + 1;
        let right = &input[right_idx..];

        let first_char = right.chars().next().unwrap();

        let (end_idx, right_node) = if first_char == '[' {
            let (subnode, subnode_len) = Self::parse_input(right);
            (right_idx + subnode_len + 1, subnode)
        } else {
            assert!(first_char.is_digit(10));

            let end_idx = right.find(']').unwrap();
            let value = right[0..end_idx].parse().unwrap();

            (
                right_idx + end_idx,
                Rc::new(RefCell::new(Node::Number(value))),
            )
        };

        (
            Rc::new(RefCell::new(Node::Edges(Edges {
                left: left_node,
                right: right_node,
            }))),
            end_idx,
        )
    }

    fn new(input: &str) -> Rc<RefCell<Self>> {
        let (node, _) = Self::parse_input(input);
        node
    }

    fn to_str(&self) -> String {
        match self {
            Node::Number(n) => format!("{}", n),
            Node::Edges(edges) => {
                format!(
                    "[{},{}]",
                    edges.left.borrow().to_str(),
                    edges.right.borrow().to_str()
                )
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Node::Number(n) => *n,
            Node::Edges(edges) => {
                let left = edges.left.borrow().magnitude();
                let right = edges.right.borrow().magnitude();

                left * 3 + right * 2
            }
        }
    }
}

struct NodeVisitor {
    node: Rc<RefCell<Node>>,
    depth: u32,
}

fn explode(node: Rc<RefCell<Node>>) -> bool {
    let mut stack = if let Node::Edges(node) = &*node.borrow() {
        vec![
            NodeVisitor {
                node: node.right.clone(),
                depth: 1,
            },
            NodeVisitor {
                node: node.left.clone(),
                depth: 1,
            },
        ]
    } else {
        panic!("Unexpected root node type");
    };

    let mut previous_value: Option<Rc<RefCell<Node>>> = None;
    let mut found_pair: Option<Rc<RefCell<Node>>> = None;
    let mut next_value: Option<Rc<RefCell<Node>>> = None;

    while let Some(visitor) = stack.pop() {
        match &*visitor.node.borrow() {
            Node::Number(_) => {
                if found_pair.is_some() {
                    next_value = Some(visitor.node.clone());
                    break;
                } else {
                    previous_value = Some(visitor.node.clone());
                }
            }
            Node::Edges(node) => {
                if found_pair.is_none() && visitor.depth == 4 {
                    found_pair = Some(visitor.node.clone());
                } else {
                    stack.push(NodeVisitor {
                        node: node.right.clone(),
                        depth: visitor.depth + 1,
                    });

                    stack.push(NodeVisitor {
                        node: node.left.clone(),
                        depth: visitor.depth + 1,
                    });
                }
            }
        }
    }

    if let Some(node) = found_pair {
        // Get left and right
        let (left, right) = if let Node::Edges(node) = &*node.borrow() {
            let left = if let Node::Number(n) = *node.left.borrow() {
                n
            } else {
                panic!("Failed to unpack found pair");
            };

            let right = if let Node::Number(n) = *node.right.borrow() {
                n
            } else {
                panic!("Failed to unpack found pair");
            };

            (left, right)
        } else {
            panic!("Failed to unpack found pair");
        };

        // Update nodes
        *node.borrow_mut() = Node::Number(0);

        if let Some(previous_value) = previous_value {
            if let Node::Number(n) = &mut *previous_value.borrow_mut() {
                *n += left;
            }
        }

        if let Some(next_value) = next_value {
            if let Node::Number(n) = &mut *next_value.borrow_mut() {
                *n += right;
            }
        }

        true
    } else {
        false
    }
}

fn split(node: Rc<RefCell<Node>>) -> bool {
    let mut stack = if let Node::Edges(node) = &*node.borrow() {
        vec![
            NodeVisitor {
                node: node.right.clone(),
                depth: 1,
            },
            NodeVisitor {
                node: node.left.clone(),
                depth: 1,
            },
        ]
    } else {
        panic!("Unexpected root node type");
    };

    while let Some(visitor) = stack.pop() {
        let node = &mut *visitor.node.borrow_mut();

        match node {
            Node::Number(n) => {
                if *n >= 10 {
                    let left = *n / 2;
                    let right = *n - left;

                    *node = Node::Edges(Edges {
                        left: Rc::new(RefCell::new(Node::Number(left))),
                        right: Rc::new(RefCell::new(Node::Number(right))),
                    });

                    return true;
                }
            }
            Node::Edges(node) => {
                stack.push(NodeVisitor {
                    node: node.right.clone(),
                    depth: visitor.depth + 1,
                });

                stack.push(NodeVisitor {
                    node: node.left.clone(),
                    depth: visitor.depth + 1,
                });
            }
        }
    }

    false
}

fn reduce(node: Rc<RefCell<Node>>) {
    while explode(node.clone()) || split(node.clone()) {}
}

fn run(input: &str) -> (u32, String) {
    let lines: Vec<_> = input.lines().collect();

    let mut node = Node::new(lines[0]);

    for l in lines.iter().skip(1) {
        node = Rc::new(RefCell::new(Node::Edges(Edges {
            left: node,
            right: Node::new(l),
        })));

        reduce(node.clone());
    }

    let node = &node.borrow();
    (node.magnitude(), node.to_str())
}

fn part1(input: &str) -> u32 {
    let (magnitude, _) = run(input);
    magnitude
}

fn part2(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();

    let mut magnitude = u32::MIN;

    let get_magnitude = |a: &str, b: &str| {
        let node = Rc::new(RefCell::new(Node::Edges(Edges {
            left: Node::new(a),
            right: Node::new(b),
        })));

        reduce(node.clone());

        let node = node.borrow();
        node.magnitude()
    };

    for i in 0..lines.len() - 1 {
        for j in i + 1..lines.len() {
            magnitude = max(magnitude, get_magnitude(lines[i], lines[j]));
            magnitude = max(magnitude, get_magnitude(lines[j], lines[i]));
        }
    }

    magnitude
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse_print(input: &str) {
        let node = Node::new(input);
        let output = node.borrow().to_str();
        assert_eq!(input, output);
    }

    fn test_magnitude(input: &str, value: u32) {
        let node = Node::new(input);
        assert_eq!(node.borrow().magnitude(), value);
    }

    fn test_explode(input: &str, output: &str) {
        let node = Node::new(input);
        assert!(explode(node.clone()));
        assert_eq!(node.borrow().to_str(), output);
    }

    fn test_split(input: &str, output: &str) {
        let node = Node::new(input);
        assert!(split(node.clone()));
        assert_eq!(node.borrow().to_str(), output);
    }

    fn test_reduce(input: &str, output: &str) {
        let node = Node::new(input);
        reduce(node.clone());
        assert_eq!(node.borrow().to_str(), output);
    }

    #[test]
    fn day18_part1() {
        test_parse_print("[1,2]");
        test_parse_print("[[1,2],3]");
        test_parse_print("[9,[8,7]]");
        test_parse_print("[[1,9],[8,5]]");
        test_parse_print("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
        test_parse_print("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
        test_parse_print("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");

        test_magnitude("[[1,2],[[3,4],5]]", 143);
        test_magnitude("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384);
        test_magnitude("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
        test_magnitude("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
        test_magnitude("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137);
        test_magnitude(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            3488,
        );
        test_magnitude(
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
            4140,
        );

        test_explode("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        test_explode("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        test_explode("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        test_explode(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        test_explode(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );

        test_split(
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        );
        test_split(
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
        );

        test_reduce(
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        );
        test_reduce(
            "[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]",
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        );

        const INPUT: &'static str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

        let (v, s) = run(INPUT);
        assert_eq!(
            s,
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
        );
        assert_eq!(v, 4140);

        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 4365);
    }

    #[test]
    fn day18_part2() {
        const INPUT: &'static str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

        assert_eq!(part2(INPUT), 3993);
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 4490);
    }
}
