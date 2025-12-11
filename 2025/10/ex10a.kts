// A regex for parsing this:
var lineRegex = "\\[([#\\.]+)\\](.*)\\{([0-9,]+)\\}".toRegex();
var input = java.io.File("input.txt").readText().trim()

var parsedLines = input
    .split("\n")
    .mapNotNull({i -> lineRegex.matchEntire(i.trim())?.groupValues})
    .map({gv -> Triple(gv[1],
        gv[2].trim().split(") (").map{ it.trim('(', ')') }.filter{ it.isNotEmpty() }
            .map{ it.split(",").map{ it.toInt() }}, gv[3].split(",").map{ it.toInt() })})

    .map { (targetStr, buttons, joltsList) ->
        val targetConfigBits = stringToBits(targetStr)
        val buttonsBits = buttons.map { intListToBits(it) }
        Triple(targetConfigBits, buttonsBits, joltsList)
    }

fun stringToBits(s: String): UShort {
    var result = 0U
    for(c in s.reversed()) {
        result = (result shl 1) or if(c == '#') 1U else 0U
    }
    return result.toUShort()
}

fun intListToBits(ints: List<Int>): UShort {
    var result = 0U
    for(i in ints) {
        result = result or (1U shl i)
    }
    return result.toUShort()
}

var totalPresses = 0L;
for((targetConfig, buttons, joltsList) in parsedLines) {
    println("Target: $targetConfig, Buttons: $buttons")
    var numPresses = 0L;
    var state = listOf(0U.toUShort())
    while (!state.contains(targetConfig)) {
        val newStates = mutableSetOf<UShort>()
        for(s in state) {
            for(b in buttons) {
                val newState = s xor b
                newStates.add(newState)
            }
        }
        state = newStates.toList()
        numPresses += 1
        if(numPresses > 1000) {
            println("Too many presses, giving up")
            break
        }
    }
    println("Number of presses to reach target: $numPresses")
    totalPresses += numPresses
}

println("Total presses for all lines: $totalPresses")
