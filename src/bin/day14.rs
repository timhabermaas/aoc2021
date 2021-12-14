use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn parse_insertion(insertion: &str) -> (Vec<u8>, u8) {
    let (from, to) = insertion.split(" -> ").collect_tuple().unwrap();

    (
        from.chars().map(|c| c as u8).collect(),
        to.chars().nth(0).unwrap() as u8,
    )
}

fn parse_input(input: &str) -> (Vec<u8>, HashMap<Vec<u8>, u8>) {
    let (template, insertions) = input.split("\n\n").collect_tuple().unwrap();
    (
        template.chars().map(|c| c as u8).collect(),
        insertions.lines().map(|l| parse_insertion(l)).collect(),
    )
}

fn main() {
    let input = read_to_string("inputs/day14.txt").expect("file not found");

    let (template, insertions) = parse_input(&input);
    dbg!(template.clone());
    dbg!(insertions.clone());

    let part_1 = run(template.clone(), &insertions, 10);

    println!("Part 1: {}", part_1);

    let part_2 = run_3(&template, &insertions, 40);

    println!("Part 2: {}", part_2);
}

fn run_3(template: &[u8], insertions: &HashMap<Vec<u8>, u8>, count: usize) -> usize {
    // Contains the count of all (overlapping) pairs. E.g. result['A']['B'] contains how often
    // `template` contains the substring "AB".
    let mut result: Vec<Vec<usize>> = vec![vec![0; 256]; 256];

    // Keeps track of how often each character is present in the target string.
    let mut counts: [usize; 256] = [0; 256];

    for t in template {
        counts[*t as usize] += 1;
    }

    for t in template.windows(2) {
        result[t[0] as usize][t[1] as usize] += 1;
    }

    print(&result);

    for i in 1..=count {
        println!("Step {}", i);

        let mut new_result: Vec<Vec<usize>> = result.clone();

        for (from, to) in insertions {
            if result[from[0] as usize][from[1] as usize] > 0 {
                // We want to replace AB with ACB, count how often AB occurs in the target string.
                let occurences = result[from[0] as usize][from[1] as usize];

                // Since we replace all AB, we remove all occurences from the next iteration.
                new_result[from[0] as usize][from[1] as usize] -= occurences;
                // Add AC
                new_result[from[0] as usize][*to as usize] += occurences;
                // Add CB
                new_result[*to as usize][from[1] as usize] += occurences;
                // We produced #occurences new Cs.
                counts[*to as usize] += occurences;
            }
        }

        result = new_result;
    }

    print(&result);

    println!("Counts:");
    print_single(&counts);

    counts.sort();

    counts[255] - counts.iter().filter(|x| **x > 0).nth(0).unwrap()
}

fn print_single(x: &[usize; 256]) {
    for i in 0..256 {
        if x[i] > 0 {
            println!("{}: {}", i as u8 as char, x[i]);
        }
    }
}

fn print(x: &[Vec<usize>]) {
    for i in 0..256 {
        for j in 0..256 {
            if x[i][j] > 0 {
                println!("{}{}: {}", i as u8 as char, j as u8 as char, x[i][j]);
            }
        }
    }
}

fn run(mut template: Vec<u8>, insertions: &HashMap<Vec<u8>, u8>, count: usize) -> usize {
    let mut result: Vec<u8> = Vec::with_capacity(template.len() * 2);

    for _ in 1..=count {
        result = Vec::with_capacity(template.len() * 2);

        for t in template.windows(2) {
            let foo = insertions.get(t);

            result.push(t[0]);
            result.push(*foo.unwrap());
        }

        result.push(*template.last().unwrap());

        template = result.clone();
    }

    let (min, max) = count_min_max(result);
    max - min
}

fn count_min_max(template: Vec<u8>) -> (usize, usize) {
    let mut map = [0; 256];

    for c in template {
        map[c as usize] += 1;
    }

    map.sort();

    (*map.iter().filter(|x| **x > 0).min().unwrap(), map[255])
}
