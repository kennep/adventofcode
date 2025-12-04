import kotlin.collections.plusAssign
import kotlin.math.max
import kotlin.math.pow

var input = java.io.File("input.txt").readText().trim()

var lines = input.split("\n").map { it.trim() }
var batteryBanks = lines.map({ line ->
    line.map { it.code - '0'.code }
})

println(batteryBanks)
var sumJolts = 0
for(bank in batteryBanks) {
    val result = maxJolts(bank)
    println("Max jolts for bank $bank: $result")
    sumJolts += result
}
println("Sum of max jolts: $sumJolts")

fun maxJolts(batteryBank: List<Int>) : Int {
    if(batteryBank.isEmpty()) return 0
    if(batteryBank.size == 1) return 0
    var maxJolts = 0
    var first = batteryBank[0]
    for(i in 1 until batteryBank.size) {
        val second = batteryBank[i]
        val jolts = first * 10 + second
        if(jolts > maxJolts) {
            maxJolts = jolts
        }
    }
    return max(maxJolts, maxJolts(batteryBank.drop(1)))
}
