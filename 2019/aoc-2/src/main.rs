use std::io::{stdin,BufRead};

fn intcode(mut input: Vec<usize>) -> Vec<usize>
{
    let mut ip: usize = 0;
    loop {
        let opcode = input[ip];
        match opcode {
            1 => {
                let op1_ptr = input[ip + 1];
                let op2_ptr = input[ip + 2];
                let output_ptr = input[ip + 3];
                let op1 = input[op1_ptr];
                let op2 = input[op2_ptr];
                let result = op1 + op2;
                input[output_ptr]  = result;
            },
            2 => {
                let op1_ptr = input[ip + 1];
                let op2_ptr = input[ip + 2];
                let output_ptr = input[ip + 3];
                let op1 = input[op1_ptr];
                let op2 = input[op2_ptr];
                let result = op1 * op2;
                input[output_ptr]  = result;
            }
            99 => {
                println!("Program halted at ip: {}", ip);
                break;
            }
            o => {
                panic!("Invalid opcode {} at ip: {}", o, ip);
            }
        }
        ip += 4;
    }

    println!("Value at position 0 after program exit: {}", input[0]);
    return input;
}

fn main() {
    let input: Vec<usize> = stdin().lock().lines()
        .map(|l| l.expect("failed to read line"))
        .filter(|l| l.len() > 0)
        .flat_map(|l| l.split(',').map(|n| n.parse::<usize>().unwrap_or_else(|_| panic!("Expected number, not {}", n))).collect::<Vec<usize>>())
        .collect();

    intcode(input.clone());

    for v1 in 0..=99 {
        for v2 in 0..=99 {
            let mut mem = input.clone();
            mem[1] = v1;
            mem[2] = v2;
            mem = intcode(mem);
            if mem[0] == 19690720 {
                println!("Target value found, v1={}, v2={}, answer={}", v1, v2, 100*v1+v2);
                return;
            }
        }
    }
}

