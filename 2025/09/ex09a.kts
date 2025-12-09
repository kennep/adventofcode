data class Point(val x: Long, val y: Long)
var input = java.io.File("input.txt").readText().trim()
    .split("\n").map{ it.trim().split(",") }
    .map{ Point(it[0].toLong(), it[1].toLong()) }
println(input)

fun area(p1: Point, p2: Point): Long {
    return kotlin.math.abs(p2.x - p1.x + 1) * kotlin.math.abs(p2.y - p1.y + 1)
}

var areas =
    input.flatMap { p1 ->
        input.filter { p2 -> p1 != p2 }
            .map { p2 -> Pair(p1, p2) }
    }
        .flatMap { pair ->
            val (p1, p2) = pair
            listOf(
                pair,
                Pair(Point(p1.x, p2.y), Point(p2.x, p1.y))
            )
        }.map { Pair(it, area(it.first, it.second)) }

var maxArea = areas.maxOf { it.second }
//println(areas.joinToString("\n"))
println("Max area between points: $maxArea")
