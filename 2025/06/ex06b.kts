var input = java.io.File("input.txt").readText().trim()
    .replace("\r", "")
    .split("\n")

var longestLineCount =  input.maxOf { it.length }

var transposed = (0 until longestLineCount).map { colIndex ->
    input.map { row -> if (row.length > colIndex) row[colIndex] else ' ' }
}.map { it.joinToString("") }

data class Assignment(val operation: Char, val values: List<Long>)

var assignments = mutableListOf<Assignment>()
var values = mutableListOf<Long>()
for(element in transposed.reversed()) {
    if(element.all{ it == ' '}) continue;

    if(element.last() == '*' || element.last() == '+')
    {
        val operation = element.last()
        values.add(element.dropLast(1).trim().toLong())
        assignments.add(Assignment(operation, values))
        values = mutableListOf<Long>()
    }
    else {
        values.add(element.trim().toLong())
    }
}


fun applyAssignment(assignment: Assignment): Long {
    return when(assignment.operation) {
        '+' -> assignment.values.sum()
        '*' -> assignment.values.reduce { acc, v -> acc * v }
        else -> 0L
    }
}

var grandTotal = assignments
    .fold(0L) { acc, a -> acc + applyAssignment(a) }
println("Grand total of all assignments: $grandTotal")
