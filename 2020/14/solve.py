import sys
import re

INSTRUCTION_PATTERN = re.compile(r"(?P<opcode>[a-z]+)\[?(?P<address>[0-9]+)?\]? = (?P<operand>[X0-9]+)")

program = [INSTRUCTION_PATTERN.match(l).groupdict() for l in sys.stdin.readlines()]

for instruction in program:
    if instruction['opcode'] == 'mask':
        mask = instruction['operand']
        instruction['and_mask'] = int(mask.replace('X', '1'), 2)
        instruction['or_mask'] = int(mask.replace('X', '0'), 2)        
    elif instruction['opcode'] == 'mem':
        instruction['operand'] = int(instruction['operand'])

memory = {}

and_mask = 0
or_mask = 0
for instruction in program:
    if instruction['opcode'] == 'mask':
        and_mask = instruction['and_mask']
        or_mask = instruction['or_mask']
    elif instruction['opcode'] == 'mem':
        memory[instruction['address']] = (instruction['operand'] | or_mask) & and_mask

addrsum = sum(memory.values())
print(f"Sum of memory values: {addrsum}")