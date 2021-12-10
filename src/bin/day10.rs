use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/day10.txt").expect("file not found");

    let lines: Vec<&str> = input.lines().collect();

    let checked_lines = lines.iter().map(|l| check_line(l)).collect::<Vec<_>>();

    let mut sum = 0;
    for l in &checked_lines {
        match l {
            Ok(_) => {}
            Err(ParseFail::Corrupted(c)) => {
                let p = match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!("invalid char"),
                };
                sum += p;
            }
            _ => {}
        }
    }

    println!("Part 1: {}", sum);

    let mut scores: Vec<i64> = checked_lines
        .iter()
        .filter_map(|l| match l {
            Err(ParseFail::Incomplete(s)) => Some(s),
            _ => None,
        })
        .map(|stack| {
            stack.iter().rev().fold(0, |sum, c| {
                let p = match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("invalid char"),
                };
                sum * 5 + p
            })
        })
        .collect();

    scores.sort();

    println!("Part 2: {}", scores[scores.len() / 2]);
}

enum ParseFail {
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn check_line(line: &str) -> Result<(), ParseFail> {
    let mut stack = Vec::new();

    for c in line.chars() {
        match c {
            '{' | '(' | '<' | '[' => stack.push(c),
            '}' | ')' | '>' | ']' => match stack.pop() {
                Some('(') if c == ')' => {}
                Some('[') if c == ']' => {}
                Some('{') if c == '}' => {}
                Some('<') if c == '>' => {}
                _ => return Err(ParseFail::Corrupted(c)),
            },
            _ => panic!("invalid char"),
        }
    }

    if !stack.is_empty() {
        return Err(ParseFail::Incomplete(stack));
    }

    Ok(())
}
