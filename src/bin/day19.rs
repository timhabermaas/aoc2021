use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

impl Beacon {
    fn rotation(&self, index: usize) -> Beacon {
        match index {
            0 => Beacon {
                x: self.x,
                y: self.y,
                z: self.z,
            },
            // rotate around z
            1 => Beacon {
                x: self.y,
                y: -self.x,
                z: self.z,
            },
            2 => Beacon {
                x: -self.x,
                y: -self.y,
                z: self.z,
            },
            3 => Beacon {
                x: -self.y,
                y: self.x,
                z: self.z,
            },
            // rotate around x
            4 => Beacon {
                x: self.x,
                y: -self.z,
                z: self.y,
            },
            5 => Beacon {
                x: self.x,
                y: -self.y,
                z: -self.z,
            },
            6 => Beacon {
                x: self.x,
                y: self.z,
                z: -self.y,
            },
            // rotate around y
            7 => Beacon {
                x: self.z,
                y: self.y,
                z: -self.x,
            },
            8 => Beacon {
                x: -self.x,
                y: self.y,
                z: -self.z,
            },
            9 => Beacon {
                x: -self.z,
                y: self.y,
                z: self.x,
            },
            // rotate around BFL/UBR
            10 => Beacon {
                x: -self.z,
                y: -self.x,
                z: self.y,
            },
            11 => Beacon {
                x: -self.y,
                y: self.z,
                z: -self.x,
            },
            // rotate around UFL/DBR
            12 => Beacon {
                x: self.y,
                y: self.z,
                z: self.x,
            },
            13 => Beacon {
                x: self.z,
                y: self.x,
                z: self.y,
            },
            // rotate around UBL/DFR
            14 => Beacon {
                x: -self.y,
                y: -self.z,
                z: self.x,
            },
            15 => Beacon {
                x: self.z,
                y: -self.x,
                z: -self.y,
            },
            // rotate around UFR/DBL
            16 => Beacon {
                x: -self.z,
                y: self.x,
                z: -self.y,
            },
            17 => Beacon {
                x: self.y,
                y: -self.z,
                z: -self.x,
            },
            // rotate around UF/DB
            18 => Beacon {
                x: self.y,
                y: self.x,
                z: -self.z,
            },
            // rotate around UB/DF
            19 => Beacon {
                x: -self.y,
                y: -self.x,
                z: -self.z,
            },
            // rotate around UL/DR
            20 => Beacon {
                x: self.z,
                y: -self.y,
                z: self.x,
            },
            // rotate around UR/DL
            21 => Beacon {
                x: -self.z,
                y: -self.y,
                z: -self.x,
            },
            // rotate around FR/BL
            22 => Beacon {
                x: -self.x,
                y: -self.z,
                z: -self.y,
            },
            // rotate around FL/BR
            23 => Beacon {
                x: -self.x,
                y: self.z,
                z: self.y,
            },
            _ => panic!("index out of range"),
        }
    }

    fn add(&self, other: &Beacon) -> Beacon {
        Beacon {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn sub(&self, other: &Beacon) -> Beacon {
        Beacon {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn negate(&self) -> Beacon {
        Beacon {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    fn distance(&self, other: &Beacon) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

#[derive(Debug)]
struct Scanner(Vec<Beacon>);

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut result = vec![];
    for block in input.split("\n\n") {
        let mut beacons = vec![];
        for line in block.lines().skip(1) {
            let coords = line
                .split(",")
                .map(|c| c.parse::<i32>().unwrap())
                .collect_vec();

            beacons.push(Beacon {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            });
        }

        result.push(Scanner(beacons));
    }

    result
}

fn pairs_match(pairs: &[(Beacon, Beacon)]) -> bool {
    if pairs.is_empty() {
        return true;
    }

    let assumed_pos = pairs[0].0.sub(&pairs[0].1);

    pairs[1..].iter().all(|(a, b)| a.sub(&b) == assumed_pos)
}

fn solve(
    beacon_1s: &mut Vec<Beacon>,
    beacon_2s: &mut Vec<Beacon>,
    matching_pairs: &mut Vec<(Beacon, Beacon)>,
    rotation: usize,
) -> Option<Beacon> {
    if !pairs_match(&matching_pairs) {
        return None;
    }
    if pairs_match(&matching_pairs) && matching_pairs.len() >= 6 {
        let (a, b) = matching_pairs[0];
        return Some(b.sub(&a));
    }

    for i in 0..beacon_1s.len() {
        for j in 0..beacon_2s.len() {
            let c1 = beacon_1s.remove(i);
            let c2 = beacon_2s.remove(j);
            let p = (c1, c2);
            matching_pairs.push(p);

            if let Some(pos) = solve(beacon_1s, beacon_2s, matching_pairs, rotation) {
                return Some(pos);
            }

            matching_pairs.pop();
            beacon_1s.push(c1);
            beacon_2s.push(c2);
        }
    }

    None
}

fn main() {
    let input = read_to_string("inputs/day19.txt").expect("file not found");

    let scanners = parse_input(&input);

    let mut known: HashMap<usize, (Beacon, usize)> = HashMap::new();
    let mut unknown: HashSet<usize> = HashSet::new();
    known.insert(0, (Beacon { x: 0, y: 0, z: 0 }, 0));
    for i in 1..scanners.len() {
        unknown.insert(i);
    }

    loop {
        let old_unkown = unknown.clone();
        for index_unknown in old_unkown {
            let old_known = known.clone();
            'outer: for (index_known, (reference_pos, reference_rotation)) in old_known {
                for i in 0..24 {
                    let mut pairs = vec![];
                    let mut base = scanners[index_known]
                        .0
                        .iter()
                        .map(|b| b.rotation(reference_rotation))
                        .collect();
                    let mut search = scanners[index_unknown]
                        .0
                        .iter()
                        .map(|b| b.rotation(i))
                        .collect();
                    if let Some(pos) = solve(&mut base, &mut search, &mut pairs, i) {
                        known.insert(index_unknown, (pos.sub(&reference_pos).negate(), i));
                        unknown.remove(&index_unknown);

                        break 'outer;
                    }
                }
            }
        }

        if known.len() == scanners.len() {
            break;
        }
    }

    let mut all_beacons: HashSet<Beacon> = HashSet::new();
    for (index, s) in scanners.iter().enumerate() {
        let (scanner_pos, scanner_rotation) = known.get(&index).unwrap();
        for b in &s.0 {
            let foo = b.rotation(*scanner_rotation).add(scanner_pos);
            all_beacons.insert(foo);
        }
    }

    println!("Part 1: {:?}", all_beacons.len());

    let part_2 = (0..scanners.len())
        .cartesian_product(0..scanners.len())
        .filter(|(a, b)| a != b)
        .map(|(a, b)| known[&a].0.distance(&known[&b].0))
        .max();

    println!("Part 2: {:?}", part_2);
}
