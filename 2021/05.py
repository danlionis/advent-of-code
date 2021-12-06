import sys
from itertools import repeat
from collections import defaultdict


def sign(a): return 1 if a >= 0 else -1

def get_points(xy):
    x1, y1 = xy[0]
    x2, y2 = xy[1]

    res = []
    x_min = min(x1,  x2)
    x_max = max(x1,  x2)
    y_min = min(y1,  y2)
    y_max = max(y1,  y2)

    if x1 == x2:
        res.extend((x1, y) for y in range(y_min, y_max + 1))
    elif y1 == y2:
        res.extend((x, y1) for x in range(x_min, x_max + 1))
    else:
        if x2 > x1:
            xs = range(x1, x2 + 1)
        else:
            xs = range(x1, x2 - 1, -1)

        if y2 > y1:
            ys = range(y1, y2 + 1)
        else:
            ys = range(y1, y2 - 1, -1)

        # print(xy, list(xs), list(ys))

        res.extend(zip(xs, ys))

    return res

def parse_entry(l):
    xy1, xy2 = l.strip().split(" -> ")
    xy1 = tuple(map(int, xy1.split(",")))
    xy2 = tuple(map(int, xy2.split(",")))
    return (xy1, xy2)

lines = list(map(parse_entry, sys.stdin))

def horizontal(xy): return xy[0][0] == xy[1][0] 
def vertical(xy): return xy[0][1] == xy[1][1] 
def diagonal(xy): return abs(xy[0][0] - xy[1][0]) == abs(xy[0][1] - xy[1][1])

def part1():
    hv = list(filter(lambda xy: horizontal(xy) or vertical(xy), lines))

    coords = defaultdict(lambda: 0)

    for l in hv:
        for p in get_points(l):
            coords[p] += 1

    print("part1:", sum( 1 for _ in filter(lambda x: x >= 2, coords.values())))


def part2():
    ls = list(filter(lambda xy: horizontal(xy) or vertical(xy) or diagonal(xy), lines))

    coords = defaultdict(lambda: 0)

    for l in ls:
        points = get_points(l)
        # print(l, points)
        for p in points:
            coords[p] += 1

    print("part2:", sum( 1 for _ in filter(lambda x: x >= 2, coords.values())))

part1()
part2()
