import sys

fields = {}

# Read fields
while True:
    line = sys.stdin.readline()
    if line.strip() == "": break
    field, rest = line.split(":", 1)
    rangedescriptions = rest.split(" or ")
    ranges = []
    for rangedescription in rangedescriptions:
        lower, upper = rangedescription.split("-", 1)
        lower = int(lower)
        upper = int(upper)
        ranges.append((lower, upper))
    fields[field] = ranges

def read_tickets():
    tickets = []
    while True:
        line = sys.stdin.readline()
        if line.strip() == "": break
        values = [int(v) for v in line.split(",")]
        tickets.append(values)
    return tickets

line = sys.stdin.readline()
assert(line.startswith("your ticket:"))
your_ticket = read_tickets()[0]

assert(sys.stdin.readline().startswith("nearby tickets:"))
nearby_tickets = read_tickets()

print(fields)
print(your_ticket)
print(nearby_tickets)

invalid_values = 0
for ticket in nearby_tickets:
    for value in ticket:
        valid = False
        for field, ranges in fields.items():
            for (lower, upper) in ranges:
                if value >= lower and value <= upper:
                    valid = True
        if not valid:
            invalid_values += value

print(f"Ticket scanning error rate: {invalid_values}")
