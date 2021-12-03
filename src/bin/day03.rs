use std::fs::read_to_string;

fn part_1(input: &str) -> (u32, u32) {
    let rows: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let colums: Vec<Vec<char>> = (0..rows[0].len())
        .map(|i| rows.iter().map(|inner| inner[i].clone()).collect())
        .collect();

    let gamma_str: String = colums
        .iter()
        .map(|d| {
            if d.iter().filter(|x| **x == '1').count() >= d.iter().filter(|x| **x == '0').count() {
                '1'
            } else {
                '0'
            }
        })
        .collect();

    let bit_len = gamma_str.len();

    let gamma = u32::from_str_radix(&gamma_str, 2).unwrap();
    let mask: u32 = (1 << bit_len) - 1;

    let epsilon = !gamma & mask;

    (gamma, epsilon)
}

fn most_common_bit(numbers: &[u32], index: usize) -> u32 {
    let mask = 1 << index;
    let ones = numbers.iter().filter(|n| (*n & mask) > 0).count();

    if ones >= numbers.len() - ones {
        1
    } else {
        0
    }
}

fn least_common_bit(numbers: &[u32], index: usize) -> u32 {
    match most_common_bit(numbers, index) {
        0 => 1,
        1 => 0,
        _ => panic!("nope"),
    }
}

fn main() {
    let input = read_to_string("inputs/day03.txt").expect("file not found");

    // Part 1
    let (gamma, epsilon) = part_1(&input);

    println!("Part 1: {}", gamma * epsilon);

    // Part 2

    let bit_len = input.lines().nth(0).unwrap().len();

    let mut oxygens: Vec<u32> = input
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    for i in (0..bit_len).rev() {
        let mask = 1 << i;
        let b = most_common_bit(&oxygens, i);

        oxygens = oxygens
            .iter()
            .filter(|n| *n & mask == b << i)
            .copied()
            .collect();

        if oxygens.len() == 1 {
            break;
        }
    }

    let mut co2_ratings: Vec<u32> = input
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    for i in (0..bit_len).rev() {
        let mask = 1 << i;
        let b = least_common_bit(&co2_ratings, i);

        co2_ratings = co2_ratings
            .iter()
            .filter(|n| *n & mask == b << i)
            .copied()
            .collect();

        if co2_ratings.len() == 1 {
            break;
        }
    }

    let (oxygen, co2_rating) = (oxygens.first().unwrap(), co2_ratings.first().unwrap());

    println!("Part 2: {}", oxygen * co2_rating);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_common_bit() {
        assert_eq!(most_common_bit(&vec![0, 1, 1], 0), 1);
        assert_eq!(most_common_bit(&vec![0, 1], 0), 1);
        assert_eq!(most_common_bit(&vec![0, 1, 0], 0), 0);
    }
}
