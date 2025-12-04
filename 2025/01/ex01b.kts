// Read contents of input.txt into input
var input = java.io.File("input.txt").readText().trim()

var itemRegex = "([RL])(\\d+)".toRegex();

data class Rotation(val direction: String, val distance: Int)
var rotations = input
    .split("\n")
    .mapNotNull({i -> itemRegex.matchEntire(i.trim())?.groupValues})
    .map({gv -> Rotation(gv[1], gv[2].toInt())})

var dial = 50
var numberOfTimesZero = 0

rotations.forEach({
    r ->
    var distance = r.distance;
    var direction = if (r.direction == "L") -1 else 1;

    while(distance > 0)
    {
        if(direction == -1 && dial == 0)
        {
            dial = 100;
        }
        var step = distance.coerceAtMost(if (direction == -1) dial else (100 - dial))
        if(step == 0)
        {
            throw Exception("Step out of bounds: $step")
        }
        dial += step * direction
        distance -= step
        if(dial == 100)
        {
            dial = 0;
        }
        if(dial == 0)
        {
            numberOfTimesZero += 1;
        }
        if(dial !in 0..100)
        {
            throw Exception("Dial out of bounds: $dial")
        }
        println("Rotation: $r Dial pos: $dial Number of times zero: $numberOfTimesZero")
    }
})

println("Number of times dial was zero: $numberOfTimesZero")