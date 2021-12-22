use std::io::{stdin,stdout,BufRead,BufReader,Write};
use std::env;
use std::fs::File;

fn fetch(program: &Vec<i32>, arg: usize, mode: i32) -> i32
{
    if mode == 0 {
        let loc = program[arg];
        let rv = program[loc as usize];
        //println!("fetch(0): {} -> {}", loc, rv);
        rv
    } else {
        let rv = program[arg];
        //println!("fetch(1): {}", rv);
        rv
    }
}

fn store(program: &mut Vec<i32>, loc: usize, val: i32)
{
    let addr = program[loc];
    //println!("store: {} -> {}: {}", loc, addr, val);
    program[addr as usize] = val;
}

fn op_add(program: &mut Vec<i32>, ip: usize, mode_1: i32, mode_2: i32) -> usize {
    store(program, ip + 3, fetch(program, ip + 1, mode_1) + fetch(program, ip + 2, mode_2));
    4
}

fn op_mul(program: &mut Vec<i32>, ip: usize, mode_1: i32, mode_2: i32) -> usize {
    store(program, ip + 3, fetch(program, ip + 1, mode_1) * fetch(program, ip + 2, mode_2));
    4
}

fn op_input(program: &mut Vec<i32>, ip: usize) -> usize
{
    let mut input_text = String::new();
    print!("Input: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    let num = trimmed.parse::<i32>().unwrap();

    store(program, ip + 1, num);
    2
}

fn op_output(program: &Vec<i32>, ip: usize, mode: i32) -> usize
{
    let val = fetch(program, ip + 1, mode);
    println!("Output: {}", val);
    2
}

fn op_jump_if_true(program: &Vec<i32>, ip: &mut usize, mode_1: i32, mode_2: i32) -> usize
{
    let val = fetch(program, *ip + 1, mode_1);
    if val > 0 {
        *ip = fetch(program, *ip + 2, mode_2) as usize;
        return 0;
    }
    3
}

fn op_jump_if_false(program: &Vec<i32>, ip: &mut usize, mode_1: i32, mode_2: i32) -> usize
{
    let val = fetch(program, *ip + 1, mode_1);
    if val == 0 {
        *ip = fetch(program, *ip + 2, mode_2) as usize;
        return 0;
    }
    3
}

fn op_lt(program: &mut Vec<i32>, ip: usize, mode_1: i32, mode_2: i32) -> usize {
    store(program, ip + 3, if fetch(program, ip + 1, mode_1) < fetch(program, ip + 2, mode_2) { 1 } else { 0 });
    4
}

fn op_eq(program: &mut Vec<i32>, ip: usize, mode_1: i32, mode_2: i32) -> usize {
    store(program, ip + 3, if fetch(program, ip + 1, mode_1) == fetch(program, ip + 2, mode_2) { 1 } else { 0 });
    4
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();

    println!("Reading from filename: {}", filename);
    let fd = File::open(filename).expect("Could not open file");
    let mut program: Vec<i32> =  BufReader::new(fd).lines()
        .map(|l| l.expect("Could not read line"))
        .filter(|l| !l.is_empty())
        .flat_map(|l| l.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>())
        .collect();

    let mut ip: usize = 0;
    loop {
        let opcode_and_modes = program[ip];
        let opcode = opcode_and_modes % 100;
        let mode_1 = (opcode_and_modes / 100) % 10;
        let mode_2 = (opcode_and_modes / 1000) % 10;
        let mode_3 = (opcode_and_modes / 10000) % 10;
        //println!("Ip: {}, Opm: {}", ip, opcode_and_modes);
        ip += match opcode {
            1 => op_add(&mut program, ip, mode_1, mode_2),
            2 => op_mul(&mut program, ip, mode_1, mode_2),
            3 => op_input(&mut program, ip),
            4 => op_output(&program, ip, mode_1),
            5 => op_jump_if_true(&program, &mut ip, mode_1, mode_2),
            6 => op_jump_if_false(&program, &mut ip, mode_1, mode_2),
            7 => op_lt(&mut program, ip, mode_1, mode_2),
            8 => op_eq(&mut program, ip, mode_1, mode_2),
            99 => {
                println!("Program halted at ip: {}", ip);
                break;
            },
            _ => panic!("Invalid opcode at ip {}: {}", ip, opcode)
        };
    }
}
