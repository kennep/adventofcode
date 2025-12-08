import kotlin.math.pow
import kotlin.math.sqrt

data class Point(val x: Int, val y: Int, val z: Int)

var input = java.io.File("example.txt").readText().trim()
    .split("\n").map { it.trim().split(",") }
    .map { Point(it[0].toInt(), it[1].toInt(), it[2].toInt()) }

fun straightLineDistance(p1: Point, p2: Point): Double {
    return sqrt(
        ((p2.x - p1.x).toDouble().pow(2)) +
                ((p2.y - p1.y).toDouble().pow(2)) +
                ((p2.z - p1.z).toDouble().pow(2))
    )
}

data class PointDistance(val p1: Point, val p2: Point, val distance: Double)
var closestPoints =
    input.map({ p1 ->
        val closestPoint = input.filter({ p2 -> p1 != p2 })
            .map({ p2 -> Pair(p2, straightLineDistance(p1, p2)) })
            .minBy({ it.second })
        PointDistance(p1, closestPoint.first, closestPoint.second)
    })
        .sortedBy{ it.distance }

var circuits = mutableListOf<List<Point>>()

var iterator = closestPoints.iterator()
var numConnections = 0
while(iterator.hasNext()) {
    var connection = iterator.next()
    var firstConnected = circuits.firstOrNull { c -> c.contains(connection.p1) }
    var secondConnected = circuits.firstOrNull { c -> c.contains(connection.p2) }
    if(firstConnected != null && secondConnected != null)
    {
        continue
    }
    numConnections++
    if(firstConnected != null)
    {
        circuits.remove(firstConnected)
        firstConnected = firstConnected + listOf(connection.p2)
        circuits.add(firstConnected)
    }
    else if(secondConnected != null)
    {
        circuits.remove(secondConnected)
        secondConnected = secondConnected + listOf(connection.p1)
        circuits.add(secondConnected)
    }
    else
    {
        circuits.add(listOf(connection.p1, connection.p2))
    }

    println(circuits.joinToString("\n"))
    var circuitSizes = circuits.map { it.size }
    println(circuitSizes)
    var maxSizes = circuitSizes.sortedDescending().take(3)
    println(maxSizes.reduce { acc, i -> acc * i })
    println("-----")
}

