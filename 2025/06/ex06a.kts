var input = java.io.File("input.txt").readText().trim()
    .split("\n").map{ it.trim().split(Regex(" +")) }

var transposed = input[0].indices.map { colIndex ->
    input.map { row -> row[colIndex] }
}

data class Assignment(val operation: Char, val values: List<Long>)

var assigments = transposed.map {
    Assignment(it.last()[0], it.dropLast(1).map { v -> v.toLong() })
}

fun applyAssignment(assignment: Assignment): Long {
    return when(assignment.operation) {
        '+' -> assignment.values.sum()
        '*' -> assignment.values.reduce { acc, v -> acc * v }
        else -> 0L
    }
}

var grandTotal = assigments.fold(0L) { acc, a -> acc + applyAssignment(a) }
println("Grand total of all assignments: $grandTotal")