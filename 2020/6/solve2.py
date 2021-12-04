import sys

groups = []
current_group = None
for line in sys.stdin.readlines():
    line = line.strip()
    if not line:
        groups.append(current_group)
        current_group = None
        continue
    answer_set = set(line)
    if current_group is None:
        current_group = answer_set
    else:
        current_group = current_group.intersection(answer_set)

if current_group:
    groups.append(current_group)

for g in groups:
    print(len(g))

print(sum(len(g) for g in groups))