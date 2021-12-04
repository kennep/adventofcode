import sys
import time

numbers = [int(n) for n in sys.stdin.readline().split(",")]

number_dict = {}
last_number = None
i = 0
for n in numbers:
    if last_number is not None: number_dict[last_number] = i
    last_number = n
    i += 1

turns = len(numbers)

lt = time.time()
while turns < 30000000:
    #print(last_number)
    if not last_number in number_dict:
        new_number = 0
    else:        
        new_number = turns - number_dict[last_number]
    number_dict[last_number] = turns
    last_number = new_number

    t = time.time()
    if t - lt > 5:
        print(turns)
        lt = t
    turns += 1

print(last_number)
