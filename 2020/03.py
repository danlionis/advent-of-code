import sys

lines = list(map(str.strip, sys.stdin))


def trees_for_slope(lines, slope_x, slope_y):
    linelen = len(lines[0])
    trees = 0
    x, y = 0, 0
    while y < len(lines) - 1:
        y += slope_y
        x += slope_x
        x = x % linelen
        line = lines[y]

        if line[x] == "#":
            trees += 1

    return trees


print("part1: " + str(trees_for_slope(lines, 3, 1)))

res = trees_for_slope(lines, 1, 1) * trees_for_slope(lines, 3, 1) * trees_for_slope(
    lines, 5, 1) * trees_for_slope(lines, 7, 1) * trees_for_slope(lines, 1, 2)

print("part2: " + str(res))

