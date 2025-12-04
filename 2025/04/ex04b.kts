var input = java.io.File("input.txt").readText().trim()

var grid = input.split("\n").map { it.trim().toCharArray() }

data class Point(val x: Int, val y: Int)
fun adjacentPoints(p: Point, maxX: Int, maxY: Int): List<Point> {
    val points = mutableListOf<Point>()
    if(p.x > 0) points.add(Point(p.x - 1, p.y))
    if(p.x < maxX - 1) points.add(Point(p.x + 1, p.y))
    if(p.y > 0) points.add(Point(p.x, p.y - 1))
    if(p.y < maxY - 1) points.add(Point(p.x, p.y + 1))

    // Diagonals
    if(p.x > 0 && p.y > 0) points.add(Point(p.x - 1, p.y - 1))
    if(p.x < maxX - 1 && p.y > 0) points.add(Point(p.x + 1, p.y - 1))
    if(p.x > 0 && p.y < maxY - 1) points.add(Point(p.x - 1, p.y + 1))
    if(p.x < maxX - 1 && p.y < maxY - 1) points.add(Point(p.x + 1, p.y + 1))
    return points
}

fun isReachable(grid: List<CharArray>, p: Point): Boolean {
    val rv = adjacentPoints(p, grid[0].size, grid.size)
        .count { grid[it.y][it.x] == '@' } < 4
    //println("Point $p is reachable: $rv adj: ${adjacentPts}")
    return rv
}

fun removeRolls(grid: List<CharArray>): Int
{
    var removed = 0
    for(y in grid.indices) {
        for(x in grid[y].indices) {
            if(grid[y][x] == '@' && isReachable(grid, Point(x, y))) {
                grid[y][x] = '.'
                removed += 1
            }
        }
    }
    return removed
}

var totalRemoved = 0
while(true) {
    val removed = removeRolls(grid)
    if (removed == 0) break
    totalRemoved += removed
}

println("Total removed rolls: $totalRemoved")

