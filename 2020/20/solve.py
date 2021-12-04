import sys
import math

def flip(border):
    return int(''.join(reversed(f"{border:010b}")), 2)

LEFT = 'left'
RIGHT = 'right'
TOP = 'top'
BOTTOM = 'bottom'

leftborders = {}
topborders = {}
tileids = {}

class Tile:
    def __init__(self, tileid, rotation, xflip, yflip, borders, imagedata):
        self.id = tileid
        self.rotation = rotation
        self.xflip = xflip
        self.yflip = yflip
        self.borders = borders
        self._imagedata = imagedata
        leftborders.setdefault(self.borders[LEFT], []).append(self)
        topborders.setdefault(self.borders[TOP], []).append(self)
        tileids.setdefault(self.id, []).append(self)

    def __str__(self):
        rotation = ''
        if self.rotation != 0:
            rotation = f'r{"" if self.rotation == 90 else self.rotation}'        
        return f"Tile {self.id}{rotation}{'x' if self.xflip else ''}{'y' if self.yflip else ''}"

    def __repr__(self):
        rotation = ''
        return f"""
        {str(self)}:
        Left   : {self.borders[LEFT]:010b}
        Right  : {self.borders[RIGHT]:010b}
        Top    : {self.borders[TOP]:010b}
        Bottom : {self.borders[BOTTOM]:010b}
        """

    def rotated(self):
        return Tile(
            tileid = self.id,
            rotation = self.rotation + 90,
            xflip = self.xflip,
            yflip = self.yflip,
            borders = {
                TOP: flip(self.borders[LEFT]),
                RIGHT: self.borders[TOP],
                BOTTOM: flip(self.borders[RIGHT]),
                LEFT: self.borders[BOTTOM]
            },
            imagedata = self._imagedata
        )

    def xflipped(self):
        return Tile(
            tileid = self.id,
            rotation = self.rotation,
            xflip = not self.xflip,
            yflip = self.yflip,
            borders = {
                TOP: self.borders[BOTTOM],
                RIGHT: flip(self.borders[RIGHT]),
                BOTTOM: self.borders[TOP],
                LEFT: flip(self.borders[LEFT])
            },
            imagedata = self._imagedata
        )

    def yflipped(self):
        return Tile(
            tileid = self.id,
            rotation = self.rotation,
            xflip = self.xflip,
            yflip = not self.yflip,
            borders = {
                TOP: flip(self.borders[TOP]),
                RIGHT: self.borders[LEFT],
                BOTTOM: flip(self.borders[BOTTOM]),
                LEFT: self.borders[RIGHT]
            },
            imagedata = self._imagedata
        )

    @property
    def imagedata(self):
        imagedata = [y.copy() for y in self._imagedata]
        width = len(imagedata[0])
        height = len(imagedata)
        if self.rotation:
            imagedata_tmp = [y.copy() for y in imagedata]
            assert self.rotation == 90
            for y in range(0, width):
                for x in range(0, height):
                    imagedata[x][height - y - 1] = imagedata_tmp[y][x]
        if self.xflip:
            imagedata_tmp = [y.copy() for y in imagedata]
            for y in range(0, width):
                for x in range(0, height):
                    imagedata[height - y - 1][x] = imagedata_tmp[y][x]
        if self.yflip:
            imagedata_tmp = [y.copy() for y in imagedata]
            for y in range(0, width):
                for x in range(0, height):
                    imagedata[y][width - x - 1] = imagedata_tmp[y][x]
        return imagedata
        
tiles = []
while True:
    tileid = sys.stdin.readline()
    if not tileid.startswith("Tile"): break
    tileid, _ = tileid.split(":", 2)
    _, tileid = tileid.split(" ", 2)
    tileid = int(tileid)
    lines = []
    while True:
        line = sys.stdin.readline().strip()
        if not line: break
        line = line.replace(".", "0").replace("#", "1")
        lines.append(line)
    tile = Tile(tileid, 0, False, False, {
            TOP: int(lines[0], 2),
            BOTTOM: int(lines[-1], 2),
            LEFT: int(''.join(l[0] for l in lines), 2),
            RIGHT: int(''.join(l[-1] for l in lines), 2)
        }, [
            list(l[1:-1].replace("0", ".").replace("1", "#")) for l in lines[1:-1]
        ])
    tiles.append(tile)


print(f"Read {len(tiles)} tiles.")
side = int(math.sqrt(len(tiles)))
assert side * side == len(tiles)
print(f"This makes a {side}x{side} square.")

