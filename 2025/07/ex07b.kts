var input = java.io.File("input.txt").readText().trim()
    .split("\n").map {  it.trim() }

data class Point(val x: Int, val y: Int, val dup: Long)

var startingPosition = Point(input[0].indexOf('S') - 1, 0, 1)
assert(startingPosition.x != -1)

var beamPositions = listOf(startingPosition)

var splits = 1L

while(beamPositions[0].y < input.size - 1) {
    beamPositions = beamPositions.map { Point(it.x, it.y + 1, it.dup) }
    beamPositions = beamPositions.flatMap { pos ->
        when(input[pos.y][pos.x]) {
            '.' -> listOf(pos)
            '^' -> {
                splits += pos.dup
                listOf(Point(pos.x - 1, pos.y, pos.dup), Point(pos.x + 1, pos.y, pos.dup))
            }
            else -> {
                assert(false)
                listOf(pos)
            }
        }
    }.groupBy { Pair(it.x, it.y) }
     .map { entry -> 
         val totalDup = entry.value.sumOf { it.dup }
         Point(entry.key.first, entry.key.second, totalDup)
     }
    //showMap(input, beamPositions)
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

println("Num timelines: $splits")
