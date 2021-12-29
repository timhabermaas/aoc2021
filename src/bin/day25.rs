use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/day25.txt").expect("file not found");

    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut iterations = 0;
    loop {
        let mut new_grid = grid.clone();

        let mut moved = false;

        // > first
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == '>' && grid[row][(col + 1) % grid[0].len()] == '.' {
                    new_grid[row][(col + 1) % grid[0].len()] = '>';
                    new_grid[row][col] = '.';
                    moved = true;
                } else if grid[row][col] == '>' {
                    new_grid[row][col] = '>';
                }
            }
        }

        grid = new_grid;
        new_grid = grid.clone();

        // v last
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == 'v' && grid[(row + 1) % grid.len()][col] == '.' {
                    new_grid[(row + 1) % grid.len()][col] = 'v';
                    new_grid[row][col] = '.';
                    moved = true;
                } else if grid[row][col] == 'v' {
                    new_grid[row][col] = 'v';
                }
            }
        }

        if !moved {
            break;
        }

        iterations += 1;
        grid = new_grid;
    }

    println!("Part 1: {}", iterations + 1);
}