addtiles = []
for tile in tiles:
    addtiles.append(tile.xflipped())
    addtiles.append(tile.yflipped())
    addtiles.append(tile.xflipped().yflipped())
    tile = tile.rotated() #90
    addtiles.append(tile)
    addtiles.append(tile.xflipped())
    addtiles.append(tile.yflipped())
    addtiles.append(tile.xflipped().yflipped())
    #tile = tile.rotated().rotated() #270
    #addtiles.append(tile)
    #addtiles.append(tile.xflipped())
    #addtiles.append(tile.yflipped())
    #addtiles.append(tile.xflipped().yflipped())
    
tiles += addtiles

print(f"After flipping and rotation, we have a total of {len(tiles)} tiles to look at.")

full_matches = []
for tile in tiles:
    candidate_pictures = [[[tile]]]
    for y in range(0, side):
        for x in range(0, side):
            if x == 0 and y == 0: continue
            additional_pictures = []
            for picture in candidate_pictures:
                ref_left = None
                ref_top = None
                if x > 0:
                    ref_left = picture[y][x-1]
                if y > 0:
                    ref_top = picture[y-1][x]
                #candidate_tiles = [t for t in tiles if 
                #    t.id not in [p.id for row in picture for p in row] and
                #    (ref_left is None or ref_left.borders[RIGHT] == t.borders[LEFT]) and
                #    (ref_top is None or ref_top.borders[BOTTOM] == t.borders[TOP])
                #]
                if ref_left is not None:
                    matching_left = set(leftborders.get(ref_left.borders[RIGHT], []))
                if ref_top is not None:
                    matching_top = set(topborders.get(ref_top.borders[BOTTOM], []))
                if ref_left is None:
                    candidate_tiles = matching_top
                elif ref_top is None:
                    candidate_tiles = matching_left
                else:
                    candidate_tiles = matching_left.intersection(matching_top)
                candidate_tiles = candidate_tiles.difference(set(t for row in picture for p in row for t in tileids.get(p.id)))
                candidate_tiles = list(candidate_tiles)

                if len(candidate_tiles) == 0:
                    candidate_pictures.remove(picture)
                    break
                working_pictures = [picture]
                for i in range(0, len(candidate_tiles)-1):
                    picture_copy = [row.copy() for row in picture]
                    working_pictures.append(picture_copy)
                    additional_pictures.append(picture_copy)
                for i in range(0, len(candidate_tiles)):
                    pic = working_pictures[i]
                    tile = candidate_tiles[i]
                    if x == 0:
                        pic.append([])
                    pic[-1].append(tile)                       

    for picture in candidate_pictures:
        if len(picture) < side or len(picture[-1]) < side: continue
        print("*** MATCH FOUND ***")
        for y in picture:
            print('\t'.join(str(x) for x in y))
        print(f"Corner mult: {picture[0][0].id * picture[0][-1].id * picture[-1][-1].id * picture[-1][0].id}")
        full_matches.append(picture)

sea_monster = """
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   
"""
sea_monster_pixel_count = sea_monster.count('#')
sea_monster = sea_monster.split("\n")
sea_monster = [list(s) for s in sea_monster if s]
assert len(sea_monster) == 3
assert sea_monster_pixel_count == 15

for match in full_matches:
    scanlines = []
    for row in match:
        images = [p.imagedata for p in row]
    
        for scanline in range(0, len(images[0])):
            scanlines.append([pixel for image in images for pixel in image[scanline]])

    print("Candidate image:")
    #for scanline in scanlines:
    #    print(''.join(scanline))

    monster_pixels = set()
    sea_pixels = set()
    monsters_found = 0
    for y in range(0, len(scanlines)):
        for x in range(0, len(scanlines[0])):
            if scanlines[y][x] == '#':
                sea_pixels.add((x, y))

    for y in range(0, len(scanlines) - len(sea_monster)):
        for x in range(0, len(scanlines[0]) - len(sea_monster[0])):
            found_monster = True
            for smy in range(0, len(sea_monster)):
                for smx in range(0, len(sea_monster[0])):
                    if sea_monster[smy][smx] == '#' and scanlines[smy + y][smx + x] != '#':
                        found_monster = False
            if found_monster:
                print(f"*** Found a sea monster at coordinates {x}, {y}")
                monsters_found += 1
                for smy in range(0, len(sea_monster)):
                    for smx in range(0, len(sea_monster[0])):
                        if sea_monster[smy][smx] == '#':
                            try:
                                sea_pixels.remove((smx + x, smy + y))
                                monster_pixels.add((smx + x, smy + y))
                            except KeyError:
                                pass
    print(f"Total of {monsters_found} monsters found.")
    print(f"Sea roughness: {len(sea_pixels)}")

    for y in range(0, len(scanlines)):
        print(''.join(
            '#' if (x, y) in sea_pixels else 'O' if (x, y) in monster_pixels else ' '
            for x in range(0, len(scanlines[0]))
        ))
