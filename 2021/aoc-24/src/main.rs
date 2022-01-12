use std::io::{stdin, BufRead};
use std::collections::{HashMap};
use std::env;

#[derive(Debug,Copy,Clone)]
enum Register {
    X,
    Y,
    W,
    Z
}

#[derive(Debug,Copy,Clone)]
enum Operand {
    Register(Register),
    Immediate(i64)
}

#[derive(Debug,Copy,Clone)]
enum Instruction {
    Inp(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Eql(Register, Operand)
}

#[derive(Debug)]
struct Alu<'a, T: Iterator<Item=i64>> {
    input: T,
    program: &'a Vec<Instruction>,
    ip: usize,
    x: i64,
    y: i64,
    w: i64,
    z: i64    
}

impl<T: Iterator<Item=i64>> Alu<'_, T> {
    fn new<'a>(input: T, program: &'a Vec<Instruction>) -> Alu<'a, T> {
        Alu{input, program, ip:0, x: 0, y: 0, w: 0, z: 0}
    }

    fn set_register(&mut self, register: Register, value: i64) {
        match register {
            Register::X => self.x = value,
            Register::Y => self.y = value,
            Register::W => self.w = value,
            Register::Z => self.z = value
        }
    }

    fn get_register(&self, register: Register) -> i64 {
        match register {
            Register::X => self.x,
            Register::Y => self.y,
            Register::W => self.w,
            Register::Z => self.z
        }
    }

    fn get_value(&self, operand: Operand) -> i64 {
        match operand {
            Operand::Register(register) => self.get_register(register),
            Operand::Immediate(value) => value
        }
    }
    
    fn run(&mut self) {
        loop {
            if self.ip >= self.program.len() {
                break;
            }
            let instruction = self.program[self.ip];
            match instruction {
                Instruction::Inp(reg) => {
                    let input = self.input.next();
                    match input {
                        Some(value) => self.set_register(reg, value.abs()),
                        None => {
                            break;
                        }
                    }
                },
                Instruction::Add(reg, op) => self.set_register(reg, 
                    self.get_register(reg) + self.get_value(op)),
                Instruction::Mul(reg, op) => self.set_register(reg, 
                    self.get_register(reg) * self.get_value(op)),
                Instruction::Div(reg, op) => self.set_register(reg, 
                    self.get_register(reg) / self.get_value(op)),
                Instruction::Mod(reg, op) => self.set_register(reg, 
                    self.get_register(reg) % self.get_value(op)),
                Instruction::Eql(reg, op) => self.set_register(reg, 
                    if self.get_register(reg) == self.get_value(op) { 1 } else { 0 }),
            }
            self.ip += 1;
        }
    }
}

fn parse_operand(operand: &str) -> Operand
{
    match operand {
        "x" => Operand::Register(Register::X),
        "y" => Operand::Register(Register::Y),
        "z" => Operand::Register(Register::Z),
        "w" => Operand::Register(Register::W),
        i => Operand::Immediate(i.parse::<i64>().unwrap())
    }
}

fn parse_register(register: &str) -> Register
{
     match parse_operand(register) {
         Operand::Register(r) => r,
         _ => panic!("Register expected: {}", register)
     }
}

fn parse_instruction(line: &str) -> Instruction
{
    let mut components = line.split(" ");

    let instruction = components.next().unwrap();
    let register = parse_register(components.next().unwrap());
    let operand = match components.next() {
        Some(o) => Some(parse_operand(o)),
        None => None
    };

    match instruction {
        "inp" => Instruction::Inp(register),
        "add" => Instruction::Add(register, operand.unwrap()),
        "mul" => Instruction::Mul(register, operand.unwrap()),
        "div" => Instruction::Div(register, operand.unwrap()),
        "mod" => Instruction::Mod(register, operand.unwrap()),
        "eql" => Instruction::Eql(register, operand.unwrap()),
        _ => panic!("Illegal instruction: {}", line)
    }
}

fn find_z_and_w_values(ip: usize, base: i64, target_z: i64, program: &Vec<Instruction>) -> Vec<(i64, i64)> {
    let mut result: Vec<(i64, i64)> = Vec::new();
    for z in 0..52 {
         for w in 1..10 {
             let v = z + base - 26 ;
            let mut alu = Alu::new(vec![w].into_iter(), program);
            alu.z = v;
            alu.ip = ip;
            alu.run();
            if alu.z == target_z {
                result.push((v, w));
            }
        }
    }
    if result.len() == 0 && base > 0 {
        return find_z_and_w_values(ip, base / 26, target_z, program);
    }
    result
}

#[derive(Copy,Clone)]
enum Operation {
    FindLargest,
    FindSmallest
}

// This function tries to work "backwards" - start with the last input, and figure out
// which combination of input and input z value gives a z value of zero.
// Then for each combination, figure out which combination of input and z value that
// gives the z value we want on the following steps, and so on.
// Along the way we use some heuristics to find appropriate z valus to try with, but
// the algorihm is not very efficient, so this still takes quite some time.
fn allowed_input(ip: usize, program: &Vec<Instruction>, operation: Operation) -> HashMap<i64, Vec<i64>> {
    let mut result: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut found_inp = false;
    for i in ip + 1 .. program.len() {
        if let Instruction::Inp(_) = program[i] {
            found_inp = true;
            let ali = allowed_input(i, program, operation);
            loop {
                for (target_z, _) in ali.iter() {
                    let zaw = find_z_and_w_values(ip, target_z * 26, *target_z, &program);
                    for (z, w) in zaw {
                        let mut r_vec = vec![w];
                        let additional_input = &ali[target_z];
                        for inp in additional_input.iter() {
                            r_vec.push(*inp);
                        }
                        let entry = result.get(&z);
                        if let Some(input_vec) = entry {
                            match operation {
                                Operation::FindLargest if r_vec > *input_vec => { result.insert(z, r_vec); }, 
                                Operation::FindSmallest if r_vec < * input_vec => { result.insert(z, r_vec); },
                                _ => ()
                            }
                        } else {
                            result.insert(z, r_vec);
                        }
                    }
                }
                if result.len() > 0 {
                    break;
                }
                break;
            }
            break;
        }
    }
    if !found_inp {
        for (z, w) in find_z_and_w_values(ip, 14, 0, &program) {
            result.insert(z, vec![w]);
        }
    }
    for (z, v) in result.iter() {
        println!("z: {} i: {:?}", z, v);
    }
    println!("{} results from ip {}", result.len(), ip);
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Expected operation - one of: smallest largest");
        return;
    }

    let program: Vec<Instruction> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| parse_instruction(&l))
        .collect();


    
    let result = allowed_input(0, &program, match args[1].as_str() {
        "smallest" => Operation::FindSmallest,
        "largest" => Operation::FindLargest,
        _ => {
            println!("Invalid operation!");
            return;
        }
    });
    println!("{} results found", result.len());
    for (z, inp) in result {
        if z == 0 {
            println!("{} {}", z, inp.iter().map(|d| d.to_string()).collect::<Vec<String>>().concat());
        }
    }
}