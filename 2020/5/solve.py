import sys

boarding_passes = [l.strip() for l in sys.stdin.readlines()]

def get_seat_row(boarding_pass):
    return int(boarding_pass[0:7].replace("B", "1").replace("F", "0"), 2)

def get_seat_col(boarding_pass):
    return int(boarding_pass[7:].replace("R", "1").replace("L", "0"), 2)

def get_seat_id(boarding_pass):
    return get_seat_row(boarding_pass) * 8 + get_seat_col(boarding_pass)


print(max(get_seat_id(boarding_pass) for boarding_pass in boarding_passes))


seat_ids = set(get_seat_id(boarding_pass) for boarding_pass in boarding_passes)
all_seat_ids = set(range(0,128*8+8))

print(all_seat_ids - seat_ids)