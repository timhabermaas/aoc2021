use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq)]
struct Bingo {
    field: Vec<Vec<u32>>,
    marks: HashSet<(usize, usize)>,
}

impl Bingo {
    fn new(field: Vec<Vec<u32>>) -> Self {
        Self {
            field,
            marks: HashSet::new(),
        }
    }

    fn sum_of_unmarked(&self) -> u32 {
        self.field
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, n)| if !self.marks.contains(&(x, y)) { *n } else { 0 })
            })
            .sum()
    }

    fn is_bingo(&self) -> bool {
        // Sorting values, otherwise `group_by` doesn't work (it's non-deterministic even). It only
        // groups consecutive elements, same as Haskell's `groupBy`.
        let mut xs: Vec<_> = self.marks.iter().map(|(x, _y)| x).collect();
        xs.sort();
        let mut ys: Vec<_> = self.marks.iter().map(|(_x, y)| y).collect();
        ys.sort();

        for (_, g) in &xs.iter().group_by(|x| *x) {
            if g.count() == self.field[0].len() {
                return true;
            }
        }

        for (_, g) in &ys.iter().group_by(|y| *y) {
            if g.count() == self.field.len() {
                return true;
            }
        }
        false
    }

    fn mark(&mut self, n: u32) {
        if self.is_bingo() {
            return;
        }

        for y in 0..self.field.len() {
            for x in 0..self.field[0].len() {
                if self.field[y][x] == n {
                    self.marks.insert((x, y));
                    return;
                }
            }
        }
    }
}

#[derive(Debug)]
struct Game {
    inputs: Vec<u32>,
    bingos: Vec<Bingo>,
}

impl Game {
    fn mark(&mut self, n: u32) -> Option<&Bingo> {
        for b in self.bingos.iter_mut() {
            b.mark(n);
        }

        self.bingos.iter().find(|b| b.is_bingo())
    }

    fn drop_bingos(&mut self) {
        self.bingos.retain(|b| !b.is_bingo());
    }
}

fn main() {
    let input = read_to_string("inputs/day04.txt").expect("file not found");

    let mut game = parse_bingos(&input);

    let mut result = 0;

    let inputs = game.inputs.clone();

    // Part 1
    for n in &inputs {
        if let Some(b) = game.mark(*n) {
            result = b.sum_of_unmarked() * n;
            break;
        }
    }

    println!("Part 1: {}", result);

    let mut game = parse_bingos(&input);

    let mut result: u32 = 0;

    for n in inputs {
        game.mark(n);

        if game.bingos.len() == 1 && game.bingos[0].is_bingo() {
            result = game.bingos[0].sum_of_unmarked() * n;
            break;
        }

        game.drop_bingos();
    }

    println!("Part 2: {}", result);
}

fn parse_bingos(input: &str) -> Game {
    let mut lines = input.lines();

    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| str::parse(n).unwrap())
        .collect();

    lines.next().unwrap();

    let bingos: Vec<Bingo> = lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| str::parse::<u32>(n).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|x| Bingo::new(x.to_owned()))
        .collect();

    Game {
        inputs: numbers,
        bingos,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bingo_horizontal() {
        let mut b = Bingo::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        assert_eq!(b.is_bingo(), false);
        b.mark(1);
        b.mark(2);
        assert_eq!(b.is_bingo(), false);
        b.mark(3);
        assert_eq!(b.is_bingo(), true);
    }

    #[test]
    fn bingo_vertical() {
        let mut b = Bingo::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        assert_eq!(b.is_bingo(), false);
        b.mark(2);
        b.mark(6);
        assert_eq!(b.is_bingo(), false);
        b.mark(8);
        assert_eq!(b.is_bingo(), false);
        b.mark(5);
        assert_eq!(b.is_bingo(), true);
    }
}
