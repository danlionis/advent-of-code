import sys
from functools import reduce

lines = map(str.strip, sys.stdin)


def bit(c): return {"F": 0, "L": 0, "B": 1, "R": 1}[c]
# def bit(c): return ord(c) % 7 % 2
# def bit(c): return ord(c) >> 2 & 1 ^ 1


def seat_id(seat): return reduce(lambda acc, b: acc << 1 | b, map(bit, seat))


seats = sorted(map(seat_id, lines))
print("part1: " + str(max(seats)))

min_seat = min(seats)
max_seat = max(seats)

print("part2:", [x for x in range(min_seat, max_seat) if x not in seats][0])

