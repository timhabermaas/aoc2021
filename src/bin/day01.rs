use std::fs::read_to_string;

fn count_increases<I>(iter: I) -> i64
where
    I: Iterator<Item = i64> + Clone,
{
    iter.clone()
        .zip(iter.skip(1))
        .map(|(old, new)| if old < new { 1 } else { 0 })
        .sum::<i64>()
}

fn main() {
    let input = read_to_string("inputs/day01.txt").expect("file not found");

    let values: Vec<i64> = input
        .lines()
        .map(|l| l.parse::<i64>().expect("not a number"))
        .collect();

    // Part 1
    println!("Part 1: {}", count_increases(values.iter().copied()));

    // Part 2
    let sums = values.windows(3).map(|w| w.iter().sum::<i64>());

    println!("Part 2: {}", count_increases(sums.into_iter()));
}
