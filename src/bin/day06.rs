use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/day06.txt").expect("file not found");

    let fish_input: Vec<i128> = input
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|l| l.parse::<i128>().expect("not a number"))
        .collect();

    let mut cache: HashMap<i128, i128> = HashMap::new();

    let sum: i128 = fish_input
        .iter()
        .map(|f| lanternfish(80 - *f, &mut cache))
        .sum();

    println!("Part 1: {}", sum);

    let mut cache: HashMap<i128, i128> = HashMap::new();

    let sum: i128 = fish_input
        .iter()
        .map(|f| lanternfish(256 - *f, &mut cache))
        .sum();

    println!("Part 2: {}", sum);
}

fn lanternfish(remaining_days: i128, cache: &mut HashMap<i128, i128>) -> i128 {
    if let Some(n) = cache.get(&remaining_days) {
        return *n;
    }

    if remaining_days <= 0 {
        return 1;
    }

    // Only the day of reproduction is interesting, so we skip these "boring" days of mere
    // existence..
    let r = lanternfish(remaining_days - 7, cache) + lanternfish(remaining_days - 9, cache);
    cache.insert(remaining_days, r);
    r
}
