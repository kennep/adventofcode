import sys

instructions = [(s[0], int(s[1:])) for s in sys.stdin.readlines()]

# negative x - west,
# positive x - east
# negative y - north
# positive y - south

xpos = 0
ypos = 0
heading = 90

headings = {
    0: (0, -1), # north
    90: (1, 0), # east
    180: (0, 1), # south
    270: (-1, 0) # west
}

for cmd, val in instructions:
    if cmd == 'N':
        ypos -= val
    if cmd == 'S':
        ypos += val
    if cmd == 'E':
        xpos += val
    if cmd == 'W':
        xpos -= val
    if cmd == 'F':
        xpos += val * headings[heading][0]
        ypos += val * headings[heading][1]
    if cmd == 'R':
        heading = (heading + val) % 360
    if cmd == 'L':
        heading = (heading - val) % 360

print(f"Ending coordinate: {xpos}, {ypos}. Manhattan distance: {abs(xpos) + abs(ypos)}")
