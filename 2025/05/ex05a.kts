val input = java.io.File("input.txt").readText().trim().split("\n\n")

val freshRanges = input[0].split("\n")
    .map {
        var parts = it.trim().split("-").map { 
            ingredient -> 
            ingredient.toLong()
        }
        parts[0]..parts[1]
    }

val ingredientList = input[1].split("\n").map { it.trim().toLong() }

val validIngredients = ingredientList.filter { ingredient ->
    freshRanges.any { range -> ingredient in range }
}

println("Valid ingredient count: ${validIngredients.size}")