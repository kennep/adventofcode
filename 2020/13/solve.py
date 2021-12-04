import sys
from itertools import count
from math import ceil

departure = int(sys.stdin.readline())
buslines = sys.stdin.readline().split(",")

buslines = [int(x) for x in buslines if x != 'x']

print(departure, buslines)

def bus_departures(lineid):
    return (c*lineid for c in count(0))

closest_departures = [
    (lineid, ceil(departure/lineid) * lineid)
    for lineid in buslines
]

closest_departures.sort(key=lambda l: l[1])

(lineid, closest_departure) = closest_departures[0]

print(f"Line {lineid} departs at {closest_departure}")
waiting_time = closest_departure - departure
print(f"Waiting time: {waiting_time} mins, Product: {lineid*waiting_time}")