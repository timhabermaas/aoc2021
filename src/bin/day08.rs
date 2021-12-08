use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn parse_line(line: &str) -> (Vec<HashSet<char>>, Vec<String>) {
    let mut parts = line.split(" | ");
    let first = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|w| w.chars().collect::<HashSet<_>>())
        .collect();
    let last = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|w| {
            let mut chars = w.chars().collect::<Vec<_>>();
            chars.sort();
            chars.iter().collect::<String>()
        })
        .collect();

    (first, last)
}

fn main() {
    let input = read_to_string("inputs/day08.txt").expect("file not found");

    let values: Vec<(Vec<HashSet<char>>, Vec<String>)> =
        input.lines().map(|l| parse_line(l)).collect();

    let result = values
        .iter()
        .flat_map(|(_, l)| l.iter())
        .filter(|d| [2 as usize, 4, 3, 7].contains(&d.len()))
        .count();

    println!("Part 1: {}", result);

    let mut sum = 0;
    for (digits, number) in values {
        /*
         *   aaaa
         *  b    c
         *  b    c
         *   dddd
         *  e    f
         *  e    f
         *   gggg
         */

        // Digits with unique length:
        let one = digits.iter().find(|d| d.len() == 2).unwrap();
        let four = digits.iter().find(|d| d.len() == 4).unwrap();
        let seven = digits.iter().find(|d| d.len() == 3).unwrap();
        let eight = digits.iter().find(|d| d.len() == 7).unwrap();

        // Finding the remaining digits/elements using set operations:

        // {a} = 7 \ 1
        let a = difference(&seven, &one);

        // {eg} = 8 \ {a} \ 4
        let mut eg = difference(&eight, &a);
        eg = difference(&eg, &four);

        // Removing {eg} from all digits leaves 3 and 7 with a length of 3.
        let two = digits
            .iter()
            .find(|d| difference(d, &eg).len() == 3 && **d != *seven)
            .unwrap();

        // {bf} = 8 \ 2
        let bf = difference(&eight, &two);
        // {c} = 1 \ {bf}
        let c = difference(&one, &bf);
        // {f} = 1 \ {c}
        let f = difference(&one, &c);
        // {d} = 2 \ {eg} \ {a} \ {c}
        let d = difference(&difference(&difference(&two, &eg), &a), &c);

        // 0 = 8 \ {d}
        let zero = difference(&eight, &d);

        // 6 = 8 \ {c}
        let six = difference(&eight, &c);

        // 8 \ 2 \ {f}
        let b = difference(&difference(&eight, &two), &f);

        // The difference between 3 and 9 is b. we can find 9 and 3 by simple brute force.
        let (three, nine) = digits
            .iter()
            .cartesian_product(digits.iter())
            .find(|(pot_three, pot_nine)| {
                pot_nine.len() == 6 && difference(&pot_nine, &pot_three) == b
            })
            .unwrap();

        // 5 = (3 \ {c}) + {b}
        let five = union(&difference(&three, &c), &b);

        let map: HashMap<String, i32> = [
            (zero.iter().sorted().collect(), 0),
            (one.iter().sorted().collect(), 1),
            (two.iter().sorted().collect(), 2),
            (three.iter().sorted().collect(), 3),
            (four.iter().sorted().collect(), 4),
            (five.iter().sorted().collect(), 5),
            (six.iter().sorted().collect(), 6),
            (seven.iter().sorted().collect(), 7),
            (eight.iter().sorted().collect(), 8),
            (nine.iter().sorted().collect(), 9),
        ]
        .into_iter()
        .collect();

        sum += to_digits(&map, &number);
    }

    println!("Part 2: {}", sum);
}

fn to_digits(map: &HashMap<String, i32>, s: &[String]) -> i32 {
    s.iter().rev().enumerate().fold(0, |sum, (i, d)| {
        let n = *map.get(d).unwrap();
        sum + n * 10_i32.pow(i as u32)
    })
}

fn difference(a: &HashSet<char>, b: &HashSet<char>) -> HashSet<char> {
    a.difference(b).copied().collect()
}

fn union(a: &HashSet<char>, b: &HashSet<char>) -> HashSet<char> {
    a.union(b).copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_digits() {
        let map = [
            ("abc".to_string(), 0),
            ("gfr".to_string(), 2),
            ("ita".to_string(), 8),
        ]
        .into_iter()
        .collect();

        let result = to_digits(
            &map,
            &vec!["ita".to_string(), "gfr".to_string(), "abc".to_string()],
        );
        assert_eq!(result, 820);
    }
}
