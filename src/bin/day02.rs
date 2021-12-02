use std::fs::read_to_string;

enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

fn parse_line(line: &str) -> Option<Command> {
    if line.starts_with("forward") {
        Some(Command::Forward(line[8..].parse::<i64>().ok()?))
    } else if line.starts_with("down") {
        Some(Command::Down(line[5..].parse::<i64>().ok()?))
    } else if line.starts_with("up") {
        Some(Command::Up(line[3..].parse::<i64>().ok()?))
    } else {
        None
    }
}

fn main() {
    let input = read_to_string("inputs/day02.txt").expect("file not found");

    let commands = input
        .lines()
        .map(|l| parse_line(l).unwrap())
        .collect::<Vec<_>>();

    let result_1 = commands.iter().fold((0, 0), |(hor, dep), c| match c {
        Command::Forward(n) => (hor + n, dep),
        Command::Down(n) => (hor, dep + n),
        Command::Up(n) => (hor, dep - n),
    });

    println!("Part 1: {}", result_1.0 * result_1.1);

    let result_2 = commands
        .iter()
        .fold((0, 0, 0), |(hor, dep, aim), c| match c {
            Command::Forward(n) => (hor + n, dep + aim * n, aim),
            Command::Down(n) => (hor, dep, aim + n),
            Command::Up(n) => (hor, dep, aim - n),
        });

    println!("Part 2: {}", result_2.0 * result_2.1);
}
