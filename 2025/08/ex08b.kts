import kotlin.math.pow
import kotlin.math.sqrt

data class Point(val x: Int, val y: Int, val z: Int)

var input = java.io.File("input.txt").readText().trim()
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

var closestPoints = input.flatMap({ p1 ->
    input.filter({ p2 -> p1 != p2 })
        .map({ p2 -> PointDistance(p1, p2, straightLineDistance(p1, p2)) })
})
    .map{ pd -> if(pd.p1.x < pd.p2.x ||
        (pd.p1.x == pd.p2.x && pd.p1.y < pd.p2.y) ||
        (pd.p1.x == pd.p2.x && pd.p1.y == pd.p2.y && pd.p1.z < pd.p2.z))
        pd else PointDistance(pd.p2, pd.p1, pd.distance)
    }
    .distinct()
    .sortedBy { it.distance }
println(closestPoints.joinToString("\n"))

var circuits = mutableListOf<List<Point>>()

var iterator = closestPoints.iterator()
var numConnections = 0
var lastConnected : PointDistance? = null
while(iterator.hasNext()) {
    var connection = iterator.next()

    var firstConnected = circuits.firstOrNull { c -> c.contains(connection.p1) }
    var secondConnected = circuits.firstOrNull { c -> c.contains(connection.p2) }
    if(firstConnected != null && secondConnected != null)
    {
        if(firstConnected == secondConnected)
        {
            // Both points already connected
            numConnections++
            continue
        }
        // Merge circuits
        circuits.remove(firstConnected)
        circuits.remove(secondConnected)
        val merged = firstConnected + secondConnected
        circuits.add(merged)
        lastConnected = connection
        numConnections++
        continue;
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
    lastConnected = connection

}
println(numConnections)
println(circuits.joinToString("\n"))
var circuitSizes = circuits.map { it.size }
println(circuitSizes)
var maxSizes = circuitSizes.sortedDescending().take(3)
println(maxSizes.reduce { acc, i -> acc * i })
println("-----")

println("Last connected points: $lastConnected")
println("X coords multiplied: ${lastConnected?.let { it.p1.x.toLong() * it.p2.x.toLong() }}")