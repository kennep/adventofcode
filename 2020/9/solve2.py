import sys

numbers = [int(l) for l in sys.stdin.readlines()]

weakness = 731031916

for i in range(0, len(numbers)):
    for j in range(i + 2, len(numbers)):
        sum_range = numbers[i:j]
        if sum(sum_range) == weakness:
            min_number = min(sum_range)
            max_number = max(sum_range)
            print(f"The range {sum_range} sums to {weakness}.")
            print(f"The smallest number in the range is {min_number}")
            print(f"The largest number in the range is {max_number}")
            print(f"The sum of these is {min_number+max_number}.")
