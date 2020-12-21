import sys
import itertools
import functools
import copy

lines = list(map(list, map(str.strip, sys.stdin)))

row_len = len(lines[0])

lines = [list("." * row_len)] + lines + [list("." * row_len)]
lines = list(map(lambda l: ["."] + l + ["."], lines))


def get_adjacent(row, col, seats):
    return [seats[y][x] for x in range(col - 1, col + 2) for y in range(row - 1, row + 2) if x != col or y != row]


def get_los(row, col, seats):

    horizonal = [seats[row][x] for x in range(len(seats[row]))]
    vertical = [seats[y][col] for y in range(len(seats))]
    rc_min = min(row, col)

    # print(horizonal)
    # print(vertical)

    mid_left = horizonal[:col]
    mid_right = horizonal[col + 1:]

def change(state, adj, treshold=4):
    if state == ".":
        return "."

    if state == "L":
        if adj.count("#") == 0:
            return "#"
        else:
            return "L"

    if state == "#":
        if adj.count("#") >= treshold:
            return "L"
        else:
            return "#"


def print_seats(seats):
    for y in range(len(lines)):
        # print("".join(lines[y]))
        pass


print_seats(lines)

print(get_los(2, 2, lines))

tmp = copy.deepcopy(lines)
last = 0
while True:
    for y in range(1, len(lines) - 1):
        for x in range(1, len(lines[y]) - 1):
            adj = get_adjacent(y, x, lines)
            tmp[y][x] = change(lines[y][x], adj)

    res = sum(map(lambda s: s.count("#"), tmp))
    if res == last:
        break
    lines = copy.deepcopy(tmp)
    last = res
    break

print("part1:", res)
