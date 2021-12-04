import sys

class World:
    def __init__(self):
        self.cubes = set()
        self.bounds = {
            'x': (0, 0),
            'y': (0, 0),
            'z': (0, 0),
            'w': (0, 0),
        }

    def add_active_cube(self, **coords):
        for coord_name, coord in coords.items():
            (lo, hi) = self.bounds[coord_name]
            if coord < lo:
                self.bounds[coord_name] = (coord, hi)
            elif coord > hi:
                self.bounds[coord_name] = (lo, coord)
        self.cubes.add((coords['x'], coords['y'], coords['z'], coords['w']))

    def number_of_active_neighbors(self, x, y, z, w):
        neighbors = [(xn, yn, zn, wn) for xn in range(x-1, x+2) for yn in range(y-1, y+2) for zn in range(z-1, z+2) for wn in range(w-1, w+2) if (xn, yn, zn, wn) != (x, y, z, w)]
        active_neighbors = [(xn, yn, zn, wn) for (xn, yn, zn, wn) in neighbors if (xn, yn, zn, wn) in self.cubes]
        return len(active_neighbors)

    def iterate_world(self):
        new_world = World()
        lo_x, hi_x = self.bounds['x']
        lo_y, hi_y = self.bounds['y']
        lo_z, hi_z = self.bounds['z']
        lo_w, hi_w = self.bounds['w']

        for w in range(lo_w -1, hi_w + 2):
            for z in range(lo_z -1, hi_z +2):
                for y in range(lo_y -1, hi_y +2):
                    for x in range(lo_x -1, hi_x +2):
                        if (x, y, z, w) in self.cubes: # Active cube
                            if self.number_of_active_neighbors(x, y, z, w) in (2, 3):
                                new_world.add_active_cube(x=x, y=y, z=z, w=w)
                        else: # Inactive
                            if self.number_of_active_neighbors(x, y, z, w) == 3:
                                new_world.add_active_cube(x=x, y=y, z=z, w=w)
        return new_world

    def print_world(self):
        lo_x, hi_x = self.bounds['x']
        lo_y, hi_y = self.bounds['y']
        lo_z, hi_z = self.bounds['z']
        lo_w, hi_w = self.bounds['w']
        for w in range(lo_w -1, hi_w +2):
            for z in range(lo_z -1, hi_z +2):
                print(f"z={z}, w={w}")
                for y in range(lo_y -1, hi_y +2):
                    line = ['#' if (x, y, z) in self.cubes else '.' for x in range(lo_x -1, hi_x +2)]
                    print(f"{y: 9d} {''.join(line)}")


world = World()
y=0
for line in sys.stdin.readlines():
    for x, char in enumerate(line):
        if char == '#':
            world.add_active_cube(x=x, y=y, z=0, w=0)
    y+= 1

print(f"Before any cycles:")
world.print_world()

for cycles in range(0, 6):
    world = world.iterate_world()
    print(f"After {cycles} cycle(s):")
    world.print_world()

print(f"There are {len(world.cubes)} active cubes.")



            