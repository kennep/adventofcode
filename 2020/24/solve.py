import sys, re

input_regex = re.compile("(?:nw)|(?:ne)|(?:sw)|(?:se)|e|w")

tilerefs = [input_regex.findall(l) for l in sys.stdin.readlines()]

# x-axis: east-west
# y-axis: nw-se
# z-axis: ne-sw

# nw + sw = w
# (0, -1, 1) + (-1, 0, 1) = (-1, -1, 2)
#                         = (-1, 1, 0)

offsets = {
    'nw': (0, 1, -1),
    'se': (0, -1, 1),
    'ne': (1, 0, -1),
    'sw': (-1, 0, 1),
    'e': (1, -1, 0),
    'w': (-1, 1, 0)
}

tilecoords = {}

for tileref in tilerefs:
    coord = (0, -1, 1)
    for instr in tileref:
        offset = offsets[instr]
        coord = (coord[0] + offset[0], coord[1] + offset[1], coord[2] + offset[2])
    if coord in tilecoords:
        tilecoords[coord] = not tilecoords[coord]
    else:
        tilecoords[coord] = False # Black

black_tiles = 0
for coord, color in tilecoords.items():
    if color == False: black_tiles += 1
    print(f"({coord[0]}, {coord[1]}, {coord[2]}): {'white' if color else 'black'}")

print(f"Number of black tiles: {black_tiles} in the starting configuration")

def adjacent_black_tiles(tc, coords):
    black_tiles = 0
    for offset in offsets.values():
        c = (coords[0] + offset[0], coords[1] + offset[1], coords[2] + offset[2])
        if tc.get(c, True) == False:
            black_tiles += 1
    return black_tiles

def determine_color(tc, coords):
    adj = adjacent_black_tiles(tc, coords)
    if tc.get(coords, True):
        if adj == 2:
            return False
        return True
    else:
        if adj == 0 or adj > 2:
            return True
        else:
            return False   

for day in range(1, 101):
    new_tc = {}
    for coord, tile in tilecoords.items():
        new_tc[coord] = determine_color(tilecoords, coord)
        for offset in offsets.values():
            c = (coord[0] + offset[0], coord[1] + offset[1], coord[2] + offset[2])
            if not c in new_tc:
                new_tc[c] = determine_color(tilecoords, c)
    
    black_tiles = 0
    for coord, color in new_tc.items():
        if color == False: black_tiles += 1

    print(f"Day {day}: {black_tiles}")

    tilecoords = new_tc