import kotlin.math.pow

var input = java.io.File("input.txt").readText().trim()

var lines = input.split("\n").map { it.trim() }
var batteryBanks = lines.map({ line ->
    line.map { it.code - '0'.code }
})
println(batteryBanks)
var sumJolts = 0L
val sequenceLen = 12
for(bank in batteryBanks) {
    val result = maxJolts( bank, sequenceLen)
    println("Max jolts for bank $bank: $result")
    sumJolts += joltsValue(result)
}
println("Sum of max jolts: $sumJolts")

fun joltsValue(joltsList: List<Int>) : Long {
    return joltsList.joinToString("", transform = { it.toString() }).toLong()
    /*
    var jolts = 0L
    for(i in joltsList.indices) {
        jolts += joltsList[i] * 10.0.pow((joltsList.size - i - 1).toDouble()).toLong()
    }
    return jolts*/
}

fun isBetter(a: List<Int>, b: List<Int>) : Boolean {

    assert(a.size == b.size)
    return joltsValue(a) > joltsValue(b)
    /*
    for(i in a.indices) {
        if(i >= b.size) return true
        if(a[i] > b[i]) return true
        if(a[i] < b[i]) return false
    }
    return false*/
}

fun maxJolts(batteryBank: List<Int>, sequenceLen: Int) : List<Int> {
    //
    assert(batteryBank.size >= sequenceLen)
    if(batteryBank.size == sequenceLen) return batteryBank

    // First try to find the best of a shorter input
    var best = maxJolts(batteryBank.drop(1), sequenceLen)

    // Then try to see if adding the first digit and removing
    // one of the other digits help
    if(best[0] > batteryBank[0])
    {
        // The first digit is lower than the first digit of best,
        // so no need to try adding it
        return best
    }
    var improved = best
    for(i in 0 until best.size)
    {
        val candidate = listOf(batteryBank[0]) +
                best.take(i) + best.drop(i + 1)
        assert(candidate.size == sequenceLen)
        if(isBetter(candidate, improved))
        {
            //println("choosing candidate: ${joltsValue(candidate)} over ${joltsValue(candidate)}")
            improved = candidate
        }
    }
    return improved
}
