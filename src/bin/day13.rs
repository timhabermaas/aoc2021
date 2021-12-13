use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

fn parse(input: &str) -> (Vec<(u32, u32)>, Vec<(&str, u32)>) {
    let (coordinates, folds) = input.split("\n\n").collect_tuple().unwrap();

    (
        coordinates
            .lines()
            .map(|l| {
                let (x, y) = l.split(",").collect_tuple().unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect(),
        folds
            .lines()
            .map(|l| {
                let (dir, value) = l
                    .split_whitespace()
                    .nth(2)
                    .unwrap()
                    .split("=")
                    .collect_tuple()
                    .unwrap();

                (dir, value.parse().unwrap())
            })
            .collect(),
    )
}

fn main() {
    let input = read_to_string("inputs/day13.txt").expect("file not found");

    let (coordinates, folds) = parse(&input);

    let mut dots = coordinates.iter().copied().collect::<HashSet<_>>();

    for fold in folds {
        let f = fold.1;

        let (before_fold, after_fold) = if fold.0 == "x" {
            let (mut after_fold, before_fold): (HashSet<(u32, u32)>, HashSet<(u32, u32)>) =
                dots.iter().partition(|(x, _y)| *x > f);

            after_fold = after_fold.iter().map(|(x, y)| (f - (x - f), *y)).collect();

            (before_fold, after_fold)
        } else {
            let (mut after_fold, before_fold): (HashSet<(u32, u32)>, HashSet<(u32, u32)>) =
                dots.iter().partition(|(_x, y)| *y > f);

            after_fold = after_fold.iter().map(|(x, y)| (*x, f - (y - f))).collect();

            (before_fold, after_fold)
        };

        dots = before_fold
            .union(&after_fold)
            .copied()
            .collect::<HashSet<_>>();

        print_paper(&dots);
        println!("{}", dots.len());
    }
}

fn print_paper(set: &HashSet<(u32, u32)>) {
    let max_y = *set.iter().map(|(_x, y)| y).max().unwrap();
    let max_x = *set.iter().map(|(x, _y)| x).max().unwrap();

    (0..=max_y).for_each(|y| {
        let line: String = (0..=max_x)
            .map(|x| if set.contains(&(x, y)) { '#' } else { '.' })
            .collect();

        println!("{}", line);
    })
}
