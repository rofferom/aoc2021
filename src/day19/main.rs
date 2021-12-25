use std::collections::{HashMap, HashSet};
use std::fs;

const INPUT_PATH: &str = "src/day19/input.txt";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn manhattan(&self, other: &Self) -> i32 {
        let d = self.sub(other);
        d.x.abs() + d.y.abs() + d.z.abs()
    }
}

#[derive(Clone, Debug)]
struct Scanner {
    points: Vec<Point>,
}

impl Scanner {
    fn new() -> Self {
        Self { points: vec![] }
    }

    fn rotate(&self, rot: &Matrix) -> Self {
        Self {
            points: self
                .points
                .iter()
                .map(|v| {
                    let p = Matrix::new_from_vector(vec![vec![v.x], vec![v.y], vec![v.z]]);

                    let result = rot.mult(&p);

                    Point {
                        x: result.values[0][0],
                        y: result.values[1][0],
                        z: result.values[2][0],
                    }
                })
                .collect(),
        }
    }

    fn remap_origin(&self, origin_idx: usize) -> HashSet<Point> {
        let mut out = HashSet::new();

        for i in 0..self.points.len() {
            if i == origin_idx {
                continue;
            }

            out.insert(self.points[i].sub(&self.points[origin_idx]));
        }

        out
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Matrix {
    values: Vec<Vec<i32>>,
    rows: usize,
    columns: usize,
}

impl Matrix {
    fn new_from_vector(values: Vec<Vec<i32>>) -> Self {
        Self {
            rows: values.len(),
            columns: values[0].len(),
            values,
        }
    }

    fn new(rows: usize, columns: usize) -> Self {
        Matrix {
            values: vec![vec![0; columns]; rows],
            rows,
            columns,
        }
    }

    fn mult(&self, other: &Self) -> Self {
        assert_eq!(self.columns, other.rows);

        let mut result = Self::new(self.rows, other.columns);

        for i in 0..self.rows {
            for j in 0..other.columns {
                for k in 0..self.columns {
                    result.values[i][j] += self.values[i][k] * other.values[k][j];
                }
            }
        }

        result
    }
}

fn get_all_rotations() -> Vec<Matrix> {
    let cos = |angle| match angle {
        0 => 1,
        90 => 0,
        180 => -1,
        270 => 0,
        _ => panic!("Unsupported angle {}", angle),
    };

    let sin = |angle| match angle {
        0 => 0,
        90 => 1,
        180 => 0,
        270 => -1,
        _ => panic!("Unsupported angle {}", angle),
    };

    let get_rot_x = |angle| {
        Matrix::new_from_vector(vec![
            vec![1, 0, 0],
            vec![0, cos(angle), -sin(angle)],
            vec![0, sin(angle), cos(angle)],
        ])
    };

    let get_rot_y = |angle| {
        Matrix::new_from_vector(vec![
            vec![cos(angle), 0, sin(angle)],
            vec![0, 1, 0],
            vec![-sin(angle), 0, cos(angle)],
        ])
    };

    let get_rot_z = |angle| {
        Matrix::new_from_vector(vec![
            vec![cos(angle), -sin(angle), 0],
            vec![sin(angle), cos(angle), 0],
            vec![0, 0, 1],
        ])
    };

    let mut rotations: HashSet<Matrix> = HashSet::new();

    // One axis
    for angle in [90, 180, 270] {
        rotations.insert(get_rot_x(angle));
        rotations.insert(get_rot_y(angle));
        rotations.insert(get_rot_z(angle));
    }

    // Two axis
    for angle_a in [0, 90, 180, 270] {
        for angle_b in [0, 90, 180, 270] {
            // x + y
            let rot_x = get_rot_x(angle_a);
            let rot_y = get_rot_y(angle_b);

            rotations.insert(rot_x.mult(&rot_y));
            rotations.insert(rot_y.mult(&rot_x));

            // x + z
            let rot_x = get_rot_x(angle_a);
            let rot_z = get_rot_z(angle_b);

            rotations.insert(rot_x.mult(&rot_z));
            rotations.insert(rot_z.mult(&rot_x));

            // y + z
            let rot_y = get_rot_y(angle_a);
            let rot_z = get_rot_z(angle_b);

            rotations.insert(rot_y.mult(&rot_z));
            rotations.insert(rot_z.mult(&rot_y));
        }
    }

    // Three axis
    for angle_x in [0, 90, 180, 270] {
        for angle_y in [0, 90, 180, 270] {
            for angle_z in [0, 90, 180, 270] {
                let rot_x = get_rot_x(angle_x);
                let rot_y = get_rot_y(angle_y);
                let rot_z = get_rot_z(angle_z);

                rotations.insert(rot_x.mult(&rot_y).mult(&rot_z));
                rotations.insert(rot_x.mult(&rot_z).mult(&rot_y));

                rotations.insert(rot_y.mult(&rot_x).mult(&rot_z));
                rotations.insert(rot_y.mult(&rot_z).mult(&rot_x));

                rotations.insert(rot_z.mult(&rot_x).mult(&rot_y));
                rotations.insert(rot_z.mult(&rot_y).mult(&rot_x));
            }
        }
    }

    rotations.iter().cloned().collect()
}

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut out = vec![];
    let mut scanner = Scanner::new();

    for l in input.lines() {
        if l.starts_with("---") {
            continue;
        }

        if l.is_empty() {
            out.push(scanner);
            scanner = Scanner::new();
        } else {
            let values: Vec<i32> = l.split(',').map(|x| x.parse().unwrap()).collect();

            scanner.points.push(Point {
                x: values[0],
                y: values[1],
                z: values[2],
            });
        }
    }

    out.push(scanner);

    out
}

// Try to check if scanner_b overlaps scanner_a after one specific rotation.
//
// At first, check if there is a couple of beacon in each scanner view that have the same relative
// position. In this case, we might have found an overlapping beacon.
//
// To confirm this, remap each scanner view from one of the beacons found previously. After that,
// check there are 12 common beacons.
fn scanners_match(
    scanner_a: &Scanner,
    scanner_b: &Scanner,
    rotations: &[Matrix],
) -> Option<(Scanner, Point)> {
    for rot in rotations {
        let rotated_b = scanner_b.rotate(rot);

        for a_i in 0..scanner_a.points.len() {
            for a_j in a_i + 1..scanner_a.points.len() {
                let point_a = scanner_a.points[a_i].sub(&scanner_a.points[a_j]);

                for b_i in 0..rotated_b.points.len() {
                    for b_j in b_i + 1..rotated_b.points.len() {
                        let point_b = rotated_b.points[b_i].sub(&rotated_b.points[b_j]);

                        // Maybe the beacons are overlapping with the current rotation.
                        // Check this is really the case
                        if point_a == point_b {
                            let a_map = scanner_a.remap_origin(a_i);
                            let b_map = rotated_b.remap_origin(b_i);

                            // Origins are not included in both remap. But we know they are already
                            // in the possible overlap
                            if a_map.intersection(&b_map).count() != 11 {
                                continue;
                            }

                            // Use both "origins" to find the Scanner B position
                            let scanner_b_pos = scanner_a.points[a_i].sub(&rotated_b.points[b_i]);

                            return Some((rotated_b, scanner_b_pos));
                        }
                    }
                }
            }
        }
    }

    None
}

fn find_beacons(input: &str) -> (Vec<Point>, HashMap<usize, Point>) {
    let rotations = get_all_rotations();
    assert_eq!(rotations.len(), 24);

    let mut scanners = parse_input(input);
    let mut scanner_abspos: HashMap<usize, Point> = HashMap::new();
    let mut missing_scanners: HashSet<usize> = (1..scanners.len()).collect();
    let mut to_visit: Vec<usize> = vec![0];

    let mut found_beacons: HashSet<Point> = scanners[0].points.iter().cloned().collect();

    while let Some(visiting_idx) = to_visit.pop() {
        let mut found_idx = HashSet::new();

        // Visit all scanners that have not known position
        for research_idx in &missing_scanners {
            if let Some((mut scanner_rotated, scanner_pos)) = scanners_match(
                &scanners[visiting_idx],
                &scanners[*research_idx],
                &rotations,
            ) {
                // At this point, we got the rotated scanner view, but we want to remap it from
                // the Scanner 0 point of view
                scanner_rotated.points = scanner_rotated
                    .points
                    .iter()
                    .map(|p| p.add(&scanner_pos))
                    .collect();

                // Now all beacons have the correct coordinates, store them
                for p in &scanner_rotated.points {
                    found_beacons.insert(*p);
                }

                // Update the scanner, it will be reused later: because its beacons are now in
                // Scanner 0 coordinates, any overlapping beacon could be projected to Scanner 0
                // coordinates directly.
                scanners[*research_idx] = scanner_rotated;

                // Update variables that track found stuff
                scanner_abspos.insert(*research_idx, scanner_pos);
                found_idx.insert(*research_idx);

                // Now this scanner is known, it can be used to search for other scanners
                to_visit.push(*research_idx);
            }
        }

        missing_scanners = missing_scanners.difference(&found_idx).copied().collect();
    }

    assert!(missing_scanners.is_empty());

    (found_beacons.iter().cloned().collect(), scanner_abspos)
}

fn part1(input: &str) -> usize {
    let (beacons, _) = find_beacons(input);
    beacons.len()
}

fn part2(input: &str) -> i32 {
    let (_, scanner_abspos) = find_beacons(input);
    let scanners: Vec<&Point> = scanner_abspos.values().collect();

    let mut max_dist = 0;

    for i in 0..scanners.len() {
        for j in i + 1..scanners.len() {
            max_dist = std::cmp::max(max_dist, scanners[i].manhattan(scanners[j]));
        }
    }

    max_dist
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day19_matrix() {
        // Test 1
        let a = Matrix::new_from_vector(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
        assert_eq!(a, a);

        let b = Matrix::new_from_vector(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert_eq!(b, b);

        assert_eq!(a.mult(&b), b);

        // Test 2
        let a = Matrix::new_from_vector(vec![vec![8, 9, 10], vec![11, 12, 13], vec![14, 15, 16]]);
        assert_eq!(a, a);

        let b = Matrix::new_from_vector(vec![vec![16, 15, 14], vec![13, 12, 11], vec![10, 9, 8]]);
        assert_eq!(b, b);

        let c = Matrix::new_from_vector(vec![
            vec![345, 318, 291],
            vec![462, 426, 390],
            vec![579, 534, 489],
        ]);

        assert_eq!(a.mult(&b), c);

        // Test 3
        let a = Matrix::new_from_vector(vec![vec![1, 0, 0], vec![0, 0, -1], vec![0, 1, 0]]);
        let b = Matrix::new_from_vector(vec![vec![1], vec![1], vec![1]]);
        let c = Matrix::new_from_vector(vec![vec![1], vec![-1], vec![1]]);

        assert_eq!(a.mult(&b), c);
    }

    #[test]
    fn day19_all_rotations() {
        let rotations = get_all_rotations();
        assert_eq!(rotations.len(), 24);
    }

    const INPUT: &'static str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn day19_part1() {
        assert_eq!(part1(INPUT), 79);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 381);
    }

    #[test]
    fn day19_part2() {
        assert_eq!(part2(INPUT), 3621);
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 12201);
    }
}
