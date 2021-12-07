use std::fs::read_to_string;
fn main() {
    let input = read_to_string("inputs/day07.txt").expect("file not found");

    let values: Vec<i64> = input
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|l| l.parse::<i64>().expect("not a number"))
        .collect();

    println!("Part 1: {}", min_fuel(&values, fuel_cost_part_1).unwrap());

    println!("Part 2: {}", min_fuel(&values, fuel_cost_part_2).unwrap());
}

fn min_fuel<F>(positions: &[i64], fuel_fn: F) -> Option<i64>
where
    F: Fn(i64) -> i64,
{
    let min = *positions.iter().min()?;
    let max = *positions.iter().max()?;

    (min..=max)
        .map(|meeting_point| {
            positions
                .iter()
                .fold(0, |sum, h| fuel_fn((h - meeting_point).abs()) + sum)
        })
        .min()
}

fn fuel_cost_part_1(dist: i64) -> i64 {
    dist
}

fn fuel_cost_part_2(dist: i64) -> i64 {
    // triangular number
    dist * (dist + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_cost_part_2() {
        assert_eq!(fuel_cost_part_2(0), 0);
        assert_eq!(fuel_cost_part_2(1), 1);
        assert_eq!(fuel_cost_part_2(2), 3);
        assert_eq!(fuel_cost_part_2(3), 6);
        assert_eq!(fuel_cost_part_2(4), 10);
    }
}
