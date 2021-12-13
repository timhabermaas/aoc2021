use std::collections::HashMap;
use std::fs::read_to_string;

type Connections<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_line(line: &str) -> (&str, &str) {
    let mut parts = line.split("-");
    (parts.next().unwrap(), parts.next().unwrap())
}

fn main() {
    let input = read_to_string("inputs/day12.txt").expect("file not found");

    let values: Vec<(&str, &str)> = input.lines().map(|l| parse_line(l)).collect();

    let mut connections: Connections = HashMap::new();

    for (a, b) in values {
        let entry = connections.entry(a).or_insert(Vec::new());

        entry.push(b);

        let entry = connections.entry(b).or_insert(Vec::new());

        entry.push(a);
    }

    let mut visited_small_caves = HashMap::new();
    visited_small_caves.insert("start", 1);

    let result = all_paths(
        &["start"],
        &connections,
        visited_small_caves,
        &may_visit_part_1,
    );

    println!("Part 1: {}", result.len());

    let mut visited_small_caves = HashMap::new();
    visited_small_caves.insert("start", 1);

    let result = all_paths(
        &["start"],
        &connections,
        visited_small_caves,
        &may_visit_part_2,
    );

    println!("Part 2: {}", result.len());
}

fn all_paths<'a, F>(
    start: &[&'a str],
    connections: &'a Connections,
    visited_small_caves: HashMap<&'a str, usize>,
    f: &F,
) -> Vec<Vec<&'a str>>
where
    F: Fn(&str, &HashMap<&str, usize>) -> bool,
{
    if start.last().unwrap() == &"end" {
        return vec![start.to_owned()];
    }

    let mut result: Vec<Vec<&'a str>> = vec![];

    for n in connections.get(start.last().unwrap()).unwrap() {
        let mut visited_small_caves = visited_small_caves.clone();

        if !f(n, &visited_small_caves) {
            continue;
        }

        if n.chars().all(|c| c.is_ascii_lowercase()) {
            let entry = visited_small_caves.entry(n).or_insert(0);
            *entry += 1;
        }

        let mut new_start = start.to_owned();
        new_start.append(&mut vec![*n]);

        let mut paths = all_paths(&new_start, &connections, visited_small_caves.clone(), f);
        result.append(&mut paths);
    }

    result
}

fn may_visit_part_1(node: &str, visited_small_caves: &HashMap<&str, usize>) -> bool {
    let visit_count = visited_small_caves.get(node);

    match visit_count {
        Some(n) if *n > 0 => false,
        _ => true,
    }
}

fn may_visit_part_2(node: &str, visited_small_caves: &HashMap<&str, usize>) -> bool {
    let visit_count = visited_small_caves.get(node);

    match visit_count {
        Some(2) => {
            return false;
        }
        Some(1) if node == "start" || node == "end" => {
            return false;
        }
        Some(1) if visited_small_caves.values().any(|c| *c >= 2) => {
            return false;
        }
        Some(_) => {
            return true;
        }
        None => {
            return true;
        }
    }
}
