use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
};

///         [G]         [D]     [Q]    
/// [P]     [T]         [L] [M] [Z]    
/// [Z] [Z] [C]         [Z] [G] [W]    
/// [M] [B] [F]         [P] [C] [H] [N]
/// [T] [S] [R]     [H] [W] [R] [L] [W]
/// [R] [T] [Q] [Z] [R] [S] [Z] [F] [P]
/// [C] [N] [H] [R] [N] [H] [D] [J] [Q]
/// [N] [D] [M] [G] [Z] [F] [W] [S] [S]
/// 1   2   3   4   5   6   7   8   9
fn main() {
    let file = File::open("input.txt").unwrap();
    let mut row = 0;
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(9);
    for _i in 0..9 {
        stacks.push(Vec::new())
    }

    for line in io::BufReader::new(file).lines().map(|x| x.unwrap()) {
        if row < 8 {
            parse_stack(&line, &mut stacks);
        }
        if row > 9 {
            let instruction = parse_instruction(&line);
            apply_instruction(&mut stacks, instruction);
        }

        row += 1;
    }
    print_stack(&stacks);
    for stack in stacks {
        print!("{}", stack.last().unwrap())
    }
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    source: usize,
    dest: usize,
}

fn print_stack(stacks: &Vec<Vec<char>>) {
    let mut idx = 1;
    for stack in stacks {
        println!("Stack {}: {:?}", idx, stack);
        idx += 1;
    }
}

fn apply_instruction(stacks: &mut Vec<Vec<char>>, instruction: Instruction) {
    let mut moving = Vec::new();
    for _i in 0..instruction.count {
        let removed = stacks[instruction.source].pop().unwrap();
        moving.insert(0, removed);
    }
    stacks[instruction.dest].append(&mut moving)
}

fn parse_instruction(row: &str) -> Instruction {
    let re = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
    let matches = re.captures(row).unwrap();

    let count = matches.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let source = matches.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
    let dest = matches.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
    Instruction {
        count,
        source,
        dest,
    }
}

fn parse_stack(row: &str, stacks: &mut Vec<Vec<char>>) {
    let binding = row.chars().collect::<Vec<char>>();
    let cols = binding.chunks(4);

    for (idx, col) in cols.enumerate() {
        for each in col {
            if each.is_alphabetic() {
                stacks[idx].insert(0, each.clone())
            }
        }
    }
}
