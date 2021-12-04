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

print(len(nearby_tickets))

def is_valid(ticket):
    all_valid=True
    for value in ticket:
        valid = False
        for field, ranges in fields.items():
            for (lower, upper) in ranges:
                if value >= lower and value <= upper:
                    valid = True
        if valid is False:
            all_valid=False
    return all_valid    

nearby_tickets = [t for t in nearby_tickets if is_valid(t)]

column_field_mappings = {}

for column in range(0, len(nearby_tickets[0])):    
    for field, ranges in fields.items():
        is_valid_value = True
        for ticket in nearby_tickets:
            value = ticket[column]
            valid_value_for_ticket = False
            for (lower, upper) in ranges:
                if value >= lower and value <= upper:
                    valid_value_for_ticket = True
                    break
            if not valid_value_for_ticket:
                is_valid_value = False
            #print(f"Column {column} is valid for {field}? {valid_value_for_ticket} {ranges} {ticket}")
        if is_valid_value:
            print(f"Column {column} is valid for {field}")
            column_field_mappings.setdefault(field, set()).add(column)

for field, columns in column_field_mappings.items():
    print(f"Field {field}: {columns}")

final_mapping = {}

while column_field_mappings:
    found_mapping = None
    for field, columns in column_field_mappings.items():
        if len(columns) == 1:
            final_mapping[field] = list(columns)[0]
            found_mapping = list(columns)[0]
            del column_field_mappings[field]
            break
    if found_mapping is not None:
        for field, columns in column_field_mappings.items():
            if found_mapping in columns:
                columns.remove(found_mapping)
    else:
        raise ValueError(f"Found no mapping in iteration! {column_field_mappings}")

print(final_mapping)

product = 1
for field, column in final_mapping.items():
    if field.startswith("departure"):
        product *= your_ticket[column]

print(f"Product: {product}")