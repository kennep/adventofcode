import sys

map = [list(l.strip()) for l in sys.stdin.readlines()]

right=3
down=1
posx=0
posy=0

trees = 0
while posy < len(map):
	posx += right
	posy += down
	if posy >= len(map):
		break
	line = map[posy]
	print ''.join(line),
	if posx >= len(line):
		posx -= len(line)
	if line[posx] == '#':
		trees += 1
		line[posx] = 'X'
	else:
		line[posx] = 'O'
	print '',
	print ''.join(line)

print "Number of trees: %d" % trees

