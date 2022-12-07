use regex::Regex;
use std::{io::{stdin, BufRead}, str::FromStr, num::ParseIntError};

#[derive(Debug,Clone,Copy)]
struct MoveInput{
    num_moves: usize,
    from_stack: usize,
    to_stack: usize
}

impl FromStr for MoveInput
{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();
        let captures = re.captures(s).unwrap();
        let num_moves = captures[1].parse::<usize>()?;
        let from_stack = captures[2].parse::<usize>()?;
        let to_stack = captures[3].parse::<usize>()?;

        return Ok(MoveInput{num_moves, from_stack, to_stack});
    }
}

fn main() {
    let lines: Vec<_> = stdin().lock().lines().map(|l| l.unwrap()).collect();

    let line_groups: Vec<Vec<String>> = lines.split(|l| l.is_empty()).map(|g| g.to_vec()).collect();

    let stack_lines: Vec<_> = line_groups[0].iter().rev().map(|s| s.to_owned()).collect();
    let moves: Vec<_> = line_groups[1].iter().map(|s| s.parse::<MoveInput>().unwrap()).collect();
    let stacks = create_stacks(&stack_lines);

    part_a(stacks.clone(), &moves);
    part_b(stacks.clone(), &moves);
}

fn create_stacks<'a>(stack_input: &'a[String]) -> Vec<Vec<String>>
{
    let max_stack_label = stack_input[0]
        .split_whitespace()
        .map(|d| d.parse::<u32>().unwrap())
        .max()
        .unwrap();
    println!("There are {} stacks", max_stack_label);
    let mut stacks: Vec<Vec<String>> = Vec::new();
    for _ in 1..=max_stack_label {
        stacks.push(Vec::new());
    }
    for stack_line in stack_input.iter().skip(1) {
        let stack_elems: Vec<Vec<char>> = stack_line
            .chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|c| c.to_vec())
            .collect();

        for (i, x) in stack_elems.iter().enumerate() {
            if x.len() < 2 || *x.iter().nth(0).unwrap() != '[' || i >= stacks.len() {
                continue;
            }
            let label = x.iter().nth(1).unwrap().to_string();
            stacks[i].push(label);
        }
    }
    stacks
}

fn part_a<'a>(mut stacks: Vec<Vec<String>>, move_input: &'a[MoveInput]) {
    println!("Stacks: {:?}", stacks);
    for crate_move in move_input {
        for _ in 1..=crate_move.num_moves {
            let elem = stacks[crate_move.from_stack - 1].pop().unwrap();
            stacks[crate_move.to_stack - 1].push(elem);
        }
    }
    println!("Stack after moves: {:?}", stacks);
    let top_cates: String = stacks.iter().map(|s| s[s.len() - 1].to_owned()).collect();
    println!("Top crates: {:?}", top_cates)
}

fn part_b<'a>(mut stacks: Vec<Vec<String>>, move_input: &'a[MoveInput]) {
    println!("Stacks: {:?}", stacks);
    for crate_move in move_input {
        let from_stack_len = stacks[crate_move.from_stack - 1].len();

        let mut moved_crates = Vec::from(&stacks[crate_move.from_stack - 1][from_stack_len - crate_move.num_moves..]);
        stacks[crate_move.to_stack - 1].append(&mut moved_crates);
        stacks[crate_move.from_stack - 1].resize(from_stack_len - crate_move.num_moves, String::from(""));
    }
    println!("Stack after moves: {:?}", stacks);
    let top_cates: String = stacks.iter().map(|s| s[s.len() - 1].to_owned()).collect();
    println!("Top crates: {:?}", top_cates)
}
