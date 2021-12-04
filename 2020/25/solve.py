import sys

door_pk = int(sys.stdin.readline())
card_pk = int(sys.stdin.readline())

def transform_sn(sn, loop_size):
    value = 1
    for i in range(0, loop_size):
        value = (value * sn) % 20201227
    return value

def find_loop_size(pk):
    loop_size = 1
    value = 1
    while True:
        value = (value * 7) % 20201227
        if value == pk:
            return loop_size
        loop_size += 1

door_loop_size = find_loop_size(door_pk)
card_loop_size = find_loop_size(card_pk)
print(f"Door loop size: {door_loop_size}")
print(f"Card loop size: {card_loop_size}")

door_enc = transform_sn(door_pk, card_loop_size)
card_enc = transform_sn(card_pk, door_loop_size)

print(f"Encryption key (door): {door_enc}")
print(f"Encryption key (card): {card_enc}")

