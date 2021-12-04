import sys

def parse(l):
	policy, password = l.strip().split(": ")
	limits, letter = policy.split(" ")
	min, max = limits.split("-")
	min = int(min)
	max = int(max)
	return (min, max, letter, password)

input = [parse(l) for l in sys.stdin.readlines()]

valid = 0
for (min, max, letter, password) in input:
	counts = len([c for c in password if c == letter])
	if counts >= min and counts <= max:
		valid += 1
		print "%d-%d %s: %s Valid: Contains %d %s" % (min, max, letter, password, counts, letter)
	else:
		print "%d-%d %s: %s Invalid: Contains %d %s" % (min, max, letter, password, counts, letter)
print "Valid passwords: %d" % valid

