use std::{io::{stdin, BufRead}, str::FromStr};

#[derive(Debug)]
enum Instruction {
    AddX(i32),
    Noop
}

impl Instruction {
    fn cycle_count(&self) -> i32 {
        match self {
            Instruction::AddX(_) => 2,
            Instruction::Noop => 1
        }
    }
}

#[derive(Debug)]
struct InstructionParseErr(String);

impl FromStr for Instruction {
    type Err = InstructionParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ws = s.split_whitespace();
        let instruction_str = match ws.next() {
            Some(i) => i,
            None => return Err(InstructionParseErr(s.to_owned()))
        };
        let instruction = match instruction_str {
            "addx" => 
                match ws.next() {
                    Some(o) => match o.parse::<i32>() {
                        Ok(oi) => Instruction::AddX(oi),
                        Err(_) => return Err(InstructionParseErr(s.to_owned()))
                    }
                    None => return Err(InstructionParseErr(s.to_owned()))
                },
            "noop" => Instruction::Noop,
            _ => return Err(InstructionParseErr(s.to_owned()))
        };
        Ok(instruction)
    }
}

struct Cpu {
    x: i32,
    cycles: i32
}

impl Cpu {
    fn execute_instruction(&mut self, instruction: &Instruction)
    {
        self.cycles += instruction.cycle_count();
        match instruction {
            Instruction::AddX(o) => self.x += o,
            Instruction::Noop => ()
        };
    }
}

fn main() {
    let program: Vec<Instruction> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();

    let mut signal_strength = 0;
    let mut cpu = Cpu{ x: 1, cycles: 0 };
    let mut last_emit_cycles = 0;

    let mut screen: Vec<char> = std::iter::repeat('!').take(40 * 6).collect();
    let mut crt_x = 0;
    let mut crt_y = 0;

    for instruction in program {
        let x_before_instr = cpu.x;
        let instruction_cycles = instruction.cycle_count();
        for _ in 0..instruction_cycles {
            if (x_before_instr - crt_x).abs() <= 1 {
                screen[(crt_y * 40 + crt_x) as usize] = '#'
            } else {
                screen[(crt_y * 40 + crt_x) as usize] = ' '
            }
            crt_x += 1;
            if crt_x >= 40 {
                crt_x -= 40;
                crt_y += 1
            }
        }

        cpu.execute_instruction(&instruction);

        if (last_emit_cycles == 0 && cpu.cycles - last_emit_cycles >= 20) ||
            cpu.cycles - last_emit_cycles >= 40 {
            last_emit_cycles = cpu.cycles / 20 * 20;
            println!("Cycle: {} X value: {}", last_emit_cycles, x_before_instr);
            signal_strength += x_before_instr * last_emit_cycles;
        }
    }

    println!("Sum of signal strength: {}", signal_strength);
    println!("Screen:");
    for row in screen[..].chunks(40) {
        println!("{}", row.iter().collect::<String>())
    }
}
