use std::collections::HashSet;
use std::fs::read_to_string;

fn parse(input: &str) -> (Vec<(u32, u32)>, Vec<(&str, u32)>) {
    let mut coordinates = vec![];
    let mut folds = vec![];

    for l in input.lines() {
        if l == "" {
            continue;
        }
        if l.starts_with("fold") {
            let mut parts = l.split_whitespace();
            parts.next();
            parts.next();
            let c = parts.next().unwrap();
            let mut parts = c.split("=");
            folds.push((
                parts.next().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            ));
        } else {
            let mut parts = l.split(",");
            coordinates.push((
                parts.next().unwrap().parse::<u32>().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            ));
        }
    }

    (coordinates, folds)
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
