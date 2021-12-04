import sys

numbers = [int(l) for l in sys.stdin.readlines()]

previous_numbers = numbers[0:25]

def calculate_sums(prev_numbers):
    sums = set()
    for num in prev_numbers:
        for other in [n for n in prev_numbers if n != num]:
            sums.add(num + other)
    return sums

for number in numbers[25:]:
    if number in calculate_sums(previous_numbers):
        previous_numbers = previous_numbers[1:] + [number]
    else:
        print(f"{number}:This number is not a sum of one of the previous numbers!")
        break
