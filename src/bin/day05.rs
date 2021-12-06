use std::collections::HashMap;
use std::fs::read_to_string;

fn parse_coord(coord: &str) -> (i32, i32) {
    let mut cs = coord.split(',');
    (
        str::parse(cs.next().unwrap()).unwrap(),
        str::parse(cs.next().unwrap()).unwrap(),
    )
}

fn parse_line(line: &str) -> Line {
    let mut parts = line.split(" -> ");
    let from = parse_coord(parts.next().unwrap());
    let to = parse_coord(parts.next().unwrap());

    Line(from..=to)
}

#[derive(Debug)]
struct Line(std::ops::RangeInclusive<(i32, i32)>);

struct LineIter {
    dir: (i32, i32),
    start: (i32, i32),
    current_step: u32,
    line_len: u32,
}

impl Iterator for LineIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let old_step = self.current_step as i32;

        if self.current_step >= self.line_len {
            return None;
        }

        self.current_step += 1;

        Some((
            (self.start.0 + (old_step * self.dir.0)),
            (self.start.1 + (old_step * self.dir.1)),
        ))
    }
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.0.start().1 == self.0.end().1
    }

    fn is_vertical(&self) -> bool {
        self.0.start().0 == self.0.end().0
    }

    fn iter(&self) -> LineIter {
        let diff_x = self.0.end().0 as i32 - self.0.start().0 as i32;
        let diff_y = self.0.end().1 as i32 - self.0.start().1 as i32;

        let dir_x = diff_x.signum();
        let dir_y = diff_y.signum();

        LineIter {
            dir: (dir_x, dir_y),
            start: *self.0.start(),
            current_step: 0,
            line_len: std::cmp::max(diff_x.abs(), diff_y.abs()) as u32 + 1,
        }
    }
}

fn main() {
    let input = read_to_string("inputs/day05.txt").expect("file not found");

    let lines: Vec<Line> = input.lines().map(|l| parse_line(l)).collect();

    let part_1_lines: Vec<_> = lines
        .iter()
        .filter(|r| r.is_horizontal() || r.is_vertical())
        .collect();

    let mut cover: HashMap<(i32, i32), u32> = HashMap::new();

    for p in part_1_lines.iter().flat_map(|r| r.iter()) {
        let entry = cover.entry(p).or_insert(0);
        *entry += 1;
    }

    println!(
        "Part 1 (without diagonal): {}",
        cover.iter().filter(|(_, v)| **v > 1).count()
    );

    let mut cover: HashMap<(i32, i32), u32> = HashMap::new();

    for p in lines.iter().flat_map(|r| r.iter()) {
        let entry = cover.entry(p).or_insert(0);
        *entry += 1;
    }

    println!("Part 2: {}", cover.iter().filter(|(_, v)| **v > 1).count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_diagonal() {
        let r1 = Line((1, 1)..=(3, 3));

        let mut iter = r1.iter();

        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.next(), Some((2, 2)));
        assert_eq!(iter.next(), Some((3, 3)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterator_horizontal() {
        let r1 = Line((1, 1)..=(4, 1));

        let mut iter = r1.iter();

        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.next(), Some((2, 1)));
        assert_eq!(iter.next(), Some((3, 1)));
        assert_eq!(iter.next(), Some((4, 1)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let r1 = Line((4, 1)..=(1, 1));

        let mut iter = r1.iter();

        assert_eq!(iter.next(), Some((4, 1)));
        assert_eq!(iter.next(), Some((3, 1)));
        assert_eq!(iter.next(), Some((2, 1)));
        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterator_vertical() {
        let r1 = Line((1, 1)..=(1, 4));

        let mut iter = r1.iter();

        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.next(), Some((1, 2)));
        assert_eq!(iter.next(), Some((1, 3)));
        assert_eq!(iter.next(), Some((1, 4)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
