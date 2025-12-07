var input = java.io.File("input.txt").readText().trim()
    .split("\n").map {  it.trim() }

data class Point(val x: Int, val y: Int)

var startingPosition = Point(input[0].indexOf('S') - 1, 0)
assert(startingPosition.x != -1)

var beamPositions = listOf(startingPosition)

var splits = 0

while(beamPositions[0].y < input.size - 1) {
    beamPositions = beamPositions.map { Point(it.x, it.y + 1) }
    beamPositions = beamPositions.flatMap { pos ->
        when(input[pos.y][pos.x]) {
            '.' -> listOf(pos)
            '^' -> {
                splits += 1
                listOf(Point(pos.x - 1, pos.y), Point(pos.x + 1, pos.y))
            }
            else -> {
                assert(false)
                listOf(pos)
            }
        }
    }.distinct()
    showMap(input, beamPositions)
}

fun showMap(map: List<String>, positions: List<Point>) {
    for (y in map.indices) {
        for (x in map[y].indices) {
            if (positions.any { it.x == x && it.y == y }) {
                print('|')
            } else {
                print(map[y][x])
            }
        }
        println()
    }
    println()
}

println("Num splits: $splits")
