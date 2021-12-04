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
	counts = 0
	for pos in (min, max):
		if password[pos-1] == letter:
			counts += 1
	if counts == 1:
		valid += 1
		print "%d-%d %s: %s Valid" % (min, max, letter, password)
	else:
		print "%d-%d %s: %s Invalid" % (min, max, letter, password)
print "Valid passwords: %d" % valid

