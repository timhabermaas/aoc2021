use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

use itertools::Itertools;

fn parse_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn main() {
    let input = read_to_string("inputs/day09.txt").expect("file not found");

    let values: Vec<Vec<u32>> = parse_grid(&input);

    let local_minima: Vec<(usize, usize, u32)> = (0..values.len())
        .cartesian_product(0..values[0].len())
        .filter(|(i, j)| {
            let min_neighbour = *neighbours(&values, *i, *j)
                .iter()
                .map(|(_, _, v)| v)
                .min()
                .unwrap();
            values[*i][*j] < min_neighbour
        })
        .map(|(i, j)| (i, j, values[i][j]))
        .collect();

    println!(
        "Part 1: {}",
        local_minima
            .iter()
            .fold(0_u32, |sum, (_, _, h)| sum + h + 1)
    );

    // Part 2 is a flood fill (breadth first search using queue) until we find a 9. This leads to

    let basins: Vec<usize> = local_minima
        .iter()
        .map(|min| basin_size(&values, min.0, min.1))
        .sorted()
        .rev()
        .collect();

    println!("Part 2: {}", basins[0] * basins[1] * basins[2]);
}

struct FloodFill<'a> {
    visited: HashSet<(usize, usize)>,
    queue: VecDeque<(usize, usize)>,
    grid: &'a [Vec<u32>],
}

impl<'a> Iterator for FloodFill<'a> {
    type Item = (usize, usize, u32);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(c) = self.queue.pop_front() {
            for (i, j, v) in neighbours(self.grid, c.0, c.1) {
                if !self.visited.contains(&(i, j)) && v != 9 {
                    self.visited.insert((i, j));
                    self.queue.push_back((i, j));
                }
            }

            Some((c.0, c.1, self.grid[c.0][c.1]))
        } else {
            None
        }
    }
}

fn flood_fill_iter<'a>(grid: &'a [Vec<u32>], i: usize, j: usize) -> FloodFill<'a> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    queue.push_back((i, j));
    visited.insert((i, j));

    FloodFill {
        grid,
        queue,
        visited,
    }
}

fn basin_size(vec: &[Vec<u32>], i: usize, j: usize) -> usize {
    flood_fill_iter(vec, i, j).count()
}

fn neighbours(vec: &[Vec<u32>], i: usize, j: usize) -> Vec<(usize, usize, u32)> {
    let mut result = Vec::new();

    if i > 0 {
        result.push((i - 1, j, vec[i - 1][j]));
    }
    if i < vec.len() - 1 {
        result.push((i + 1, j, vec[i + 1][j]));
    }
    if j > 0 {
        result.push((i, j - 1, vec[i][j - 1]));
    }
    if j < vec[i].len() - 1 {
        result.push((i, j + 1, vec[i][j + 1]));
    }

    result
}
