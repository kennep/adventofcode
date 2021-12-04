import sys
import re

INSTRUCTION_PATTERN = re.compile(r"(?P<opcode>[a-z]+)\[?(?P<address>[0-9]+)?\]? = (?P<operand>[X0-9]+)")

program = [INSTRUCTION_PATTERN.match(l).groupdict() for l in sys.stdin.readlines()]

longest_x = 0
for instruction in program:
    if instruction['opcode'] == 'mask':
        mask = instruction['operand']
        xcount = mask.count('X')
        if xcount > longest_x:
            longest_x = xcount
        instruction['x_mask'] = int(mask.replace('1', '0').replace('X', '1'), 2)
        instruction['or_mask'] = int(mask.replace('X', '0'), 2)
        instruction['xcount'] = xcount
    elif instruction['opcode'] == 'mem':
        instruction['operand'] = int(instruction['operand'])
        instruction['address'] = int(instruction['address'])

print(f"Max number of Xes in program: {longest_x}. Max addresses modified per instruction: {2**longest_x}.")


memory = {}

x_mask = 0
or_mask = 0
xcount = 0

def bp(val):
    return f"{val:036b}"

def rotr(val):
    bit = val & 1
    val >>= 1
    if bit:
        val |= 1 << 35
    return val

allones = (1 << 36) - 1

for instruction in program:
    print(instruction)
    if instruction['opcode'] == 'mask':        
        or_mask = instruction['or_mask']
        xcount = instruction['xcount']
        x_mask = instruction['x_mask']
    elif instruction['opcode'] == 'mem':
        address = instruction['address']
        address |= or_mask
        operand = instruction['operand']
        for i in range(0, 2**xcount):
            x_tmp = x_mask
            addr_val = address
            i_tmp = i
            a_pos = 0
            while x_tmp > 0:
                if x_tmp & 1:
                    if i_tmp & 1:
                        addr_val |= 1 << a_pos
                    else:
                        addr_val &= ~(1 << a_pos)
                    i_tmp >>= 1
                x_tmp >>= 1
                a_pos += 1

            #print(f"{addr_val} = {operand}")
            memory[addr_val] = operand
    #print(memory)

addrsum = sum(memory.values())
print(f"Sum of memory values: {addrsum}")