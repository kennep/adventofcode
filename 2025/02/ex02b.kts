// Read only one line from example.txt, split on commas and convert into integer ranges A-B
var input = java.io.File("input.txt").readText().split("\n").take(1).first().trim();

var rangeRegex = "(\\d+)-(\\d+)".toRegex();
var ranges = input
    .split(",")
    .mapNotNull({i -> rangeRegex.matchEntire(i.trim())?.groupValues})
    .map({gv -> gv[1].toLong()..gv[2].toLong()})
println(ranges)

var invalidRangeSum = 0L

ranges.forEach({
    r ->
    println("Range from ${r.first} to ${r.last}")
    for(i in r)
    {
        var iStr = i.toString();
        for(div in 2 .. iStr.length)
        {
            if(iStr.length % div != 0) continue;
            var chunks = iStr.chunked(iStr.length / div);
            if(chunks.all { it == chunks[0] }) {
                invalidRangeSum += i;
                println("Found an invalid product ID: $i")
                break;
            }
        }
    }
})

println("Sum of all invalid product IDs: $invalidRangeSum")