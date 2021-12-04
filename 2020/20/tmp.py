class F:
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


a = F()
a._imagedata = [
    [ 1, 2, 3],
    [ 4, 5, 6],
    [ 7, 8, 9]
]
a.rotation = 90
a.xflip = False
a.yflip = False

print(a.imagedata)
assert a.imagedata == [
    [ 7, 4, 1],
    [ 8, 5, 2],
    [ 9, 6, 3]
]

a.rotation = 0
a.xflip = True
a.yflip = False
print(a.imagedata)
assert a.imagedata == [
    [ 7, 8, 9],
    [ 4, 5, 6],
    [ 1, 2, 3]
]

a.rotation = 0
a.xflip = False
a.yflip = True
print(a.imagedata)
assert a.imagedata == [
    [ 3, 2, 1],
    [ 6, 5, 4],
    [ 9, 8, 7]
]

a.rotation = 90
a.xflip = False
a.yflip = True
print(a.imagedata)
assert a.imagedata == [
    [ 1, 4, 7],
    [ 2, 5, 8],
    [ 3, 6, 9]
]
