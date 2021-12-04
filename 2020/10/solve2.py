import sys
from functools import lru_cache
from itertools import combinations 

ratings = [int(l) for l in sys.stdin.readlines()]

ratings = tuple([0] + sorted(ratings) + [max(ratings) + 3])

@lru_cache
def valid_combinations_(input_ratings):
    print(input_ratings)
    count = 1 # The input is a valid sequence:
    for i in range(len(input_ratings) - 2, 0, -1):
        if input_ratings[i+1] - input_ratings[i-1] <= 3:
            count += valid_combinations(input_ratings[0:i] + input_ratings[i+1:])
    return count

def number_of_runs(count):
    if count == 1: # 0
        return 1
    if count == 2: # 0,1
        return 1
    if count == 3: # 0, 1, 2: 0, 2
        return 2
    if count == 4: # 0, 1, 2, 3: 0, 1, 3: 0, 2, 3: 0, 3
        return 4
    if count == 5:
        """
        0,1,2,3,4
        0,1,2,4
        0,1,3,4
        0,2,3,4
        0,1,4
        0,2,4
        0,3,4
        """
        return 7
    if count == 6:
        """
        0,1,2,3,4,5
        0,1,2,3,5
        0,1,2,4,5
        0,2,3,4,5
        0,1,2,5
        0,2,3,5
        0,2,4,5
        0,3,4,5
        0,2,5
        0,3,5
        """
        return 10 # Don't know if this is correct
    if count == 7:
        """
        0,1,2,3,4,5,6
        0,1,2,3,4,6
        0,1,2,3,5,6
        0,1,2,4,5,6
        0,1,3,4,5,6
        0,2,3,4,5,6
        0,1,2,3,6
        0,1,2,4,6
        0,1,2,5,6
        0,1,3,4,6
        0,1,3,5,6
        0,2,3,4,6
        0,2,3,5,6
        0,2,4,5,6
        0,3,4,5,6
        0,1,4,6
        0,1,3,6
        0,2,3,6
        0,2,4,6
        0,2,5,6
        0,3,4,6
        0,3,5,6
        0,3,6
        """
        return 23 # Don't know if this is correct

    raise ValueError("Don't know what to do for count: %s" % count)


def valid_combinations(ratings):
    print(ratings)
    value = 1
    run_length = 1
    last_rating = -3
    for rating in ratings:
        if rating - last_rating == 1:
            run_length += 1        
        else:
            value *= number_of_runs(run_length)
            run_length = 1
        last_rating = rating
    value *= number_of_runs(run_length)
    return value

print(valid_combinations(ratings))
