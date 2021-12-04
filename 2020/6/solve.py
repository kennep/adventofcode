import sys

groups = []
current_group = set()
for line in sys.stdin.readlines():
    line = line.strip()
    if not line:
        groups.append(current_group)
        current_group = set()
    for answer in line:
        current_group.add(answer)

if current_group:
    groups.append(current_group)

for g in groups:
    print(len(g))

print(sum(len(g) for g in groups))