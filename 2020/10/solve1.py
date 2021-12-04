import sys

ratings = [int(l) for l in sys.stdin.readlines()]

ratings = sorted(ratings + [max(ratings)+3])

diffs = {}

last_rating = 0
for rating in ratings:
    diff = rating - last_rating
    last_rating = rating
    diffs[diff] = diffs.get(diff, 0) + 1

for diff, count in sorted(diffs.items(), key=lambda i: i[0]):
    print(f"Count of differences of {diff} jolts: {count}")

print(f"1x3 = {diffs.get(1, 0) * diffs.get(3,0)}")
