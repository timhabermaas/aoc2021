use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::fs::read_to_string;
use std::rc::Rc;

use itertools::Itertools;

#[derive(Clone, Debug)]
enum SnailfishNumber {
    Pair(Rc<RefCell<SnailfishNumber>>, Rc<RefCell<SnailfishNumber>>),
    Value(u32),
}

impl SnailfishNumber {
    fn deep_clone(&self) -> Rc<RefCell<SnailfishNumber>> {
        match self {
            Self::Pair(l, r) => Rc::new(RefCell::new(Self::Pair(
                l.borrow().deep_clone(),
                r.borrow().deep_clone(),
            ))),
            Self::Value(n) => Rc::new(RefCell::new(Self::Value(*n))),
        }
    }
}

impl fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Pair(a, b) => {
                write!(f, "[{},{}]", a.borrow(), b.borrow())
            }
            Self::Value(x) => {
                write!(f, "{}", x)
            }
        }
    }
}

impl SnailfishNumber {
    fn get_value(&self) -> Option<u32> {
        match self {
            Self::Value(x) => Some(*x),
            _ => None,
        }
    }

    fn is_value(&self) -> bool {
        self.get_value().is_some()
    }

    fn values(&self) -> (u32, u32) {
        match self {
            Self::Pair(l, r) => (
                l.borrow().get_value().unwrap(),
                r.borrow().get_value().unwrap(),
            ),
            _ => panic!("number isn't pair of numbers"),
        }
    }
}

fn parse_snailfish_number(part: &str) -> (SnailfishNumber, &str) {
    match part.chars().nth(0).unwrap() {
        '[' => {
            let (first_part, rest) = parse_snailfish_number(&part[1..]);
            if rest.chars().nth(0).unwrap() != ',' {
                panic!("expected ',', found '{}'", rest.chars().nth(0).unwrap());
            }
            let (second_part, rest) = parse_snailfish_number(&rest[1..]);
            if rest.chars().nth(0).unwrap() != ']' {
                panic!("expected ']', found '{}'", rest.chars().nth(0).unwrap());
            }
            return (
                SnailfishNumber::Pair(
                    Rc::new(RefCell::new(first_part)),
                    Rc::new(RefCell::new(second_part)),
                ),
                &rest[1..],
            );
        }
        n @ '0'..='9' => {
            return (
                SnailfishNumber::Value(n.to_string().parse::<u32>().unwrap()),
                &part[1..],
            );
        }
        c => {
            panic!("didn't expect '{}'", c)
        }
    }
}

fn parse_line(line: &str) -> Rc<RefCell<SnailfishNumber>> {
    let (result, _rest) = parse_snailfish_number(line);
    Rc::new(RefCell::new(result))
}

fn try_explode(node: Rc<RefCell<SnailfishNumber>>) -> bool {
    use SnailfishNumber::*;

    let mut nodes: VecDeque<(Rc<RefCell<SnailfishNumber>>, usize)> = VecDeque::new();
    let mut left_most_value: Option<Rc<RefCell<SnailfishNumber>>> = None;
    let mut right_most_value: Option<Rc<RefCell<SnailfishNumber>>> = None;
    let mut exploding_pair: Option<Rc<RefCell<SnailfishNumber>>> = None;

    nodes.push_front((node, 0));

    while let Some(node) = nodes.pop_front() {
        //println!("visiting {:?}", node);
        let (n, depth) = node;

        if exploding_pair.is_none() {
            match &*n.borrow() {
                Pair(l, r) => {
                    if l.borrow().is_value() && r.borrow().is_value() && depth == 4 {
                        exploding_pair = Some(n.clone());
                    } else {
                        nodes.push_front((r.clone(), depth + 1));
                        nodes.push_front((l.clone(), depth + 1));
                    }
                }
                Value(_) => {
                    left_most_value = Some(n.clone());
                }
            };
        } else {
            match &*n.borrow() {
                Pair(l, r) => {
                    nodes.push_front((r.clone(), depth + 1));
                    nodes.push_front((l.clone(), depth + 1));
                }
                Value(_) => {
                    right_most_value = Some(n.clone());
                    break;
                }
            };
        }
    }

    match exploding_pair {
        Some(p) => {
            let (l, r) = p.borrow().values();

            left_most_value.map(|node| add_to_value(node, l));
            right_most_value.map(|node| add_to_value(node, r));
            *p.borrow_mut() = Value(0);
            true
        }
        _ => false,
    }
}

fn try_split(node: Rc<RefCell<SnailfishNumber>>) -> bool {
    use SnailfishNumber::*;

    let mut found_split = false;
    let mut nodes: VecDeque<Rc<RefCell<SnailfishNumber>>> = VecDeque::new();

    nodes.push_front(node);

    while let Some(node) = nodes.pop_front() {
        let new_pair = match &*node.borrow() {
            Pair(l, r) => {
                nodes.push_front(r.clone());
                nodes.push_front(l.clone());
                None
            }
            Value(x) if *x >= 10 => {
                let l = x / 2;
                let r = x - l;
                Some(Pair(
                    Rc::new(RefCell::new(Value(l))),
                    Rc::new(RefCell::new(Value(r))),
                ))
            }
            _ => None,
        };

        found_split = new_pair.is_some();

        new_pair.map(|p| *node.borrow_mut() = p);

        if found_split {
            break;
        }
    }

    found_split
}

fn add_to_value(node: Rc<RefCell<SnailfishNumber>>, number: u32) {
    let mut node = node.borrow_mut();

    match &mut *node {
        SnailfishNumber::Value(ref mut x) => {
            *x += number;
        }
        _ => {}
    }
}

fn magnitude(node: Rc<RefCell<SnailfishNumber>>) -> u32 {
    match &*node.borrow() {
        SnailfishNumber::Pair(left, right) => {
            3 * magnitude(left.clone()) + 2 * magnitude(right.clone())
        }
        SnailfishNumber::Value(n) => *n,
    }
}

fn add(
    a: Rc<RefCell<SnailfishNumber>>,
    b: Rc<RefCell<SnailfishNumber>>,
) -> Rc<RefCell<SnailfishNumber>> {
    let a = a.borrow().deep_clone();
    let b = b.borrow().deep_clone();

    let result = Rc::new(RefCell::new(SnailfishNumber::Pair(a, b)));

    loop {
        if try_explode(result.clone()) {
            continue;
        }
        if try_split(result.clone()) {
            continue;
        } else {
            break;
        }
    }

    result
}

fn main() {
    let input = read_to_string("inputs/day18.txt").expect("file not found");

    let values: Vec<Rc<RefCell<SnailfishNumber>>> = input.lines().map(|l| parse_line(l)).collect();

    let result = magnitude(
        values[1..]
            .iter()
            .fold(values[0].clone(), |sum, x| add(sum, x.clone())),
    );

    println!("Part 1: {}", result);

    let result = (0..values.len())
        .cartesian_product(0..values.len())
        .filter(|(i, j)| i != j)
        .map(|(i, j)| magnitude(add(values[i].clone(), values[j].clone())))
        .max();

    println!("Part 2: {}", result.unwrap());
}
