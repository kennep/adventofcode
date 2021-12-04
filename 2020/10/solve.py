import sys
from functools import lru_cache

ratings = [int(l) for l in sys.stdin.readlines()]

ratings = tuple(sorted(ratings + [max(ratings)+3]))

@lru_cache
def combinations(input_ratings):
    src = input_ratings[0]
    dest = input_ratings[-1]
    if dest - src <= 3:
        if len(input_ratings) > 2:
            return 1 + combinations(tuple(input_ratings[1:-1]))
        else:
            return 1
    else:
        return 0

print(combinations(ratings))
