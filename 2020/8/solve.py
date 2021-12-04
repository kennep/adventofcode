import sys

def parse_line(l):
    op, arg = l.split(" ")
    arg = int(arg)
    return op, arg
    
program_orig = [parse_line(l) for l in sys.stdin.readlines()]

def run_program(program):
    visited_instructions = set()

    ip = 0
    acc = 0

    while True:
        if ip == len(program):
            print(f"Program terminated normally. Acc={acc}")
            return True
        op, arg = program[ip]
        #print(f"ip={ip} acc={acc}: {op} {arg}")
        if ip in visited_instructions:
            print(f"Program loop detected at instruction {ip}. Accumulator value: {acc}")
            return False
        visited_instructions.add(ip)
        if op == 'acc':
            acc += arg
        elif op == 'jmp':
            ip += arg
            continue
        ip += 1

for ip in range(0, len(program_orig)):
    program = program_orig.copy()
    op, arg = program[ip]
    if op == 'jmp':
        print(f"Modifying program by changing jmp -> nop at instruction {ip}")
        program[ip] = ('nop', arg)
    elif op == 'nop':
        print(f"Modifying program by changing nop -> jmp at instruction {ip}")
        program[ip] = ('jmp', arg)
    else:
        continue
    if run_program(program):
        print("Solution found.")
        break
