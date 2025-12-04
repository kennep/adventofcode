var input = java.io.File("input.txt").readText().trim()

var grid = input.split("\n").map { it.trim() }

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

fun isReachable(grid: List<String>, p: Point, maxX: Int, maxY: Int): Boolean {
    val rv = adjacentPoints(p, maxX, maxY)
        .count({ grid[it.y][it.x] == '@' }) < 4
    //println("Point $p is reachable: $rv adj: ${adjacentPts}")
    return rv
}

val numberReachable = grid.indices.sumOf { y ->
    grid[y].indices.count { x ->
        grid[y][x] == '@' && isReachable(grid, Point(x, y), grid[0].length, grid.size)
    }
}

println("Number of reachable points: $numberReachable")