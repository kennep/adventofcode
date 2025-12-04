// Read contents of input.txt into input
var input = java.io.File("input.txt").readText().trim()

var itemRegex = "([RL])(\\d+)".toRegex();

data class Rotation(val direction: String, val distance: Int)
var rotations = input
    .split("\n")
    .mapNotNull({i -> itemRegex.matchEntire(i.trim())?.groupValues})
    .map({gv -> Rotation(gv[1], gv[2].toInt())})
println(rotations)

var dial = 50
var numberOfTimesZero = 0

rotations.forEach({
    r ->
    dial += r.distance * (if (r.direction == "L") -1 else 1)
    while(dial < 0)
    {
        dial += 100;
    }
    dial %= 100
    println("Dial pos: $dial")
    if(dial == 0)
    {
        numberOfTimesZero += 1;
    }
})

println("Number of times dial was zero: $numberOfTimesZero")