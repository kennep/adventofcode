// A regex for parsing this:
var lineRegex = "\\[([#\\.]+)\\](.*)\\{([0-9,]+)\\}".toRegex();
var input = java.io.File("input.txt").readText().trim()

var parsedLines = input
    .split("\n")
    .mapNotNull({i -> lineRegex.matchEntire(i.trim())?.groupValues})
    .map({gv -> Triple(gv[1],
        gv[2].trim().split(") (").map{ it.trim('(', ')') }.filter{ it.isNotEmpty() }
            .map{ it.split(",").map{ it.toInt() }}, gv[3].split(",").map{ it.toUShort()
            })})

var totalPresses = 0L;
for((targetConfig, buttons, targetJolts) in parsedLines) {
    println("Buttons: $buttons, Target Jolts: $targetJolts")

    var numPresses = 0L;
    var state = mutableSetOf(List(targetJolts.size) { 0U.toUShort() } )
    for(buttonIdx in targetJolts.indices.sortedByDescending { buttons.count { b -> b.contains(it)}}) {
        println("Jolt value index: $buttonIdx")
        while (!state.any{ it[buttonIdx] == targetJolts[buttonIdx] }) {
            for(s in state.toList()) {
                state.remove(s)
                for(b in buttons.filter{ it.contains(buttonIdx)}) {
                    var n = s.toMutableList()
                    var valid = true
                    for(i in b)
                    {
                        n[i]++
                        if(n[i] > targetJolts[i]) {
                            valid = false
                            break
                        }
                    }
                    if(valid) state.add(n)
                }
            }
            numPresses += 1
            println("Num presses: $numPresses, State size: ${state.size}")
            if(numPresses > 1000) {
                println("Too many presses, giving up")
                break
            }
        }
    }

    println("Number of presses to reach target: $numPresses")
    totalPresses += numPresses
}

println("Total presses for all lines: $totalPresses")
