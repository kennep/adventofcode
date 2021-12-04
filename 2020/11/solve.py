import sys

area = [[s for s in l.strip()] for l in sys.stdin.readlines()]
width = len(area[0])
height = len(area)

adjacent = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]

def new_tile(the_area, xpos, ypos):
    seat = the_area[ypos][xpos]
    adjacent_coords = [(xpos+xoff, ypos+yoff) for (xoff, yoff) in adjacent]
    adjacent_seats = [the_area[ya][xa] for (xa, ya) in adjacent_coords
        if xa >= 0 and xa < width and ya >= 0 and ya < height]

    occupied_seats = [s for s in adjacent_seats if s == '#']

    if seat == '.':
        return '.'
    if seat == 'L':
        if len(occupied_seats) == 0:
            return '#'
        else:
            return 'L'
    if seat == '#':
        if len(occupied_seats) >= 4:
            return 'L'
        else:
            return '#'

def print_area(the_area):
    for row in the_area:
        print(''.join(row))

rounds = 1
while True:
    print_area(area)
    print()
    new_area = []
    occupied = 0
    for y in range(0, len(area)):
        row = area[y]
        new_row = []
        for x in range(0, len(row)):
            s = new_tile(area, x, y)
            new_row.append(s)
            if s == '#':
                occupied += 1
        new_area.append(new_row)
    if new_area == area:
        print(f"No change after {rounds} rounds. Number of occupied seats = {occupied}")
        break
    rounds += 1
    area = new_area
