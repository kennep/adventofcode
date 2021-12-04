import sys

numbers = [int(n) for n in sys.stdin.readlines()]

for i in range(0, len(numbers)):
	for j in range(0, len(numbers)):
		for k in range(0, len(numbers)):
			a = numbers[i]
			b = numbers[j]
			c = numbers[k]
			if a + b  + c == 2020:
				print a * b * c
