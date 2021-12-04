use std::io::{stdin, BufRead};

enum Command {
    Up(i32),
    Down(i32),
    Forward(i32)
}

struct StateA {
    depth: i32,
    position: i32
}

struct StateB {
    aim: i32,
    depth: i32,
    position: i32
}

fn parse(input: &str) -> Command {
    let fields: Vec<&str> = input.split(" ").collect();
    let arg: i32 = fields[1].parse().expect("Failed to parse argument");
    match fields[0] {
        "up" => Command::Up(arg),
        "down" => Command::Down(arg),
        "forward" => Command::Forward(arg),
        _ => Command::Forward(0)
    }
}

fn main() {
    let input: Vec<Command> = stdin().lock().lines()
        .map(|l| l.expect("failed to read line"))
        .filter(|l| l.len() > 0)
        .map(|l| parse(&l))
        .collect();

    let final_state = input.iter()
        .fold(StateA{depth: 0, position: 0}, |state, command| 
           match command {
               Command::Up(a) => StateA{depth: state.depth - a, position: state.position},
               Command::Down(a) => StateA{depth: state.depth + a, position: state.position},
               Command::Forward(a) => StateA{depth: state.depth, position: state.position + a}
           });
    println!("A: depth={} state={} dxs={}", final_state.depth, final_state.position, final_state.depth * final_state.position);

    let final_state = input.iter()
        .fold(StateB{aim: 0, depth: 0, position: 0}, |state, command| 
           match command {
               Command::Up(a) => StateB{aim: state.aim - a, depth: state.depth, position: state.position},
               Command::Down(a) => StateB{aim: state.aim + a, depth: state.depth, position: state.position},
               Command::Forward(a) => StateB{aim: state.aim, depth: state.depth + state.aim * a, position: state.position + a}
           });
    println!("B: aim={} depth={} state={} axdxs={}", final_state.aim, final_state.depth, final_state.position, final_state.depth * final_state.position);
}
