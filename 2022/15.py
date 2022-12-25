import sys

data = sys.stdin.read().strip().splitlines()

sensors = {}
beacons = set()


def manhattan(x1, y1, x2, y2):
    return abs(x1 - x2) + abs(y1 - y2)


for line in data:
    d = line.split()
    sx = int(d[2][2:].strip(","))
    sy = int(d[3][2:].strip(":"))

    bx = int(d[8][2:].strip(","))
    by = int(d[9][2:].strip(":"))

    sensors[(sx, sy)] = manhattan(sx, sy, bx, by)
    beacons.add((bx, by))

all_pos = list(sensors.keys()) + list(beacons)

min_x = min([s[0] for s in all_pos])
max_x = max([s[0] for s in all_pos])

row = 10
row = 2000000

part1 = 0

for col in range(min_x - 1000000, max_x + 1000000):
    for (sx, sy), dist in sensors.items():
        if manhattan(col, row, sx, sy) <= dist and (col, row) not in beacons and (col, row) not in sensors:
            # print(col, max_x, "#")
            part1 += 1
            break

print("part1:", part1)


def possible_points(scanner_col, scanner_row, distance):
    res = []
    distance += 1
    # print(distance)

    for i in range(distance):
        res.append((scanner_col + i, scanner_row +
                   (distance - i)))  # top right

        res.append((scanner_col + (distance - i),
                   scanner_row - i))  # bottom right

        res.append((scanner_col - i, scanner_row -
                   (distance - i)))  # bottom left

        res.append((scanner_col - (distance - i), scanner_row + i))  # top left

    return res


part2 = None
for x, y in sensors:
    # print(x, y, sensors[(x, y)], end="\r")

    if part2 != None:
        break

    for px, py in possible_points(x, y, sensors[(x, y)]):
        if px < 0 or py < 0 or px > 4000000 or py > 4000000:
            continue
        if all([manhattan(px, py, sx, sy) >= dist for (sx, sy), dist in sensors.items()]):
            part2 = (px, py)
            break

print("part2:", part2[0] * 4000000 + part2[1])
