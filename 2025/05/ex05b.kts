val input = java.io.File("input.txt").readText().trim().split("\n\n")

val freshRanges = input[0].split("\n")
    .map {
        var parts = it.trim().split("-").map { 
            ingredient -> 
            ingredient.toLong()
        }
        parts[0]..parts[1]
    }

var consolidatedRanges = mutableListOf<LongRange>()

for (range in freshRanges)
{
    // Remove any ranges that are fully contained within the new range
    consolidatedRanges = consolidatedRanges.filter { existingRange ->
        !(range.first <= existingRange.first && range.last >= existingRange.last) }.toMutableList()
    
    var firstOverlapIndex: Int? = null
    var lastOverlapIndex: Int? = null
    for(i in consolidatedRanges.indices) {
        if (range.first in consolidatedRanges[i]) {
            firstOverlapIndex = i
        }
        if (range.last in consolidatedRanges[i]) {
            lastOverlapIndex = i;
        }
    }
    
    if (firstOverlapIndex == null && lastOverlapIndex == null) {
        // No overlaps, add new range
        consolidatedRanges.add(range)
    }
    
    if (firstOverlapIndex != null && lastOverlapIndex == null) {
        // Range begins inside an existing range
        val existingRange = consolidatedRanges[firstOverlapIndex]
        consolidatedRanges[firstOverlapIndex] = existingRange.first..range.last
    }
    
    if (firstOverlapIndex == null && lastOverlapIndex != null) {
        // Range ends inside an existing range
        val existingRange = consolidatedRanges[lastOverlapIndex]
        consolidatedRanges[lastOverlapIndex] = range.first..existingRange.last
    }
 
    if (firstOverlapIndex != null && lastOverlapIndex != null && firstOverlapIndex != lastOverlapIndex) {
        // Range spans two existing ranges
        val firstRange = consolidatedRanges[firstOverlapIndex]
        val lastRange = consolidatedRanges[lastOverlapIndex]
        consolidatedRanges.removeAt(firstOverlapIndex)
        consolidatedRanges.removeAt(if(lastOverlapIndex > firstOverlapIndex) lastOverlapIndex - 1 else lastOverlapIndex) // Adjust index after removal
        consolidatedRanges.add(firstRange.first..lastRange.last)
    }
    
    // Range is fully inside existing ranges, do nothing
}

// Count the ranges
var totalCount = consolidatedRanges.sumOf { range -> range.last - range.first + 1 }

println(consolidatedRanges)
println("Total valid ingredient count: $totalCount")
