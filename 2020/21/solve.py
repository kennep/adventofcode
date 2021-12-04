import sys
import re

class Food:
    def __init__(self, ingredients):
        allergenes = ""
        if "(contains " in ingredients:
            ingredients, allergenes = ingredients.split("(contains", 2)
        
        ingredients = ingredients.strip().split(" ")
        allergenes = allergenes.strip().rstrip(")").split(", ")
        allergenes = [a for a in allergenes if a]
        self.ingredients = ingredients
        self.allergenes = allergenes

    def print(self):
        print("Ingredients:")
        for i in self.ingredients:
            print(f" - {i}")
        if self.allergenes:
            print("Allergnenes:")
            for a in self.allergenes:
                print(f" - {a}")
        print()

allergenes_map = {}

all_ingredients = set()

foods = [Food(l) for l in sys.stdin.readlines()]
for f in foods:
    f.print()
    all_ingredients.update(f.ingredients)
    for a in f.allergenes:
        if a not in allergenes_map:
            allergenes_map[a] = set(f.ingredients)
        else:
            ingredients_to_remove = [i for i in allergenes_map[a] if i not in f.ingredients]
            for i in ingredients_to_remove:
                allergenes_map[a].remove(i)


#while True:
#for allergene, possible_ingredients in allergenes_map.items():
##    if len(possible_ingredients) == 1:
 #       for allergene, possible_ingredients in allergenes_map.items():
            

ingredients_with_allergenes = set()
for allergene, possible_ingredients in allergenes_map.items():
    print(f"{allergene}: {', '.join(possible_ingredients)}")
    ingredients_with_allergenes.update(possible_ingredients)

ingredients_without_allergenes = all_ingredients - ingredients_with_allergenes
print(f"Ingredients with allergenes: ({len(ingredients_with_allergenes)}): {', '.join(ingredients_with_allergenes)}")
print(f"Ingredients that cannot contain allergenes ({len(ingredients_without_allergenes)}): {', '.join(ingredients_without_allergenes)}")

num_mentions = 0
for f in foods:
    for i in f.ingredients:
        if i in ingredients_without_allergenes:
            num_mentions += 1

print(f"Numer of mentions: {num_mentions}")

ingredient_map = {}

while allergenes_map:
    known_ingredients = [(allergen, next(iter(ingredients))) for (allergen, ingredients) in allergenes_map.items() if len(ingredients) == 1]    
    for (allergen, ingredient) in known_ingredients:
        ingredient_map[allergen] = ingredient
        del allergenes_map[allergen]
        for a, i in allergenes_map.items():
            if ingredient in i:
                i.remove(ingredient)

for allergene, ingredient in ingredient_map.items():
    print(f"{allergene}: {ingredient}")

canonical_list = sorted(ingredient_map.items(), key=lambda it: it[0])
print("Sorted:", canonical_list)
print("Canonical dangerous ingredients list:", (",".join(i for (a, i) in canonical_list)))