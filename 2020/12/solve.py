import sys

instructions = [(s[0], int(s[1:])) for s in sys.stdin.readlines()]

# negative x - west,
# positive x - east
# negative y - north
# positive y - south

xpos = 0
ypos = 0
wpx = 10
wpy = -1

transforms = {
    0: ((1, 0), (0, 1)),
    90: ((0, -1), (1, 0)),
    180: ((-1, 0), (0, -1)),
    270: ((0, 1), (-1, 0))
}

for cmd, val in instructions:
    if cmd == 'N':
        wpy -= val
    if cmd == 'S':
        wpy += val
    if cmd == 'E':
        wpx += val
    if cmd == 'W':
        wpx -= val
    if cmd == 'F':
        xpos += wpx * val
        ypos += wpy * val
    if cmd == 'R':
        tf = transforms[val]
        tpx = wpx
        tpy = wpy
        wpx = tpx * tf[0][0] + tpy * tf[0][1]
        wpy = tpx * tf[1][0] + tpy * tf[1][1]
    if cmd == 'L':
        tf = transforms[360-val]
        tpx = wpx
        tpy = wpy
        wpx = tpx * tf[0][0] + tpy * tf[0][1]
        wpy = tpx * tf[1][0] + tpy * tf[1][1]
    print(f"{cmd} {val}: ({xpos},{ypos}) Waypoint: ({wpx},{wpy})")

print(f"Ending coordinate: {xpos}, {ypos}. Manhattan distance: {abs(xpos) + abs(ypos)}")
