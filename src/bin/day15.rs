use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/day15.txt").expect("file not found");

    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| format!("{}", c).parse::<u32>().expect("not a number"))
                .collect()
        })
        .collect();

    let distance = a_star(&grid, (0, 0), (grid.len() - 1, grid[0].len() - 1));

    println!("Part 1: {}", distance);

    let old_height = grid.len();
    let old_width = grid[0].len();
    let mut new_grid: Vec<Vec<u32>> = vec![vec![0; old_width * 5]; old_height * 5];

    for i in 0..5 {
        for j in 0..5 {
            let risk_diff = manhattan_distance((0, 0), (i, j)) as u32;

            for row in 0..grid.len() {
                for col in 0..grid[0].len() {
                    let new_risk = if grid[row][col] + risk_diff > 9 {
                        (grid[row][col] + risk_diff) % 9
                    } else {
                        grid[row][col] + risk_diff
                    };
                    new_grid[row + i * old_height][col + j * old_width] = new_risk;
                }
            }
        }
    }

    let result = a_star(
        &new_grid,
        (0, 0),
        (new_grid.len() - 1, new_grid[0].len() - 1),
    );
    println!("Part 2: {}", result);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    pos: (usize, usize),
    guessed_cost_til_goal: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .guessed_cost_til_goal
            .cmp(&self.guessed_cost_til_goal)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

fn a_star(grid: &[Vec<u32>], start: (usize, usize), goal: (usize, usize)) -> usize {
    let mut open_nodes: BinaryHeap<Node> = BinaryHeap::new();
    let mut costs: HashMap<(usize, usize), usize> = HashMap::new();

    open_nodes.push(Node {
        pos: start,
        guessed_cost_til_goal: manhattan_distance(start, goal),
    });
    costs.insert(start, 0);

    while let Some(next_node) = open_nodes.pop() {
        if next_node.pos == goal {
            return next_node.guessed_cost_til_goal;
        }

        for n in neighbours(grid, next_node.pos.0, next_node.pos.1) {
            let edge_cost = grid[n.0][n.1];
            let new_cost = edge_cost as usize + costs[&next_node.pos];

            if new_cost < *costs.get(&n).unwrap_or(&usize::MAX) {
                costs.insert(n, new_cost);
                let new_guessed_cost = new_cost + manhattan_distance(n, goal);

                open_nodes.push(Node {
                    pos: n,
                    guessed_cost_til_goal: new_guessed_cost,
                });
            }
        }
    }

    panic!("couldn't find goal");

    0
}

fn neighbours(grid: &[Vec<u32>], row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];

    if row >= 1 {
        result.push((row - 1, col));
    }

    if row < grid.len() - 1 {
        result.push((row + 1, col));
    }

    if col >= 1 {
        result.push((row, col - 1));
    }

    if col < grid[0].len() - 1 {
        result.push((row, col + 1));
    }

    result
}

fn manhattan_distance(from: (usize, usize), to: (usize, usize)) -> usize {
    // avoiding underflow naively:
    let x_diff = if from.0 > to.0 {
        from.0 - to.0
    } else {
        to.0 - from.0
    };

    let y_diff = if from.1 > to.1 {
        from.1 - to.1
    } else {
        to.1 - from.1
    };

    x_diff + y_diff
}
