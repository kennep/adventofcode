import sys

area = [[s for s in l.strip()] for l in sys.stdin.readlines()]
width = len(area[0])
height = len(area)

directions = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]

def find_seat(the_area, xpos, ypos, xoff, yoff, width, height):
    xpos += xoff
    ypos += yoff
    while xpos >= 0 and ypos >= 0 and xpos < width and ypos < height:
        tile = the_area[ypos][xpos]
        if tile in ('#', 'L'):
            return tile
        xpos += xoff
        ypos += yoff
    return '.'

def new_tile(the_area, xpos, ypos):
    seat = the_area[ypos][xpos]    
    visible_seats = [find_seat(the_area, xpos, ypos, xoff, yoff, width, height) for (xoff, yoff) in directions]

    occupied_seats = [s for s in visible_seats if s == '#']

    if seat == '.':
        return '.'
    if seat == 'L':
        if len(occupied_seats) == 0:
            return '#'
        else:
            return 'L'
    if seat == '#':
        if len(occupied_seats) >= 5:
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
