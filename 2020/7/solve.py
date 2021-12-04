import sys
import re

# posh brown bags contain 5 dim coral bags, 1 plaid blue bag, 2 faded bronze bags, 2 light black bags.
# dim violet bags contain 1 pale violet bag, 1 bright gold bag.
# mirrored crimson bags contain no other bags.

def parse_list_item(item):
    match = re.match(r"([0-9]+) ([a-z]+ [a-z]+) bags?", item)
    return int(match.group(1)), match.group(2)

def parse(line):
    match = re.match(r"([a-z]+ [a-z]+) bags contain (.*)\.", line)
    bag_color = match.group(1)
    bag_list = match.group(2)
    if bag_list == "no other bags":
        return (bag_color, [])
    else:
        bag_list = bag_list.split(", ")
        bag_list = [parse_list_item(i) for i in bag_list]
        return (bag_color, bag_list)

def found_bag(bags, bag_list, find_item):
    for (count, color) in bag_list:
        if find_item == color:
            return True
        if found_bag(bags, bags[color], find_item):
            return True
    return False

def count_bags(bags, bag_color, bag_list):
    sum = 0
    for (count, color) in bag_list:
        print(f"{bag_color} bag contains {count} {color} bags")
        sum += count
    for (count, color) in bag_list:
        sub_count = count_bags(bags, color, bags[color])
        sum += count * sub_count
    print(f"{bag_color}: Total is {sum}")
    return sum


bags = {}
for line in sys.stdin.readlines():
    bag_color, bag_list = parse(line)
    bags[bag_color] = bag_list

found = 0
for bag_color, bag_list in bags.items():
    if found_bag(bags, bag_list, "shiny gold"):
        found += 1

print(f"There are {found} top-level bag colors that match.")
print(f"The shiny gold bag must contain {count_bags(bags, 'shiny gold', bags['shiny gold'])} other bags.")