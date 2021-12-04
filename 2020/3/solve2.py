import sys

map = [list(l.strip()) for l in sys.stdin.readlines()]

product = 1
for (right, down) in [
	(1, 1),
	(3, 1),
	(5, 1),
	(7, 1),
	(1, 2)
	]:

	posx=0
	posy=0

	trees = 0
	while posy < len(map):
		posx += right
		posy += down
		if posy >= len(map):
			break
		line = list(map[posy])
		#print ''.join(line),
		if posx >= len(line):
			posx -= len(line)
		if line[posx] == '#':
			trees += 1
			line[posx] = 'X'
		else:
			line[posx] = 'O'
		#print '',
		#print ''.join(line)

	print "Number of trees for slope right %d down %d: %d" % (right, down, trees)
	product *= trees
print "Product of all trees encountered: %d" % product

