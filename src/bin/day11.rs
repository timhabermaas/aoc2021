use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/day11.txt").expect("file not found");

    let numbers: Vec<Vec<u8>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| format!("{}", c).parse::<u8>().expect("not a number"))
                .collect::<Vec<_>>()
        })
        .collect();

    let mut values = numbers.clone();

    let mut flashes = 0;

    for _ in 1..=100 {
        step(&mut values);
        flashes += flash(&mut values);
    }

    println!("Part 1: {}", flashes);

    let mut values = numbers.clone();

    let mut steps = 0;
    loop {
        step(&mut values);
        steps += 1;
        let flashes = flash(&mut values);
        if flashes == values.len() * values[0].len() {
            println!("Part 2: {}", steps);
            break;
        }
    }
}

fn step(v: &mut [Vec<u8>]) {
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            v[i][j] += 1;
        }
    }
}

fn flash(v: &mut [Vec<u8>]) -> usize {
    let mut queue = VecDeque::new();
    let mut flashed = HashSet::new();

    for i in 0..v.len() {
        for j in 0..v[i].len() {
            if v[i][j] > 9 {
                queue.push_back((i, j));
                flashed.insert((i, j));
            }
        }
    }

    while let Some(c) = queue.pop_front() {
        for n in neighbours(c.0, c.1, v.len(), v[0].len()) {
            v[n.0][n.1] += 1;
            if v[n.0][n.1] > 9 && !flashed.contains(&n) {
                queue.push_back(n);
                flashed.insert(n);
            }
        }
    }

    for c in &flashed {
        v[c.0][c.1] = 0;
    }
    flashed.len()
}

fn neighbours(i: usize, j: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
    let r = [
        (i.checked_sub(1), j.checked_sub(1)),
        (i.checked_sub(1), Some(j)),
        (i.checked_sub(1), add_one(j, width)),
        (Some(i), j.checked_sub(1)),
        (Some(i), add_one(j, width)),
        (add_one(i, height), j.checked_sub(1)),
        (add_one(i, height), Some(j)),
        (add_one(i, height), add_one(j, width)),
    ];

    r.into_iter()
        .filter_map(|v| v.0.and_then(|a| v.1.map(|b| (a, b))))
        .collect()
}

fn add_one(x: usize, max: usize) -> Option<usize> {
    (x < max - 1).then(|| x + 1)
}
