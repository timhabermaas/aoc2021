use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Image(HashMap<(i32, i32), bool>, bool);

impl Image {
    fn new() -> Self {
        Self(HashMap::new(), false)
    }

    fn default(&self) -> bool {
        self.1
    }

    fn set_default(&mut self, default: bool) {
        self.1 = default;
    }

    fn from_hash_map(map: HashMap<(i32, i32), bool>) -> Self {
        Self(map, false)
    }

    fn get(&self, row: i32, col: i32) -> bool {
        *self.0.get(&(row, col)).unwrap_or(&self.1)
    }

    fn set(&mut self, row: i32, col: i32, value: bool) {
        self.0.insert((row, col), value);
    }

    fn min_row(&self) -> i32 {
        *self.0.keys().map(|(row, _)| row).min().unwrap()
    }

    fn max_row(&self) -> i32 {
        *self.0.keys().map(|(row, _)| row).max().unwrap()
    }

    fn min_col(&self) -> i32 {
        *self.0.keys().map(|(_, col)| col).min().unwrap()
    }

    fn max_col(&self) -> i32 {
        *self.0.keys().map(|(_, col)| col).max().unwrap()
    }

    fn count_lights(&self) -> usize {
        self.0.values().filter(|x| **x).count()
    }
}

fn parse_input(input: &str) -> (Vec<bool>, Image) {
    let (index, map) = input.split("\n\n").collect_tuple().unwrap();

    let mut result = HashMap::new();

    for (row, line) in map.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                result.insert((row as i32, col as i32), true);
            }
        }
    }

    (
        index.chars().map(|c| c == '#').collect(),
        Image::from_hash_map(result),
    )
}

fn enhance_image(map: Image, index: &[bool]) -> Image {
    let mut result = Image::new();

    for row in map.min_row() - 1..=map.max_row() + 1 {
        for col in map.min_col() - 1..=map.max_col() + 1 {
            let binary = vec![
                map.get(row - 1, col - 1),
                map.get(row - 1, col),
                map.get(row - 1, col + 1),
                map.get(row, col - 1),
                map.get(row, col),
                map.get(row, col + 1),
                map.get(row + 1, col - 1),
                map.get(row + 1, col),
                map.get(row + 1, col + 1),
            ];
            let mut pos = 8;
            let mut n: usize = 0;

            for b in binary {
                n += if b { 1 << pos } else { 0 };
                pos -= 1;
            }

            result.set(row, col, index[n]);
        }
    }

    // rest of universe is full of # => look up last index to find new default
    if map.default() {
        result.set_default(index[index.len() - 1]);
    // rest of universe is full of .
    } else {
        result.set_default(index[0]);
    }

    result
}

fn main() {
    let input = read_to_string("inputs/day20.txt").expect("file not found");

    let (index, image) = parse_input(&input);

    let new_image = (0..2).fold(image.clone(), |image, _| enhance_image(image, &index));

    println!("Part 1: {}", new_image.count_lights());

    let new_image = (0..50).fold(image.clone(), |image, _| enhance_image(image, &index));

    println!("Part 2: {}", new_image.count_lights());
}
