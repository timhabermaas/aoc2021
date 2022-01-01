use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
enum VarOrConst {
    Const(i64),
    Var(char),
}

impl VarOrConst {
    fn get(&self, map: &HashMap<char, i64>) -> i64 {
        match self {
            Self::Const(val) => *val,
            Self::Var(var) => *map.get(var).unwrap_or(&0),
        }
    }

    fn pretty_print(&self) -> String {
        match self {
            Self::Var(var) => var.to_string(),
            Self::Const(val) => val.to_string(),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Inp(char),
    Add(char, VarOrConst),
    Mod(char, VarOrConst),
    Div(char, VarOrConst),
    Mul(char, VarOrConst),
    Eql(char, VarOrConst),
}

impl Instruction {
    fn pretty_print(&self) -> String {
        match self {
            Self::Inp(var) => format!("inp {}", var),
            Self::Add(var, val) => format!("{} = {} + {};", var, var, val.pretty_print()),
            Self::Mod(var, val) => format!("{} = {} % {};", var, var, val.pretty_print()),
            Self::Div(var, val) => format!("{} = {} / {};", var, var, val.pretty_print()),
            Self::Mul(var, VarOrConst::Const(0)) => format!("{} = 0;", var),
            Self::Mul(var, val) => format!("{} = {} * {};", var, var, val.pretty_print()),
            Self::Eql(var, val) => format!("eql {} {}", var, val.pretty_print()),
        }
    }
}

fn parse_var_or_const(input: &str) -> VarOrConst {
    match input {
        "x" | "y" | "z" | "w" => VarOrConst::Var(input.chars().nth(0).unwrap()),
        _ => VarOrConst::Const(input.parse().unwrap()),
    }
}

fn parse_instruction(input: &str) -> Instruction {
    use Instruction::*;

    let (cmd, rest) = input.split_once(" ").unwrap();

    if cmd == "inp" {
        return Inp(rest.chars().nth(0).unwrap());
    }

    let (left, right) = rest.split(" ").collect_tuple().unwrap();

    let left = left.chars().nth(0).unwrap();
    let right = parse_var_or_const(right);

    match cmd {
        "mul" => Mul(left, right),
        "add" => Add(left, right),
        "div" => Div(left, right),
        "mod" => Mod(left, right),
        "eql" => Eql(left, right),
        _ => panic!("unknown command {}", cmd),
    }
}

fn evaluate(input: &str, instructions: &[Instruction]) -> i64 {
    use Instruction::*;

    let mut input = input.chars().rev().collect::<Vec<_>>();

    let mut variables: HashMap<char, i64> = HashMap::new();

    for i in instructions {
        match i {
            Inp(var) => {
                let x = input.pop().unwrap();
                variables.insert(*var, x.to_string().parse().unwrap());
            }
            Add(var, val) => {
                variables.insert(*var, variables.get(var).unwrap_or(&0) + val.get(&variables));
            }
            Mod(var, val) => {
                variables.insert(*var, variables.get(var).unwrap_or(&0) % val.get(&variables));
            }
            Div(var, val) => {
                variables.insert(*var, variables.get(var).unwrap_or(&0) / val.get(&variables));
            }
            Mul(var, val) => {
                variables.insert(*var, variables.get(var).unwrap_or(&0) * val.get(&variables));
            }
            Eql(var, val) => {
                variables.insert(
                    *var,
                    if *variables.get(var).unwrap_or(&0) == val.get(&variables) {
                        1
                    } else {
                        0
                    },
                );
            }
        }
    }

    variables[&'z']
}

fn main() {
    let input = read_to_string("inputs/day24.txt").expect("file not found");

    let instructions: Vec<Instruction> = input.lines().map(|l| parse_instruction(l)).collect();
    // Mostly solved on paper. The MONAD code basically does the same thing for all 14 digits.
    // Sometimes a number (derived from the input digit and some constant) is "pushed" onto a stack
    // (using `* 26)` and sometimes it's popped from the stack (using `/ 26`). For each push/pop
    // pair there are some constraints which have to hold to end up with `z == 0`:
    //
    // d[2] == d[3]
    // d[4] + 2 == d[5]
    // d[9] == d[10] + 3
    // d[8] + 7 == d[11]
    // d[7] == d[12] + 8
    // d[0] == d[13] + 7
    // d[1] + 6 == d[6]

    // NOTE: Numbers are reversed for Rust version.
    println!("Part 1 (max): {}", evaluate_rust(21969299979939));
    println!(
        "Part 1 (max): {}",
        evaluate("93997999296912", &instructions)
    );
    println!("Part 2 (min): {}", evaluate_rust(11814197311118));
    println!(
        "Part 2 (min): {}",
        evaluate("81111379141811", &instructions)
    );
}

// Puzzle input encoded as Rust program.
fn evaluate_rust(mut input: i64) -> i64 {
    let mut w;
    let mut x;
    let mut z = 0;

    let offsets_1 = [10, 14, 14, -13, 10, -13, -7, 11, 10, 13, -4, -9, -13, -9];
    let offsets_2 = [2, 13, 13, 9, 15, 3, 6, 5, 16, 1, 6, 3, 7, 9];
    let divs = [1, 1, 1, 26, 1, 26, 26, 1, 1, 1, 26, 26, 26, 26];

    for i in 0..14 {
        w = input % 10;

        x = z % 26 + offsets_1[i];

        z = z / divs[i];

        if x != w {
            z = z * 26 + w + offsets_2[i];
        }

        input = input / 10;
    }

    z
}
