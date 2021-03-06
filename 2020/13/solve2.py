import sys
from itertools import count
from math import ceil

sys.stdin.readline() # skip
buslines = sys.stdin.readline().split(",")

# t = 0 mod 7
# t = 1 mod 13
# t = 4 mod 59
# t = 6 mod 31
# t = 7 mod 19

lines = []
for i in range(0, len(buslines)):
    if buslines[i] == 'x': continue
    lines.append((int(buslines[i]), i))

from functools import reduce
def chinese_remainder(n, a):
    sum = 0
    prod = reduce(lambda a, b: a*b, n)
    for n_i, a_i in zip(n, a):
        p = prod // n_i
        sum += a_i * mul_inv(p, n_i) * p
    return sum % prod
 
 
 
def mul_inv(a, b):
    b0 = b
    x0, x1 = 0, 1
    if b == 1: return 1
    while a > 1:
        q = a // b
        a, b = b, a%b
        x0, x1 = x1 - q * x0, x0
    if x1 < 0: x1 += b0
    return x1

n = [l[0] for l in lines]
a = [l[0] - l[1] for l in lines]

print(chinese_remainder(n, a))

