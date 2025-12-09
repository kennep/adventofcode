data class Point(val x: Long, val y: Long)
var input = java.io.File("input.txt").readText().trim()
    .split("\n").map{ it.trim().split(",") }
    .map{ Point(it[0].toLong(), it[1].toLong()) }

data class Rectangle(val topLeft: Point, val bottomRight: Point)
data class Line(val start: Point, val end: Point)

fun area(r: Rectangle): Long {
    return kotlin.math.abs(r.bottomRight.x - r.topLeft.x + 1) *
            kotlin.math.abs(r.bottomRight.y - r.topLeft.y + 1)
}

fun toRectangle(p1: Point, p2: Point): Rectangle {
    val topLeftX = kotlin.math.min(p1.x, p2.x)
    val topLeftY = kotlin.math.min(p1.y, p2.y)
    val bottomRightX = kotlin.math.max(p1.x, p2.x)
    val bottomRightY = kotlin.math.max(p1.y, p2.y)
    return Rectangle(Point(topLeftX, topLeftY), Point(bottomRightX, bottomRightY))
}

fun isHorizontal(l: Line): Boolean {
    return l.start.y == l.end.y
}

fun isVertical(l: Line): Boolean {
    return l.start.x == l.end.x
}

fun intersect(a: Line, b: Line): Boolean {
    if(a.start == b.start || a.start == b.end || a.end == b.start || a.end == b.end) {
        return false // Lines that touch at endpoints are not considered intersecting
    }

    if(isHorizontal(a))
    {
        if(isHorizontal(b)) return false // Overlapping lines are not considered to be intersecting
        else {
            // a is horizontal, b is vertical
            return (b.start.x in kotlin.math.min(a.start.x, a.end.x) + 1..kotlin.math.max(a.start.x, a.end.x) - 1) &&
                    (a.start.y in kotlin.math.min(b.start.y, b.end.y) + 1..kotlin.math.max(b.start.y, b.end.y) - 1)
        }
    } else if(isVertical(a))
    {
        if(isVertical(b)) return false // Overlapping lines are not considered to be intersecting
        else {
            // a is vertical, b is horizontal
            return (a.start.x in kotlin.math.min(b.start.x, b.end.x) + 1 ..kotlin.math.max(b.start.x, b.end.x) - 1) &&
                    (b.start.y in kotlin.math.min(a.start.y, a.end.y) + 1 ..kotlin.math.max(a.start.y, a.end.y) - 1)
        }
    } else {
        throw IllegalArgumentException("Only horizontal and vertical lines are supported")
    }
}

fun edges(r: Rectangle) : List<Line> {
    val topLeft = r.topLeft
    val bottomRight = r.bottomRight
    val topRight = Point(bottomRight.x, topLeft.y)
    val bottomLeft = Point(topLeft.x, bottomRight.y)

    return listOf(
        Line(topLeft, topRight),
        Line(topRight, bottomRight),
        Line(bottomRight, bottomLeft),
        Line(bottomLeft, topLeft)
    )
}

fun intersect(r: Rectangle, l: Line) : Boolean {
    for (edge in edges(r)) {
        if (intersect(edge, l)) {
            return true
        }
    }
    return false
}

fun inside(r: Rectangle, l: Line) : Boolean {
    return (l.start.x > r.topLeft.x && l.start.x < r.bottomRight.x &&
            l.start.y > r.topLeft.y && l.start.y < r.bottomRight.y) &&
            (l.end.x > r.topLeft.x && l.end.x < r.bottomRight.x &&
                    l.end.y > r.topLeft.y && l.end.y < r.bottomRight.y)
}


var rectangles = input.flatMap { p1 ->
    input.map { p2 -> toRectangle(p1, p2) }
}

var polygon = input.zipWithNext { p1, p2 -> Line(p1, p2) } + Line(input.last(), input.first())

var validRectangles = rectangles
    .filter { r -> !polygon.any { l -> intersect(r, l) || inside(r, l) } }
    .sortedByDescending { area(it)}

var biggestValidRectangle = validRectangles.maxBy { area(it) }
println("Biggest valid rectangle found: $biggestValidRectangle with area ${area(biggestValidRectangle)}")
